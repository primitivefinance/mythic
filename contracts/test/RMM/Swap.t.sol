// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";
import "../../src/lib/RMMMath.sol";


contract RMMSwap is RMMSetUp {
    using stdStorage for StdStorage;
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for uint128;
    function test_rmm_swap_UpdatesReserves() public {
        uint256 deltaX = 500 ether;

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 preReserveX = rmm.reserveX();
        uint256 preReserveY = rmm.reserveY();

        uint256 amountY = rmm.swapAmountIn(true, 500 ether);

        uint256 postReserveX = rmm.reserveX();
        uint256 postReserveY = rmm.reserveY();

        assertEq(preReserveX + deltaX, postReserveX);
        assertEq(preReserveY - amountY, postReserveY);
    }

    function test_rmm_swap_UpdatesRMMBalances() public {
        uint256 deltaX = 500 ether;

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 preBalanceX = tokenX.balanceOf(address(rmm));
        uint256 preBalanceY = tokenY.balanceOf(address(rmm));

        uint256 amountY = rmm.swapAmountIn(true, 500 ether);

        uint256 postBalanceX = tokenX.balanceOf(address(rmm));
        uint256 postBalanceY = tokenY.balanceOf(address(rmm));

        assertEq(preBalanceX + deltaX, postBalanceX);
        assertEq(preBalanceY - amountY, postBalanceY);
    }

    function test_rmm_swap_UpdatesSenderBalances() public {
        uint256 deltaX = 500 ether;

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 preBalanceX = tokenX.balanceOf(address(this));
        uint256 preBalanceY = tokenY.balanceOf(address(this));

        uint256 amountY = rmm.swapAmountIn(true, 500 ether);

        uint256 postBalanceX = tokenX.balanceOf(address(this));
        uint256 postBalanceY = tokenY.balanceOf(address(this));

        assertEq(preBalanceX - deltaX, postBalanceX);
        assertEq(preBalanceY + amountY, postBalanceY);
    }

    function test_rmm_new_liquidity() public {
        rmm.initExactX(5_000 ether, initialPrice);
        uint256 strike = rmm.strikePrice();
        stdstore.target(address(rmm)).sig(rmm.targetStrike.selector).checked_write(1780 ether);
        uint256 liquidity = rmm.totalLiquidity();
        uint256 x = rmm.reserveX();
        console2.log(liquidity);
        (uint256 lower, uint256 upper) = rmm.getSwapUpperLower();

        console2.log("lower", lower);
        console2.log("upper", upper);
        uint256 newLiquidity = rmm.getNewLFromParameters();

        console2.log(newLiquidity);

        int256 newSwapConstant = rmm.checkSwapConstant(newLiquidity);
        console2.log("newSwapConstant", newSwapConstant);
    }
}
