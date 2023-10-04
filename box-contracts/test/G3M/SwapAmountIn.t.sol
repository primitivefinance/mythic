// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { uMAX_UD60x18, uUNIT } from "@prb/math/ud60x18/Constants.sol";
import "./SetUp.t.sol";

contract SwapAmountIn is SetUp {
    function testFuzz_swapAmountIn_UpdatesBalances(
        uint256 reserveX,
        uint256 reserveY,
        bool swapDirection,
        uint256 amountIn
    ) public {
        reserveX = bound(reserveX, uUNIT, 600_000 ether);
        reserveY = bound(reserveY, uUNIT, 600_000 ether);

        uint256 maxSwap = (swapDirection ? reserveY : reserveX) / 3;
        vm.assume(amountIn > 1 ether && amountIn <= maxSwap);

        g3m.initPool(reserveX, reserveY);

        uint256 balanceXBefore = tokenX.balanceOf(address(g3m));
        uint256 balanceYBefore = tokenY.balanceOf(address(g3m));

        assertEq(balanceXBefore, reserveX);
        assertEq(balanceYBefore, reserveY);

        uint256 amountOut = g3m.swapAmountIn(swapDirection, amountIn);

        uint256 balanceXAfter = tokenX.balanceOf(address(g3m));
        uint256 balanceYAfter = tokenY.balanceOf(address(g3m));

        if (swapDirection) {
            assertEq(balanceXAfter, balanceXBefore + amountIn);
            assertEq(balanceYAfter, balanceYBefore - amountOut);
        } else {
            assertEq(balanceXAfter, balanceXBefore - amountOut);
            assertEq(balanceYAfter, balanceYBefore + amountIn);
        }
    }

    function test_swapAmountIn_UpdatesReserves() public {
        g3m.initPool(750 ether, 250 ether);
        UD60x18 reserveX = g3m.reserveX();
        UD60x18 reserveY = g3m.reserveY();

        uint256 amountIn = 50 ether;
        uint256 amountOut = g3m.swapAmountIn(true, amountIn);

        assertEq(g3m.reserveX(), reserveX + convert(amountIn));
        assertEq(g3m.reserveY(), reserveY - convert(amountOut));
    }

    function test_swapAmountIn_IncreasesSpotPrice() public {
        g3m.initPool(750 ether, 250 ether);
        uint256 spotPriceBefore = g3m.getSpotPrice();

        uint256 amountIn = 50 ether;
        g3m.swapAmountIn(true, amountIn);

        uint256 spotPriceAfter = g3m.getSpotPrice();
        assertTrue(spotPriceAfter > spotPriceBefore);
    }
}
