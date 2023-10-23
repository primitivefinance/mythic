// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract RMMSwap is RMMSetUp {
    function test_rmm_swap() public {
        uint256 amountX = 5_000 ether;
        rmm.initExactX(amountX, initialPrice);

        uint256 amountY = rmm.swap(500 ether);
        console.log(amountY);
    }
}
