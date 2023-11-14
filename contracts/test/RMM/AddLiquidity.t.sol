// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract RMMAddLiquidity is RMMSetUp {
    function test_rmm_addLiquidityExactX() public {
        uint256 amountX = 5_000 ether;
        (uint256 l, uint256 amountY) = rmm.initExactX(amountX, initialPrice);
        (uint256 l2, uint256 amountY2) = rmm.addLiquidityExactX(amountX);
    }
}
