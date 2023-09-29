// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract RemoveLiquidity is SetUp {
    function test_removeLiquidity_DecreasesReservesAndTotalLiquidity() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;

        uint256 liquidity = g3m.initPool(initAmountX, initAmountY);
        (uint256 amountX, uint256 amountY) = g3m.removeLiquidity(liquidity / 2);

        assertEq(g3m.reserveX(), (initAmountX * 10 ** 18) / 2);
        assertEq(g3m.reserveY(), (initAmountY * 10 ** 18) / 2);
        assertApproxEqRel(g3m.totalLiquidity(), liquidity / 2, 1_000);

        assertEq(amountX, initAmountX / 2);
        assertEq(amountY, initAmountY / 2);

        assertEq(tokenX.balanceOf(address(g3m)), initAmountX / 2);
        assertEq(tokenY.balanceOf(address(g3m)), initAmountY / 2);
    }

    function test_removeLiquidity_MaintainsSpotPrice() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        uint256 liquidity = g3m.initPool(initAmountX, initAmountY);
        uint256 oldSpotPrice = g3m.getSpotPrice();
        g3m.removeLiquidity(liquidity / 2);
        assertEq(g3m.getSpotPrice(), oldSpotPrice);
    }
}
