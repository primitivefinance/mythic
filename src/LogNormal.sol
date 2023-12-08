// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./v3/BisectionLib.sol";

/// @dev Taking the square root of a WAD value returns a value with units of 1E9.
/// Multiplying the result by SQRT_WAD will normalize it back to WAD units.
uint256 constant SQRT_WAD = 1e9;
uint256 constant TWO = 2e18;
uint256 constant HALF = 0.5e18;

/// @dev Parameterization of the Log Normal curve.
struct Parameters {
    uint256 strikePriceWad;
    uint256 sigmaPercentWad;
    uint256 tauYearsWad;
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(
    bytes memory data,
    uint256 reserveXWad
) pure returns (int256) {
    (
        uint256 y,
        uint256 liquidity,
        int256 swapConstant,
        Parameters memory params
    ) = abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({
        reserveXWad: reserveXWad,
        reserveYWad: y,
        totalLiquidity: liquidity,
        params: params
    }) - swapConstant;
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(
    bytes memory data,
    uint256 reserveYWad
) pure returns (int256) {
    (
        uint256 x,
        uint256 liquidity,
        int256 swapConstant,
        Parameters memory params
    ) = abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({
        reserveXWad: x,
        reserveYWad: reserveYWad,
        totalLiquidity: liquidity,
        params: params
    }) - swapConstant;
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 liquidity
) pure returns (int256) {
    (uint256 x, uint256 y, int256 swapConstant, Parameters memory params) =
        abi.decode(data, (uint256, uint256, int256, Parameters));
    // todo: maybe update with swapConstantGrowth with previous swapConstant.
    return tradingFunction({
        reserveXWad: x,
        reserveYWad: y,
        totalLiquidity: liquidity,
        params: params
    });
}

/// @param sigmaPercentWad Must be in WAD units such that 1E18 = 100%.
/// @param tauYearsWad Must be in WAD units such that 1E18 = 1 year.
/// @return sigmaSqrtTau The product of sigma and the square root of tau in WAD units.
function computeSigmaSqrtTau(
    uint256 sigmaPercentWad,
    uint256 tauYearsWad
) pure returns (uint256) {
    // Sqrt will return a value in 1E9 units.
    uint256 sqrtTauNotWad = FixedPointMathLib.sqrt(tauYearsWad);
    // Normalize it back to WAD units.
    uint256 sqrtTauWad = sqrtTauNotWad * SQRT_WAD;
    // Find the product of the WAD values.
    return FixedPointMathLib.mulWadDown(sigmaPercentWad, sqrtTauWad);
}

/// @param reserveXWad Total quantity of X tokens in the pool, in WAD units.
/// @param reserveYWad Total quantity of Y tokens in the pool, in WAD units.
/// @param totalLiquidity Total liquidity in the pool, in WAD units.
/// @param params Parameters of the Log Normal distribution.
/// @return int256 Gaussian.ppf(x / L) + Gaussian.ppf(y / KL) + sigma * sqrt(tau)
function tradingFunction(
    uint256 reserveXWad,
    uint256 reserveYWad,
    uint256 totalLiquidity,
    Parameters memory params
) pure returns (int256) {
    require(reserveXWad < totalLiquidity, "tradingFunction: invalid x");
    int256 AAAAA = Gaussian.ppf(
        int256(FixedPointMathLib.divWadDown(reserveXWad, totalLiquidity))
    );

    // note: arithmetic overflow/underflow can occur here if KL > Y.
    int256 BBBBB = Gaussian.ppf(
        int256(
            FixedPointMathLib.divWadDown(
                reserveYWad,
                FixedPointMathLib.mulWadDown(
                    params.strikePriceWad, totalLiquidity
                )
            )
        )
    );

    int256 CCCCC = int256(
        computeSigmaSqrtTau({
            sigmaPercentWad: params.sigmaPercentWad,
            tauYearsWad: params.tauYearsWad
        })
    );

    return AAAAA + BBBBB + CCCCC;
}

/// @dev Contract that holds the strategy parameterization and validation function.
interface Source {
    function init(bytes calldata data)
        external
        returns (
            bool valid,
            int256 swapConstantGrowth,
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );

    function validate(bytes calldata data)
        external
        view
        returns (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta,
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );

    function internalPrice(
        uint256 reserveXWad,
        uint256 totalLiquidity
    ) external view returns (uint256 price);

    function getNextLiquidity(
        uint256 reserveXWad,
        uint256 reserveYWad,
        uint256 totalLiquidity
    ) external view returns (uint256);
}

/// @dev Contract that holds the reserve and liquidity state.
interface Core {
    function getReservesAndLiquidity()
        external
        view
        returns (
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );
}

/// @notice Log Normal has three variable parameters:
/// K - strike price
/// sigma - volatility
/// tau - time to expiry
///
/// Swaps are validated by the trading function:
/// Gaussian.ppf(x / L) + Gaussian.ppf(y / KL) = -sigma * sqrt(tau)
contract LogNormal is Source {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    uint256 public constant APPROXIMATED_MINIMUM_X_INPUT = 10;
    uint256 public constant BISECTION_EPSILON = 1;
    uint256 public constant MAX_BISECTION_ITERS = 100;
    uint256 public constant HALF_WAD = 0.5e18;
    int256 public constant TWO_WAD = int256(2e18);
    uint256 public constant WAD = 1e18;
    uint256 public constant ZERO = 0;

    uint256 public swapFeePercentageWad;
    Parameters public __slot__;

    uint256 private lastSigma;
    uint256 public targetSigma;
    uint256 private lastSigmaSync;
    uint256 private sigmaUpdatePerSecond;
    uint256 private sigmaUpdateEnd;

    uint256 private lastStrike;
    uint256 public targetStrike;
    uint256 private lastStrikeSync;
    uint256 private strikeUpdatePerSecond;
    uint256 private strikeUpdateEnd;

    uint256 private lastTau;
    uint256 private targetTau;
    uint256 private lastTauSync;
    uint256 private tauUpdatePerSecond;
    uint256 private tauUpdateEnd;

    constructor(uint256 swapFeePercentageWad_) {
        require(
            swapFeePercentageWad_ < WAD,
            "swap fee percentage must be less than 100%"
        );
        swapFeePercentageWad = swapFeePercentageWad_;
    }

    modifier onlyCore() {
        // todo:
        _;
    }

    /// @dev Returns the original parameters that were used to initialize the pool.
    function staticSlot() public view returns (Parameters memory) {
        return __slot__;
    }

    /// @dev Slot holds out parameters, these return the dyanmic parameters.
    function dynamicSlot() public view returns (Parameters memory params) {
        (params.strikePriceWad, params.sigmaPercentWad, params.tauYearsWad) =
            (strikePrice(), sigma(), tau());
    }

    function _syncDynamicSlot() internal {
        Parameters memory params = staticSlot();

        targetSigma = params.sigmaPercentWad;
        lastSigma = params.sigmaPercentWad;
        sigmaUpdateEnd = block.timestamp;
        lastSigmaSync = block.timestamp;

        targetStrike = params.strikePriceWad;
        lastStrike = params.strikePriceWad;
        strikeUpdateEnd = block.timestamp;
        lastStrikeSync = block.timestamp;

        targetTau = params.tauYearsWad;
        lastTau = params.tauYearsWad;
        tauUpdateEnd = block.timestamp;
        lastTauSync = block.timestamp;
    }

    function getNextLiquidity(
        uint256 reserveXWad,
        uint256 reserveYWad,
        uint256 totalLiquidity
    ) public view returns (uint256) {
        int256 swapConstant = computeSwapConstant(
            abi.encode(reserveXWad, reserveYWad, totalLiquidity)
        );
        return
            findLiquidity(reserveXWad, reserveYWad, swapConstant, dynamicSlot());
    }

    /// @dev Computes the result of the tradingFunction().
    function computeSwapConstant(bytes memory data)
        public
        view
        returns (int256)
    {
        (uint256 reserveXWad, uint256 reserveYWad, uint256 totalLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));
        return tradingFunction({
            reserveXWad: reserveXWad,
            reserveYWad: reserveYWad,
            totalLiquidity: totalLiquidity,
            params: dynamicSlot()
        });
    }

    /// @dev Encodes the reserves, liquidity, and parameters for initialization.
    function encodeInitData(
        uint256 reserveXWad,
        uint256 reseveYWad,
        uint256 totalLiquidity,
        Parameters memory params
    ) public pure returns (bytes memory) {
        return abi.encode(reserveXWad, reseveYWad, totalLiquidity, params);
    }

    /// @dev Decodes and validates pool initialization parameters.
    /// Sets the `slot` state variable.
    function init(bytes calldata data)
        public
        onlyCore
        returns (
            bool valid,
            int256 swapConstantGrowth,
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        )
    {
        (reserveXWad, reserveYWad, totalLiquidity, __slot__) =
            abi.decode(data, (uint256, uint256, uint256, Parameters));

        _syncDynamicSlot();

        swapConstantGrowth = tradingFunction({
            reserveXWad: reserveXWad,
            reserveYWad: reserveYWad,
            totalLiquidity: totalLiquidity,
            params: dynamicSlot()
        });

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid =
            -int256(10) < swapConstantGrowth && swapConstantGrowth < int256(10);
    }

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function simulateSwap(
        bool swapXIn,
        uint256 amountIn
    ) public view onlyCore returns (bool, uint256, uint256, bytes memory) {
        // Note: these are the original values, but we use their variables to avoid stack too deep errors.
        (
            uint256 adjustedReserveXWad,
            uint256 adjustedReserveYWad,
            uint256 adjustedLiquidity
        ) = Core(msg.sender).getReservesAndLiquidity();

        // Make sure to override the original liquidity with the `getNextLiquidity` value.
        // This is because liquidity can change given any change in parameter, including over time via parameter tau.
        adjustedLiquidity = getNextLiquidity(
            adjustedReserveXWad, adjustedReserveYWad, adjustedLiquidity
        );

        int256 originalSwapConstant = computeSwapConstant(
            abi.encode(
                adjustedReserveXWad, adjustedReserveYWad, adjustedLiquidity
            )
        );

        uint256 amountOut;

        if (swapXIn) {
            uint256 fees = amountIn.mulWadUp(swapFeePercentageWad);
            uint256 liquidityDelta =
                fees.mulWadUp(adjustedLiquidity).divWadUp(adjustedReserveXWad);
            liquidityDelta += 1;

            adjustedReserveXWad += amountIn;
            adjustedLiquidity += liquidityDelta;

            uint256 originalReserveYWad = adjustedReserveYWad;
            adjustedReserveYWad = findY(
                adjustedReserveXWad,
                adjustedLiquidity,
                originalSwapConstant,
                dynamicSlot()
            );
            adjustedReserveYWad += 1;

            require(
                adjustedReserveYWad < originalReserveYWad,
                "invalid swap: y reserve increased!"
            );

            amountOut = originalReserveYWad - adjustedReserveYWad;
            console2.log("Esimated Y reserve to submit", adjustedReserveYWad);
        } else {
            uint256 fees = amountIn.mulWadUp(swapFeePercentageWad);
            uint256 liquidityDelta =
                fees.mulWadUp(adjustedLiquidity).divWadUp(adjustedReserveYWad);
            liquidityDelta += 1;

            adjustedReserveYWad += amountIn;
            adjustedLiquidity += liquidityDelta;

            uint256 originalReserveXWad = adjustedReserveXWad;
            adjustedReserveXWad = findX(
                adjustedReserveYWad,
                adjustedLiquidity,
                originalSwapConstant,
                dynamicSlot()
            );
            adjustedReserveXWad += 1;

            require(
                adjustedReserveXWad < originalReserveXWad,
                "invalid swap: x reserve increased!"
            );
            amountOut = originalReserveXWad - adjustedReserveXWad;
        }

        bytes memory swapData = abi.encode(
            adjustedReserveXWad, adjustedReserveYWad, adjustedLiquidity
        );
        (bool valid,,,,,) = validate(swapData);
        Parameters memory params = dynamicSlot();
        return (
            valid,
            amountOut,
            computePrice({
                x: adjustedReserveXWad,
                L: adjustedLiquidity,
                K: params.strikePriceWad,
                _sigma: params.sigmaPercentWad,
                _tau: params.tauYearsWad
            }),
            swapData
        );
    }

    /// @dev Finds the root of the swapConstant given the independent variable reserveXWad.
    function findY(
        uint256 reserveXWad,
        uint256 liquidity,
        int256 swapConstant,
        Parameters memory params
    ) public pure returns (uint256 reserveY) {
        uint256 lower = 10;
        uint256 upper = liquidity.mulWadUp(params.strikePriceWad) - 10;
        reserveY = bisection(
            abi.encode(reserveXWad, liquidity, swapConstant, params),
            lower,
            upper,
            BISECTION_EPSILON,
            MAX_BISECTION_ITERS,
            findRootY
        );
    }

    /// @dev Finds the root of the swapConstant given the independent variable reserveYWad.
    function findX(
        uint256 reserveYWad,
        uint256 liquidity,
        int256 swapConstant,
        Parameters memory params
    ) public pure returns (uint256 reserveY) {
        uint256 lower = 10;
        uint256 upper = liquidity - 10; // max x = 1 - x / l, so l - x
        reserveY = bisection(
            abi.encode(reserveYWad, liquidity, swapConstant, params),
            lower,
            upper,
            BISECTION_EPSILON,
            MAX_BISECTION_ITERS,
            findRootX
        );
    }

    /// @dev Finds the root of the swapConstant given the independent variable liquidity.
    function findLiquidity(
        uint256 reserveXWad,
        uint256 reserveYWad,
        int256 swapConstant,
        Parameters memory params
    ) public pure returns (uint256 liquidity) {
        uint256 lower = reserveXWad + 1;
        uint256 upper = 1e27;
        liquidity = bisection(
            abi.encode(reserveXWad, reserveYWad, swapConstant, params),
            lower,
            upper,
            BISECTION_EPSILON,
            MAX_BISECTION_ITERS,
            findRootLiquidity
        );
    }

    /// @dev Encodes the arguments for the swap validation function.
    function encodeValidateData(
        uint256 adjustedReserveXWad,
        uint256 adjustedReserveYWad,
        uint256 adjustedLiquidity
    ) public pure returns (bytes memory) {
        return abi.encode(
            adjustedReserveXWad, adjustedReserveYWad, adjustedLiquidity
        );
    }

    /// @dev Reverts if the caller is not a contract with the Core interface.
    function validate(bytes memory data)
        public
        view
        onlyCore
        returns (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta,
            uint256 adjustedReserveXWad,
            uint256 adjustedReserveYWad,
            uint256 adjustedLiquidity
        )
    {
        (
            uint256 originalReserveXWad,
            uint256 originalReserveYWad,
            uint256 originalLiquidity
        ) = Core(msg.sender).getReservesAndLiquidity();

        // Find the next liquidity and override the original liquidity with it.
        originalLiquidity = getNextLiquidity(
            originalReserveXWad, originalReserveYWad, originalLiquidity
        );

        (adjustedReserveXWad, adjustedReserveYWad, adjustedLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));

        // Rounding up is advantageous towards the protocol, to make sure swap fees
        // are fully paid for.
        uint256 minLiquidityDelta = 1;
        if (adjustedReserveXWad > originalReserveXWad) {
            // δl = δx * L / X, where δx = delta x * fee
            uint256 amountIn = adjustedReserveXWad - originalReserveXWad;
            uint256 fees = amountIn.mulWadUp(swapFeePercentageWad);
            minLiquidityDelta +=
                fees.mulWadUp(originalLiquidity).divWadUp(originalReserveXWad);
        } else if (adjustedReserveYWad > originalReserveYWad) {
            // δl = δx * L / X, where δx = delta x * fee
            uint256 amountIn = adjustedReserveYWad - originalReserveYWad;
            uint256 fees = amountIn.mulWadUp(swapFeePercentageWad);
            minLiquidityDelta +=
                fees.mulWadUp(originalLiquidity).divWadUp(originalReserveYWad);
        } else {
            // should never get here!
            revert("invalid swap: inputs x and y have the same sign!");
        }

        liquidityDelta = int256(adjustedLiquidity) - int256(originalLiquidity);

        swapConstantGrowth = tradingFunction({
            reserveXWad: adjustedReserveXWad,
            reserveYWad: adjustedReserveYWad,
            totalLiquidity: adjustedLiquidity,
            params: dynamicSlot()
        })
            - tradingFunction({
                reserveXWad: originalReserveXWad,
                reserveYWad: originalReserveYWad,
                totalLiquidity: originalLiquidity,
                params: dynamicSlot()
            });

        // Valid should check that the trading function growth is >= expected fee growth.
        valid = swapConstantGrowth >= int256(ZERO)
            && liquidityDelta >= int256(minLiquidityDelta);
    }

