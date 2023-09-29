// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./Setup.t.sol";

contract InitPool is Setup {
    function test_initPool_InitializesPool() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 invariant =
            computeInvariantDown(amountX, 0.5 ether, amountY, 0.5 ether);
        uint256 liquidity = g3m.initPool(amountX, amountY);

        assertEq(tokenX.balanceOf(address(g3m)), amountX);
        assertEq(tokenY.balanceOf(address(g3m)), amountY);
        assertEq(g3m.reserveX(), amountX * 10 ** 18);
        assertEq(g3m.reserveY(), amountY * 10 ** 18);
        assertEq(g3m.totalLiquidity(), liquidity + BURNT_LIQUIDITY);
        assertEq(g3m.getSpotPrice(), toWad(3));
        assertEq(g3m.balanceOf(address(this)), liquidity);
        assertEq(g3m.balanceOf(address(0)), 1_000);

        // TODO: Not a huge fan of using approx, let's see if we can use
        // something better here.
        assertApproxEqRel(g3m.totalLiquidity(), invariant * 2, 1_000);
        assertApproxEqRel(
            g3m.balanceOf(address(this)), invariant * 2 - BURNT_LIQUIDITY, 1_000
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
