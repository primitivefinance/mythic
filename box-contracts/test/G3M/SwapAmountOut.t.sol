// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { uMAX_UD60x18, uUNIT } from "@prb/math/ud60x18/Constants.sol";
import "./SetUp.t.sol";

contract SwapAmountOut is SetUp {
    function testFuzz_swapAmountOut_UpdatesBalances(
        uint256 initialDepositX,
        uint256 initialDepositY,
        bool swapDirection,
        uint256 amountOut
    ) public {
        initialDepositX = bound(initialDepositX, uUNIT, 600_000 ether);
        initialDepositY = bound(initialDepositY, uUNIT, 600_000 ether);

        uint256 maxSwap =
            (swapDirection ? initialDepositY : initialDepositX) / 3;
        vm.assume(amountOut > 1 ether && amountOut <= maxSwap);

        g3m.initPool(initialDepositX, initialDepositY);

        uint256 balanceXBefore = tokenX.balanceOf(address(g3m));
        uint256 balanceYBefore = tokenY.balanceOf(address(g3m));

        assertEq(balanceXBefore, initialDepositX);
        assertEq(balanceYBefore, initialDepositY);

        uint256 amountInWithFees = g3m.swapAmountOut(swapDirection, amountOut);

        uint256 balanceXAfter = tokenX.balanceOf(address(g3m));
        uint256 balanceYAfter = tokenY.balanceOf(address(g3m));

        if (swapDirection) {
            assertEq(balanceXAfter, balanceXBefore + amountInWithFees);
            assertEq(balanceYAfter, balanceYBefore - amountOut);
        } else {
            assertEq(balanceXAfter, balanceXBefore - amountOut);
            assertEq(balanceYAfter, balanceYBefore + amountInWithFees);
        }
    }

    function testFuzz_swapAmountOut_UpdatesReserves(
        uint256 initialDepositX,
        uint256 initialDepositY,
        bool swapDirection,
        uint256 amountOut
    ) public {
        initialDepositX = bound(initialDepositX, uUNIT, 600_000 ether);
        initialDepositY = bound(initialDepositY, uUNIT, 600_000 ether);

        uint256 maxSwap =
            (swapDirection ? initialDepositY : initialDepositX) / 3;
        vm.assume(amountOut > 1 ether && amountOut <= maxSwap);

        g3m.initPool(initialDepositX, initialDepositY);

        UD60x18 reserveXBefore = g3m.reserveX();
        UD60x18 reserveYBefore = g3m.reserveY();

        uint256 amountInWithFees = g3m.swapAmountOut(swapDirection, amountOut);

        UD60x18 reserveXAfter = g3m.reserveX();
        UD60x18 reserveYAfter = g3m.reserveY();

        if (swapDirection) {
            assertEq(reserveXAfter, reserveXBefore + convert(amountInWithFees));
            assertEq(reserveYAfter, reserveYBefore - convert(amountOut));
        } else {
            assertEq(reserveXAfter, reserveXBefore - convert(amountOut));
            assertEq(reserveYAfter, reserveYBefore + convert(amountInWithFees));
        }
    }
}