    uint256 public constant INFINITY_IS_NOT_REAL = type(uint256).max;

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(
        uint256 reserveXWad,
        uint256 totalLiquidity
    ) public view returns (uint256 price) {
        Parameters memory params = dynamicSlot();
        price = computePrice(
            reserveXWad,
            totalLiquidity,
            params.strikePriceWad,
            params.sigmaPercentWad,
            params.tauYearsWad
        );
    }

    function computePrice(
        uint256 x,
        uint256 L,
        uint256 K,
        uint256 _sigma,
        uint256 _tau
    ) public pure returns (uint256 spotPrice) {
        uint256 sigmaSqrtTau = computeSigmaSqrtTau(_sigma, _tau);
        uint256 halfSigmaPower2Tau = computeHalfSigmaPower2Tau(_sigma, _tau);
        uint256 R1 = FixedPointMathLib.divWadDown(x, L);

        spotPrice = FixedPointMathLib.mulWadUp(
            K,
            uint256(
                FixedPointMathLib.expWad(
                    Gaussian.ppf(int256(WAD - R1)) * int256(sigmaSqrtTau)
                        / int256(WAD) - int256(halfSigmaPower2Tau)
                )
            )
        );
    }

    function computeHalfSigmaPower2Tau(
        uint256 _sigma,
        uint256 _tau
    ) public pure returns (uint256 halfSigmaPower2Tau) {
        uint256 innerTerm = FixedPointMathLib.mulWadDown(
            uint256(FixedPointMathLib.powWad(int256(_sigma), int256(TWO))), _tau
        );

        halfSigmaPower2Tau = FixedPointMathLib.mulWadDown(HALF, innerTerm);
    }

