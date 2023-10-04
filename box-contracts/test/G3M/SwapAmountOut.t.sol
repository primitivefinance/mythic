// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract SwapAmountOut is SetUp {
    function test_swapAmountOut_UpdatesBalances() public {
        uint256 liquidityX = 750 ether;
        uint256 liquidityY = 250 ether;
        g3m.initPool(liquidityX, liquidityY);

        uint256 initialBalanceX = tokenX.balanceOf(address(g3m));
        uint256 initialBalanceY = tokenY.balanceOf(address(g3m));
        assertEq(initialBalanceX, liquidityX);
        assertEq(initialBalanceY, liquidityY);

        uint256 amountOut = 15 ether;
        uint256 amountIn = g3m.swapAmountOut(true, amountOut);
        assertEq(tokenX.balanceOf(address(g3m)), initialBalanceX + amountIn);
        assertEq(tokenY.balanceOf(address(g3m)), initialBalanceY - amountOut);
    }
}
