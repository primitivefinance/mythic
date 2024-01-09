// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "../lib/BisectionLib.sol";
import "../lib/lognormal/LogNormalExtendedLib.sol";

interface StrategyLike {
    function computeSwapConstant(bytes memory) external view returns (int256);
    function dynamicSlotInternal()
        external
        view
        returns (LogNormParameters memory);
    function swapFee() external view returns (uint256);
    function getReservesAndLiquidity()
        external
        view
        returns (uint256, uint256, uint256);
    function validate(bytes calldata)
        external
        view
        returns (bool, int256, int256, uint256, uint256, uint256);
}

contract LogNormalSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    uint256 public constant BISECTION_EPSILON = 1;
    uint256 public constant MAX_BISECTION_ITERS = 90;

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 S,
        LogNormParameters memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
    }

    function getNextLiquidity(
        uint256 rx,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        bytes memory data = abi.encode(rx, ry, L);
        int256 swapConstant = StrategyLike(strategy).computeSwapConstant(data);
        return computeNextLiquidity(
            rx,
            ry,
            swapConstant,
            L,
            StrategyLike(strategy).dynamicSlotInternal()
        );
    }

    function getNextReserveX(
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        (uint256 rx,,) = StrategyLike(strategy).getReservesAndLiquidity();
        bytes memory data = abi.encode(rx, ry, L);
        int256 swapConstant = StrategyLike(strategy).computeSwapConstant(data);
        return computeNextRx(
            ry,
            L,
            swapConstant,
            rx,
            StrategyLike(strategy).dynamicSlotInternal()
        );
    }

    function getNextReserveY(
        uint256 rx,
        uint256 L
    ) public view returns (uint256) {
        (, uint256 ry,) = StrategyLike(strategy).getReservesAndLiquidity();
        bytes memory data = abi.encode(rx, ry, L);
        int256 swapConstant = StrategyLike(strategy).computeSwapConstant(data);
        return computeNextRy(
            rx,
            L,
            swapConstant,
            ry,
            StrategyLike(strategy).dynamicSlotInternal()
        );
    }

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function simulateSwap(
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, uint256, bytes memory) {
        Reserves memory startReserves;
        Reserves memory endReserves;
        (startReserves.rx, startReserves.ry, startReserves.L) =
            StrategyLike(strategy).getReservesAndLiquidity();
        LogNormParameters memory poolParams =
            StrategyLike(strategy).dynamicSlotInternal();

        uint256 amountOut;
        {
            uint256 swapFee = StrategyLike(strategy).swapFee();
            uint256 startComputedL = getNextLiquidity(
                startReserves.rx, startReserves.ry, startReserves.L
            );

            if (swapXIn) {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 deltaL =
                    fees.mulWadUp(startComputedL).divWadUp(startReserves.rx);
                deltaL += 1;

                endReserves.rx = startReserves.rx + amountIn;
                endReserves.L = startReserves.L + deltaL;

                endReserves.ry = getNextReserveY(endReserves.rx, endReserves.L);
                endReserves.ry += 1;

                require(
                    endReserves.ry < startReserves.ry,
                    "invalid swap: y reserve increased!"
                );
                amountOut = startReserves.ry - endReserves.ry;
            } else {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 deltaL =
                    fees.mulWadUp(startComputedL).divWadUp(startReserves.ry);
                deltaL += 1;

                endReserves.ry = startReserves.ry + amountIn;
                endReserves.L = startReserves.L + deltaL;

                endReserves.rx = getNextReserveX(endReserves.ry, endReserves.L);
                endReserves.rx += 1;

                require(
                    endReserves.rx < startReserves.rx,
                    "invalid swap: x reserve increased!"
                );
                amountOut = startReserves.rx - endReserves.rx;
            }
        }

        bytes memory swapData =
            abi.encode(endReserves.rx, endReserves.ry, endReserves.L);
        (bool valid,,,,,) = StrategyLike(strategy).validate(swapData);
        return (
            valid,
            amountOut,
            computePrice({
                rx: endReserves.rx,
                L: endReserves.L,
                K: poolParams.strike,
                sigma: poolParams.sigma,
                tau: poolParams.tau
            }),
            swapData
        );
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice() public view returns (uint256 price) {
        LogNormParameters memory params =
            StrategyLike(strategy).dynamicSlotInternal();
        (uint256 rx,, uint256 L) =
            StrategyLike(strategy).getReservesAndLiquidity();
        price = computePrice(rx, L, params.strike, params.sigma, params.tau);
    }
}
