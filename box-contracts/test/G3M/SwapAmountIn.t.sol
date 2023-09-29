// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract SwapAmountIn is SetUp {
    function test_swapAmountIn_UpdatesBalances() public {
        g3m.initPool(750 ether, 250 ether);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether);

        uint256 amountIn = 50 ether;
        uint256 amountOut = g3m.swapAmountIn(true, amountIn);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether + amountIn);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether - amountOut);
    }
}
