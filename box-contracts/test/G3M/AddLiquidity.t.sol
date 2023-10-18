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

    function test_addLiquidity_ExactX_IncreasesReservesAndTotalLiquidity()
        public
    {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        UD60x18 initLiquidity = g3m.initPool(initAmountX, initAmountY);
        (uint256 amountX, uint256 amountY, UD60x18 liquidity) =
            g3m.addLiquidity(true, initAmountX);

        assertEq(g3m.reserveX(), convert(initAmountX + amountX));
        assertEq(g3m.reserveY(), convert(initAmountY + amountY));
        assertEq(
            g3m.totalLiquidity(), initLiquidity + liquidity + BURNT_LIQUIDITY
        );
    }

    function test_addLiquidity_ExactX_TransferTokens() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        g3m.initPool(initAmountX, initAmountY);

        uint256 userPreBalanceX = tokenX.balanceOf(address(this));
        uint256 userPreBalanceY = tokenY.balanceOf(address(this));
        uint256 g3mPreBalanceX = tokenX.balanceOf(address(g3m));
        uint256 g3mPreBalanceY = tokenY.balanceOf(address(g3m));

        (uint256 amountX, uint256 amountY,) =
            g3m.addLiquidity(true, initAmountX);

        uint256 userPostBalanceX = tokenX.balanceOf(address(this));
        uint256 userPostBalanceY = tokenY.balanceOf(address(this));
        uint256 g3mPostBalanceX = tokenX.balanceOf(address(g3m));
        uint256 g3mPostBalanceY = tokenY.balanceOf(address(g3m));

        assertEq(userPostBalanceX, userPreBalanceX - amountX);
        assertEq(userPostBalanceY, userPreBalanceY - amountY);
        assertEq(g3mPostBalanceX, g3mPreBalanceX + amountX);
        assertEq(g3mPostBalanceY, g3mPreBalanceY + amountY);
    }
}
