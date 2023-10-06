// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract RemoveLiquidity is SetUp {
    function test_removeLiquidity_DecreasesReservesAndTotalLiquidity() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;

        UD60x18 liquidity = g3m.initPool(initAmountX, initAmountY);
        assertEq(g3m.reserveX(), convert(initAmountX));
        assertEq(g3m.reserveY(), convert(initAmountY));

        (uint256 amountX, uint256 amountY) =
            g3m.removeLiquidity(liquidity / convert(2));

        assertEq(g3m.reserveX(), convert(initAmountX) / convert(2));
        assertEq(g3m.reserveY(), convert(initAmountY) / convert(2));
        assertEq(g3m.totalLiquidity(), BURNT_LIQUIDITY + liquidity / convert(2));

        assertEq(amountX, initAmountX / 2);
        assertEq(amountY, initAmountY / 2);

        assertEq(tokenX.balanceOf(address(g3m)), initAmountX / 2);
        assertEq(tokenY.balanceOf(address(g3m)), initAmountY / 2);
    }

    function test_removeLiquidity_MaintainsSpotPrice() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        UD60x18 liquidity = g3m.initPool(initAmountX, initAmountY);
        uint256 oldSpotPrice = g3m.getSpotPrice();
        g3m.removeLiquidity(liquidity / convert(2));
        assertEq(g3m.getSpotPrice(), oldSpotPrice);
    }
}
