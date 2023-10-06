// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract AddLiquidity is SetUp {
    function test_addLiquidity_IncreasesReservesAndTotalLiquidity() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;

        UD60x18 liquidity = g3m.initPool(initAmountX, initAmountY);
        (uint256 amountX, uint256 amountY) =
            g3m.addLiquidity(liquidity + BURNT_LIQUIDITY);

        assertEq(g3m.reserveX(), convert(initAmountX + amountX));
        assertEq(g3m.reserveY(), convert(initAmountY + amountY));
        assertEq(
            g3m.totalLiquidity(), (liquidity + BURNT_LIQUIDITY) * convert(2)
        );
        assertEq(amountX, 750 ether);
        assertEq(amountY, 250 ether);
    }

    function test_addLiquidity_MaintainsSpotPrice() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        UD60x18 liquidity = g3m.initPool(initAmountX, initAmountY);
        uint256 oldSpotPrice = g3m.getSpotPrice();
        g3m.addLiquidity(liquidity);
        assertEq(g3m.getSpotPrice(), oldSpotPrice);
    }

    function test_addLiquidity_Revert_PoolNotInitialized() public {
        vm.expectRevert("Pool not initialized");
        g3m.addLiquidity(ud(100 ether));
    }
}