    /// @dev Computes the approximated spot price given current reserves and liquidity.
    // function computePrice(
    //     uint256 reserveXWad,
    //     uint256 totalLiquidity,
    //     uint256 strikePriceWad,
    //     uint256 sigmaPercentWad,
    //     uint256 tauYearsWad
    // ) public pure returns (uint256 price) {
    // uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigmaPercentWad, tauYearsWad);
    // uint256 halfSigmaSquared = computeHalfSigmaSquared(sigmaPercentWad);
    // uint256 halfSigmaSquaredTau = halfSigmaSquared.mulWadDown(tauYearsWad);

    // // Gaussian.ppf has a range of [-inf, inf], so we need to make sure the input is in [0, 1].
    // int256 reserveXDivLiquidity =
    //     int256(reserveXWad.divWadDown(totalLiquidity));
    // // As x -> 1, price -> 0.
    // if (reserveXDivLiquidity >= int256(WAD)) {
    //     return 0;
    // }
    // // As x -> 0, price -> infinity.
    // if (reserveXDivLiquidity <= int256(ZERO)) {
    //     // todo: can returning an infinity price be worse than returning zero or reverting?
    //     return INFINITY_IS_NOT_REAL;
    // }
    // // The output can be negative so we have to be careful not to lose that information by casting.
    // int256 inverse_cdf_result =
    //     Gaussian.ppf(int256(WAD) - reserveXDivLiquidity);
    // int256 exponent = inverse_cdf_result * int256(sigmaSqrtTau)
    //     / int256(WAD) - int256(halfSigmaSquaredTau);

