// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MSwapTest is G3MSetUp {
    function test_G3M_swap() public init {
        uint256 amountIn = 0.1 ether;
        bool swapXForY = true;

        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(G3M_POOL_ID, swapXForY, amountIn);
        assertEq(valid, true);
        dfmm.swap(G3M_POOL_ID, payload);
    }
}
