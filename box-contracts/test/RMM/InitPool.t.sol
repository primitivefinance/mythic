// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract RMMInitPool is RMMSetUp {
    function test_rmm_initExactX() public {
        uint256 amountX = 5000 ether;
        (uint256 l, uint256 amountY) = rmm.initExactX(amountX, initialPrice);

        console.log("l:", l);
        console.log("amountY:", amountY);

        assertEq(rmm.totalLiquidity(), l);
        assertEq(rmm.reserveX(), amountX);
        assertEq(rmm.reserveY(), amountY);
    }

    function test_rmm_initExactY() public {
        uint256 amountY = 2000 ether;
        (uint256 l, uint256 amountX) = rmm.initExactY(amountY, 2000 ether);

        assertEq(rmm.totalLiquidity(), l);
        assertEq(rmm.reserveX(), amountX);
        assertEq(rmm.reserveY(), amountY);
    }
}
