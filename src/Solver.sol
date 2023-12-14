// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./v3/BisectionLib.sol";
import "./v3/LogNormalExtendedLib.sol";

interface StrategyLike {
    function computeSwapConstant(bytes memory) external view returns (int256);
    function dynamicSlot() external view returns (Parameters memory);
    function swapFeePercentageWad() external view returns (uint256);
    function getReservesAndLiquidity()
        external
        view
        returns (uint256, uint256, uint256);
    function validate(bytes calldata)
        external
        view
        returns (bool, int256, int256, uint256, uint256, uint256);
}

contract Solver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    uint256 public constant BISECTION_EPSILON = 1;
    uint256 public constant MAX_BISECTION_ITERS = 90;

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    /// @dev Encodes the reserves, liquidity, and parameters for initialization.
    function encodeInitData(
        uint256 reserveXWad,
        uint256 reserveYWad,
        uint256 totalLiquidity,
        Parameters memory params
    ) public pure returns (bytes memory) {
        return abi.encode(reserveXWad, reserveYWad, totalLiquidity, params);
    }

    function getNextLiquidity(
        uint256 reserveXWad,
        uint256 reserveYWad,
        uint256 totalLiquidity
    ) public view returns (uint256) {
        int256 swapConstant = StrategyLike(strategy).computeSwapConstant(
            abi.encode(reserveXWad, reserveYWad, totalLiquidity)
        );
        return computeNextLiquidity(
            reserveXWad,
            reserveYWad,
            swapConstant,
            totalLiquidity,
            StrategyLike(strategy).dynamicSlot()
        );
    }

    function getNextReserveX(
        uint256 reserveYWad,
        uint256 totalLiquidity
    ) public view returns (uint256) {
        int256 swapConstant = StrategyLike(strategy).computeSwapConstant(
            abi.encode(reserveYWad, totalLiquidity)
        );
        return computeNextReserveX(
            reserveYWad,
            totalLiquidity,
            swapConstant,
            StrategyLike(strategy).dynamicSlot()
        );
    }

    function getNextReserveY(
        uint256 reserveXWad,
        uint256 totalLiquidity
    ) public view returns (uint256) {
        int256 swapConstant = StrategyLike(strategy).computeSwapConstant(
            abi.encode(reserveXWad, totalLiquidity)
        );
        return computeNextReserveY(
            reserveXWad,
            totalLiquidity,
            swapConstant,
            StrategyLike(strategy).dynamicSlot()
        );
    }

    /// @dev Finds the root of the swapConstant given the independent variable liquidity.
    function computeNextLiquidity(
        uint256 reserveXWad,
        uint256 reserveYWad,
        int256 swapConstant,
        uint256 currentLiquidity,
        Parameters memory params
    ) public pure returns (uint256 liquidity) {
        uint256 lower;
        uint256 upper;
        uint256 iters;
        uint256 yOverK = reserveYWad.divWadDown(params.strikePriceWad);

        if (currentLiquidity != 0) {
            if (swapConstant < 20 && swapConstant > -20) {
                return currentLiquidity;
            } else if (swapConstant < 0) {
                upper = currentLiquidity;
                lower = reserveXWad > yOverK ? reserveXWad + 1 : yOverK + 1;
                iters = 150;
            } else {
                upper = 1e27;
                lower = currentLiquidity;
                iters = 150;
            }
        } else {
            upper = 1e27;
            lower = reserveXWad > yOverK ? reserveXWad + 1 : yOverK + 1;
            iters = 150;
        }
        liquidity = bisection(
            abi.encode(reserveXWad, reserveYWad, swapConstant, params),
            lower,
            upper,
            1,
            iters,
            findRootLiquidity
        );
    }

    /// @dev Finds the root of the swapConstant given the independent variable reserveXWad.
    function computeNextReserveY(
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
    function computeNextReserveX(
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

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function simulateSwap(
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, uint256, bytes memory) {
        (uint256 startRx, uint256 startRy, uint256 startL) =
            StrategyLike(strategy).getReservesAndLiquidity();
        Parameters memory poolParams = StrategyLike(strategy).dynamicSlot();

        uint256 amountOut;
        uint256 postRx;
        uint256 postRy;
        uint256 postL;
        {
            uint256 swapFee = StrategyLike(strategy).swapFeePercentageWad();
            uint256 startComputedLiquidity =
                getNextLiquidity(startRx, startRy, startL);

            if (swapXIn) {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 deltaL =
                    fees.mulWadUp(startComputedLiquidity).divWadUp(startRx);
                deltaL += 1;

                postRx = startRx + amountIn;
                postL = startL + deltaL;

                postRy = getNextReserveY(postRx, postL);
                postRy += 1;

                require(postRy < startRy, "invalid swap: y reserve increased!");
                amountOut = startRy - postRy;
            } else {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 deltaL =
                    fees.mulWadUp(startComputedLiquidity).divWadUp(startRx);
                deltaL += 1;

                postRy = startRx + amountIn;
                postL = startL + deltaL;

                postRx = getNextReserveX(postRy, postL);
                postRx += 1;

                require(postRx < startRx, "invalid swap: x reserve increased!");
                amountOut = startRx - postRx;
            }
        }

        bytes memory swapData = abi.encode(postRx, postRy, postL);
        (bool valid,,,,,) = StrategyLike(strategy).validate(swapData);
        return (
            valid,
            amountOut,
            computePrice({
                rx: postRx,
                L: postL,
                K: poolParams.strikePriceWad,
                sigma: poolParams.sigmaPercentWad,
                tau: poolParams.tauYearsWad
            }),
            swapData
        );
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(
        uint256 reserveXWad,
        uint256 totalLiquidity
    ) public view returns (uint256 price) {
        Parameters memory params = StrategyLike(strategy).dynamicSlot();
        price = computePrice(
            reserveXWad,
            totalLiquidity,
            params.strikePriceWad,
            params.sigmaPercentWad,
            params.tauYearsWad
        );
    }
}
