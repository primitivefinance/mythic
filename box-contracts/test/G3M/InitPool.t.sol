// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract InitPool is SetUp {
    function test_initPool_InitializesPool() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        UD60x18 invariant = computeInvariant(
            convert(amountX), ud(0.5 ether), convert(amountY), ud(0.5 ether)
        );
        UD60x18 liquidity = g3m.initPool(amountX, amountY);

        assertEq(tokenX.balanceOf(address(g3m)), amountX);
        assertEq(tokenY.balanceOf(address(g3m)), amountY);
        assertEq(g3m.reserveX(), convert(amountX));
        assertEq(g3m.reserveY(), convert(amountY));
        assertEq(g3m.totalLiquidity(), liquidity + BURNT_LIQUIDITY);
        assertEq(g3m.getSpotPrice(), 3 ether);
        assertEq(g3m.balanceOf(address(this)), liquidity);
        assertEq(g3m.balanceOf(address(0)), BURNT_LIQUIDITY);
        assertEq(g3m.totalLiquidity(), invariant * ud(2));
        assertEq(
            g3m.balanceOf(address(this)), invariant * ud(2) - BURNT_LIQUIDITY
        );
    }

    function test_initPool_Revert_PoolAlreadyInitialized() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;
        g3m.initPool(amountX, amountY);
        vm.expectRevert("Pool already initialized");
        g3m.initPool(amountX, amountY);
    }
}
