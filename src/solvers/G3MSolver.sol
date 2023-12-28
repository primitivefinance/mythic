// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "../lib/BisectionLib.sol";
import "../lib/g3m/G3MExtendedLib.sol";

interface StrategyLike {
    function computeSwapConstant(bytes memory) external view returns (int256);
    function dynamicSlotInternal()
        external
        view
        returns (G3mParameters memory);
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

contract G3mSolver {
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
        G3mParameters memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
    }

    function getNextLiquidity(
        uint256 rx,
        uint256 ry
    ) public view returns (uint256) {
        return computeNextLiquidity(
            rx, ry, StrategyLike(strategy).dynamicSlotInternal()
        );
    }

    function getNextReserveX(
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        return
            computeNextRx(ry, L, StrategyLike(strategy).dynamicSlotInternal());
    }

    function getNextReserveY(
        uint256 rx,
        uint256 L
    ) public view returns (uint256) {
        return
            computeNextRy(rx, L, StrategyLike(strategy).dynamicSlotInternal());
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
        G3mParameters memory poolParams =
            StrategyLike(strategy).dynamicSlotInternal();
        console2.log("startRx", startReserves.rx);
        console2.log("startRy", startReserves.ry);
        console2.log("startL", startReserves.L);

        uint256 amountOut;
        {
            uint256 swapFee = StrategyLike(strategy).swapFee();
            uint256 startComputedL =
                getNextLiquidity(startReserves.rx, startReserves.ry);

            if (swapXIn) {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 weightedPrice = uint256(
                    int256(startReserves.ry.divWadUp(startReserves.rx)).powWad(
                        int256(poolParams.wy)
                    )
                );
                uint256 deltaL = fees.mulWadUp(weightedPrice);
                deltaL += 1;

                endReserves.rx = startReserves.rx + amountIn;
                endReserves.L = startComputedL + deltaL;

                endReserves.ry = getNextReserveY(endReserves.rx, endReserves.L);
                console2.log("endRy", endReserves.rx);
                endReserves.ry += 1;

                require(
                    endReserves.ry < startReserves.ry,
                    "invalid swap: y reserve increased!"
                );
                amountOut = startReserves.ry - endReserves.ry;
            } else {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 weightedPrice = uint256(
                    int256(startReserves.rx.divWadUp(startReserves.ry)).powWad(
                        int256(poolParams.wx)
                    )
                );
                uint256 deltaL = fees.mulWadUp(weightedPrice);
                deltaL += 1;

                endReserves.ry = startReserves.ry + amountIn;
                endReserves.L = startComputedL + deltaL;

                endReserves.rx = getNextReserveX(endReserves.ry, endReserves.L);
                console2.log("endRx", endReserves.rx);
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
            computePrice(endReserves.rx, endReserves.ry, poolParams),
            swapData
        );
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice() public view returns (uint256 price) {
        G3mParameters memory params =
            StrategyLike(strategy).dynamicSlotInternal();
        (uint256 rx, uint256 ry,) =
            StrategyLike(strategy).getReservesAndLiquidity();
        price = computePrice(rx, ry, params);
    }
}