    // // This result cannot be negative!
    // int256 exp_result = FixedPointMathLib.expWad(exponent);
    // uint256 exp_result_uint = toUint(exp_result);
    // price = strikePriceWad.mulWadUp(exp_result_uint);
    //     uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    //     uint256 halfSigmaPower2Tau = computeHalfSigmaPower2Tau(sigma, tau);
    //     uint256 R1 = FixedPointMathLib.divWadDown(reserveXWad, liquidity);

    //     price = FixedPointMathLib.mulWadUp(
    //         strikePriceWad,
    //         uint256(
    //             FixedPointMathLib.expWad(
    //                 Gaussian.ppf(int256(WAD - reserveXWad))
    //                     * int256(sigmaSqrtTau) / int256(WAD)
    //                     - int256(halfSigmaPower2Tau)
    //             )
    //         )
    //     );
    // }

    // ===== Parameters ===== //

    function sigma() public view returns (uint256) {
        if (block.timestamp >= sigmaUpdateEnd) {
            return targetSigma;
        }

        return lastSigma > targetSigma
            ? lastSigma - (block.timestamp - lastSigmaSync) * sigmaUpdatePerSecond
            : lastSigma + (block.timestamp - lastSigmaSync) * sigmaUpdatePerSecond;
    }

    function strikePrice() public view returns (uint256) {
        if (block.timestamp >= strikeUpdateEnd) {
            return targetStrike;
        }

        return lastStrike > targetStrike
            ? lastStrike
                - (block.timestamp - lastStrikeSync) * strikeUpdatePerSecond
            : lastStrike
                + (block.timestamp - lastStrikeSync) * strikeUpdatePerSecond;
    }

