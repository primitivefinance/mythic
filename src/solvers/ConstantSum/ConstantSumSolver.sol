// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "src/interfaces/IStrategy.sol";
import "src/interfaces/IDFMM.sol";
import "src/lib/StrategyLib.sol";
import "src/strategies/ConstantSum/ConstantSum.sol";

contract ConstantSumSolver {
    error NotEnoughLiquidity();

    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    address public strategy;

    constructor(address strategy_) {
        strategy = strategy_;
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 ry,
        ConstantSum.ConstantSumParams memory params
    ) public pure returns (bytes memory) {
        // The pool can be initialized with any non-negative amount of rx, and ry.
        // so we have to allow a user to pass an amount of both even if one is zero.
        uint256 L = rx + ry.divWadUp(params.price);
        return abi.encode(rx, ry, L, params);
    }

    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, uint256, bytes memory) {
        Reserves memory startReserves;
        Reserves memory endReserves;
        (startReserves.rx, startReserves.ry, startReserves.L) =
            IDFMM(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
        ConstantSum.ConstantSumParams memory poolParams = abi.decode(
            IStrategy(strategy).getPoolParams(poolId),
            (ConstantSum.ConstantSumParams)
        );

        if (swapXIn) {
            uint256 fees = amountIn.mulWadUp(poolParams.swapFee);
            uint256 deltaL =
                fees.mulWadUp(startReserves.L.divWadDown(startReserves.rx));
            uint256 effectiveAmountIn = amountIn - fees;
            uint256 amountOut = effectiveAmountIn.mulWadDown(poolParams.price);

            endReserves.rx = startReserves.rx + amountIn;
            endReserves.L = startReserves.L + deltaL;

            if (startReserves.ry < amountOut) revert NotEnoughLiquidity();
            endReserves.ry = startReserves.ry - amountOut;
        } else {
            uint256 fees = amountIn.mulWadUp(poolParams.swapFee);
            uint256 deltaL =
                fees.mulWadUp(startReserves.L.divWadDown(startReserves.ry));
            uint256 effectiveAmountIn = amountIn - fees;
            uint256 amountOut =
                effectiveAmountIn.mulWadDown(ONE.divWadDown(poolParams.price));

            endReserves.ry = startReserves.ry + amountIn;
            endReserves.L = startReserves.L + deltaL;

            if (startReserves.rx < amountOut) revert NotEnoughLiquidity();
            endReserves.rx = startReserves.rx - amountOut;
        }
    }
}
