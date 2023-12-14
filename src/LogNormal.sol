// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./v3/BisectionLib.sol";
import "./v3/LogNormalLib.sol";

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

    /// @notice the swap constant should never fall outside of range [-EPSILON, EPSILON]
    int256 constant EPSILON = 10;

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
            swapFeePercentageWad_ < ONE,
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

    function getReservesAndLiquidity()
        public
        view
        returns (uint256, uint256, uint256)
    {
        return Core(msg.sender).getReservesAndLiquidity();
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
        valid = -(EPSILON) < swapConstantGrowth && swapConstantGrowth < EPSILON;
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
        ) = getReservesAndLiquidity();

        (adjustedReserveXWad, adjustedReserveYWad, adjustedLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));

        // Rounding up is advantageous towards the protocol, to make sure swap fees
        // are fully paid for.
        uint256 minLiquidityDelta;
        uint256 amountIn;
        uint256 fees;
        if (adjustedReserveXWad > originalReserveXWad) {
            // δl = δx * L / X, where δx = delta x * fee
            amountIn = adjustedReserveXWad - originalReserveXWad;
            fees = amountIn.mulWadUp(swapFeePercentageWad);
            minLiquidityDelta +=
                fees.mulWadUp(originalLiquidity).divWadUp(originalReserveXWad);
        } else if (adjustedReserveYWad > originalReserveYWad) {
            // δl = δx * L / X, where δx = delta x * fee
            amountIn = adjustedReserveYWad - originalReserveYWad;
            fees = amountIn.mulWadUp(swapFeePercentageWad);
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

    event SetSigma(
        uint256 targetSigma,
        uint256 lastSigma,
        uint256 sigmaUpdateEnd,
        uint256 delta
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
        emit SetSigma(
            targetSigma,
            lastSigma,
            sigmaUpdateEnd,
            targetSigma > lastSigma
                ? targetSigma - lastSigma
                : lastSigma - targetSigma
        );
    }

    event SetStrikePrice(
        uint256 targetStrike,
        uint256 lastStrike,
        uint256 strikeUpdateEnd,
        uint256 delta
    );

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
        emit SetStrikePrice(
            targetStrike,
            lastStrike,
            strikeUpdateEnd,
            targetStrike > lastStrike
                ? targetStrike - lastStrike
                : lastStrike - targetStrike
        );
    }

    event SetTau(
        uint256 targetTau, uint256 lastTau, uint256 tauUpdateEnd, uint256 delta
    );

    function setTau(uint256 newTargetTau, uint256 newTauUpdateEnd) external {
        require(newTauUpdateEnd > block.timestamp, "Update end passed");

        _syncTau();

        uint256 tauDelta = lastTau > newTargetTau
            ? lastTau - newTargetTau
            : newTargetTau - lastTau;

        tauUpdatePerSecond = tauDelta / (newTauUpdateEnd - block.timestamp);
        targetTau = newTargetTau;
        tauUpdateEnd = newTauUpdateEnd;
        emit SetTau(
            targetTau,
            lastTau,
            tauUpdateEnd,
            targetTau > lastTau ? targetTau - lastTau : lastTau - targetTau
        );
    }
}