    function tau() public view returns (uint256) {
        if (block.timestamp >= tauUpdateEnd) {
            return targetTau;
        }

        return lastTau > targetTau
            ? lastTau - (block.timestamp - lastTauSync) * tauUpdatePerSecond
            : lastTau + (block.timestamp - lastTauSync) * tauUpdatePerSecond;
    }

    function getParams() public view returns (uint256, uint256, uint256) {
        return (strikePrice(), sigma(), tau());
    }

    function _syncSigma() private {
        lastSigma = sigma();
        lastSigmaSync = block.timestamp;
    }

    function _syncStrike() private {
        lastStrike = strikePrice();
        lastStrikeSync = block.timestamp;
    }

    function _syncTau() private {
        lastTau = tau();
        lastTauSync = block.timestamp;
    }

    event LogParameters(
        uint256 sigma, uint256 strikePrice, uint256 tau, uint256 blockTimestamp
    );

    function setSigma(
        uint256 newTargetSigma,
        uint256 newSigmaUpdateEnd
    ) external {
        require(newSigmaUpdateEnd > block.timestamp, "Update end passed");

        _syncSigma();

        uint256 sigmaDelta = lastSigma > newTargetSigma
            ? lastSigma - newTargetSigma
            : newTargetSigma - lastSigma;

        sigmaUpdatePerSecond =
            sigmaDelta / (newSigmaUpdateEnd - block.timestamp);
        targetSigma = newTargetSigma;
        sigmaUpdateEnd = newSigmaUpdateEnd;
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    function setStrikePrice(
        uint256 newTargetStrike,
        uint256 newStrikeUpdateEnd
    ) external {
        require(newStrikeUpdateEnd > block.timestamp, "Update end passed");

        _syncStrike();

        uint256 strikeDelta = lastStrike > newTargetStrike
            ? lastStrike - newTargetStrike
            : newTargetStrike - lastStrike;

        strikeUpdatePerSecond =
            strikeDelta / (newStrikeUpdateEnd - block.timestamp);
        targetStrike = newTargetStrike;
        strikeUpdateEnd = newStrikeUpdateEnd;
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    function setTau(uint256 newTargetTau, uint256 newTauUpdateEnd) external {
        require(newTauUpdateEnd > block.timestamp, "Update end passed");

        _syncTau();

        uint256 tauDelta = lastTau > newTargetTau
            ? lastTau - newTargetTau
            : newTargetTau - lastTau;

        tauUpdatePerSecond = tauDelta / (newTauUpdateEnd - block.timestamp);
        targetTau = newTargetTau;
        tauUpdateEnd = newTauUpdateEnd;
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    // ===== //

    /// @dev Compute total liquidity given x reserves.
    /// @return L_x(x, S) = x * WAD / (WAD - Gaussian.cdf[d1(S, K, sigma, tau)])
    function lx(
        uint256 reserveXWad,
        uint256 priceWad,
        Parameters memory params
    ) public pure returns (uint256) {
        // Computes d1 = (ln(price / K) + tau * sigma^2 / 2)) / (sigma * sqrt(tau))
        int256 d1 = computeD1({ priceWad: priceWad, params: params });
        // Computes the cumulative distribution function of d1.
        int256 cdf = Gaussian.cdf(d1);
        // Cumulative distribution function's domain is [0, 1], so it can be casted to an unsigned integer safely.
        uint256 unsignedCdf = toUint(cdf);
        // L = x * WAD / (WAD - cdf(d1))
        require(unsignedCdf < WAD, "lx: denominator is zero");
        return reserveXWad.divWadDown(WAD - unsignedCdf);
    }

    /// @dev Computes total liquidity given y reserves.
    /// @return L_y(y, S) = y * WAD / (K * Gaussian.cdf[d2(S, K, sigma, tau)])
    function ly(
        uint256 reserveYWad,
        uint256 priceWad,
        Parameters memory params
    ) public pure returns (uint256) {
        int256 d2 = computeD2({ priceWad: priceWad, params: params });
        int256 cdf = Gaussian.cdf(d2);
        uint256 unsignedCdf = toUint(cdf);
        return reserveYWad.divWadDown(
            params.strikePriceWad.mulWadDown(unsignedCdf)
        );
    }

    /// @dev Computes reserves y given L(x, S).
    /// @return y(x, s) = K * L_x(x, S) * Gaussian.cdf[d2(S, K, sigma, tau)]
    function yl(
        uint256 totalLiquidity,
        uint256 priceWad,
        Parameters memory params
    ) public pure returns (uint256) {
        int256 d2 = computeD2({ priceWad: priceWad, params: params });
        int256 cdf = Gaussian.cdf(d2);
        uint256 unsignedCdf = toUint(cdf);
        return
            params.strikePriceWad.mulWadUp(totalLiquidity).mulWadUp(unsignedCdf);
    }

    /// @dev Computes reserves x given L(y, S).
    /// @return x(y, s) = L_y(y, S) * (WAD - Gaussian.cdf[d1(S, K, sigma, tau)])
    function xl(
        uint256 totalLiquidity,
        uint256 priceWad,
        Parameters memory params
    ) public pure returns (uint256) {
        int256 d1 = computeD1({ priceWad: priceWad, params: params });
        int256 cdf = Gaussian.cdf(d1);
        uint256 unsignedCdf = toUint(cdf);
        return totalLiquidity.mulWadUp(WAD - unsignedCdf);
    }

    function computeHalfSigmaSquared(uint256 sigmaPercentWad)
        public
        pure
        returns (uint256)
    {
        int256 sigmaSquaredWad = int256(sigmaPercentWad).powWad(TWO_WAD);
        return HALF_WAD.mulWadDown(uint256(sigmaSquaredWad));
    }

    /// @param priceWad The price of X in Y, in WAD units.
    /// @param params Parameters of the Log Normal distribution.
    /// @return d1 (ln(price / K) + tau * sigma^2 / 2)) / (sigma * sqrt(tau))
    function computeD1(
        uint256 priceWad,
        Parameters memory params
    ) public pure returns (int256 d1) {
        // sigma * sqrt(tau)
        uint256 sigmaSqrtTauWad = computeSigmaSqrtTau({
            sigmaPercentWad: params.sigmaPercentWad,
            tauYearsWad: params.tauYearsWad
        });
        // sigma^2 / 2
        uint256 halfSigmaSquaredWad =
            computeHalfSigmaSquared({ sigmaPercentWad: params.sigmaPercentWad });
        // ln(price / K), round UP because ln(1) = 0, but ln(0) = undefined.
        int256 logPriceOverStrikeWad = FixedPointMathLib.lnWad(
            int256(priceWad.divWadUp(params.strikePriceWad))
        );
        // Round up because the division is truncation in the lnWad function.
        logPriceOverStrikeWad++;
        // (ln(price / K) + tau * sigma^2 * tau / 2))
        int256 numerator = logPriceOverStrikeWad
            + int256(halfSigmaSquaredWad.mulWadDown(params.tauYearsWad));
        // sigma * sqrt(tau)
        int256 denominator = int256(sigmaSqrtTauWad);
        // (ln(price / K) + tau * sigma^2 / 2)) / (sigma * sqrt(tau))
        d1 = mulidivUp(numerator, int256(WAD), denominator);
    }

    /// @param priceWad The price of X in Y, in WAD units.
    /// @param params Parameters of the Log Normal distribution.
    /// @return d2 = d1 - sigma * sqrt(tau), alternatively d2 = (ln(S/K) - tau * sigma^2 / 2) / (sigma * sqrt(tau))
    function computeD2(
        uint256 priceWad,
        Parameters memory params
    ) public pure returns (int256 d2) {
        d2 = computeD1(priceWad, params)
            - int256(
                computeSigmaSqrtTau({
                    sigmaPercentWad: params.sigmaPercentWad,
                    tauYearsWad: params.tauYearsWad
                })
            );
    }

    /// @dev Signed mul div, rounding up if the modulo quotient is non-zero.
    function mulidivUp(
        int256 x,
        int256 y,
        int256 denominator
    ) public pure returns (int256 z) {
        z = mulidiv(x, y, denominator);
        if ((x * y) % denominator != 0) {
            require(z < type(int256).max, "mulidivUp overflow");
            z += 1;
        }
    }

    /// @notice Mul div signed integers.
    /// @dev From Solmate, but not in assembly.
    function mulidiv(
        int256 x,
        int256 y,
        int256 denominator
    ) public pure returns (int256 z) {
        unchecked {
            z = x * y;
            require(
                denominator != 0 && (x == 0 || z / x == y), "mulidiv invalid"
            );
            z = z / denominator;
        }
    }

    /// @dev Casts a positived signed integer to an unsigned integer, reverting if `x` is negative.
    function toUint(int256 x) public pure returns (uint256) {
        unchecked {
            require(x >= 0, "toUint: negative");
            return uint256(x);
        }
    }
}
