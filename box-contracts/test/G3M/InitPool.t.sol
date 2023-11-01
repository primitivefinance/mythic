// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { uMAX_UD60x18, uUNIT } from "@prb/math/ud60x18/Constants.sol";
import "./SetUp.t.sol";

contract G3MInitPool is G3MSetUp {
    function testFuzz_initPool_InitializesPool(
        uint256 amountX,
        uint256 amountY
    ) public {
        amountX = bound(amountX, 1, type(uint128).max);
        amountY = bound(amountY, 1, type(uint128).max);

        UD60x18 invariant = computeInvariant(
            convert(amountX), ud(0.5 ether), convert(amountY), ud(0.5 ether)
        );
        UD60x18 liquidity = g3m.initPool(amountX, amountY);

        assertEq(tokenX.balanceOf(address(g3m)), amountX);
        assertEq(tokenY.balanceOf(address(g3m)), amountY);
        assertEq(g3m.reserveX(), convert(amountX));
        assertEq(g3m.reserveY(), convert(amountY));
        assertEq(g3m.totalLiquidity(), liquidity + BURNT_LIQUIDITY);
        // assertEq(g3m.getSpotPrice(), 3 ether);
        assertEq(g3m.balanceOf(address(this)), liquidity);
        assertEq(g3m.balanceOf(address(0)), BURNT_LIQUIDITY);
        assertEq(g3m.totalLiquidity(), invariant * convert(2));
        assertEq(
            g3m.balanceOf(address(this)),
            invariant * convert(2) - BURNT_LIQUIDITY
        );
    }

    function testFuzz_initPool_Revert_PoolAlreadyInitialized(
        uint256 amountX,
        uint256 amountY
    ) public {
        amountX = bound(amountX, 1, type(uint128).max);
        amountY = bound(amountY, 1, type(uint128).max);
        g3m.initPool(amountX, amountY);
        vm.expectRevert("Pool already initialized");
        g3m.initPool(amountX, amountY);
    }

    function test_reservesWithoutPrecision(
        uint256 amountX,
        uint256 amountY
    ) public {
        amountX = bound(amountX, 1, type(uint128).max);
        amountY = bound(amountY, 1, type(uint128).max);
        g3m.initPool(amountX, amountY);

        assertEq(g3m.reserveX(), convert(amountX));
        assertEq(g3m.reserveY(), convert(amountY));

        assertEq(g3m.reserveXWithoutPrecision(), amountX);
        assertEq(g3m.reserveYWithoutPrecision(), amountY);
    }
}
