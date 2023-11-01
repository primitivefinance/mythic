// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract G3MRemoveLiquidity is G3MSetUp {
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

    function test_removeLiquidity_ExactX_DecreasesReservesAndTotalLiquidity()
        public
    {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        UD60x18 initLiquidity = g3m.initPool(initAmountX, initAmountY);
        (uint256 amountX, uint256 amountY, UD60x18 liquidity) =
            g3m.removeLiquidity(true, initAmountX / 2);

        assertEq(g3m.reserveX(), convert(initAmountX - amountX));
        assertEq(g3m.reserveY(), convert(initAmountY - amountY));
        assertEq(
            g3m.totalLiquidity(), initLiquidity - liquidity + BURNT_LIQUIDITY
        );
    }

    function test_removeLiquidity_ExactX_TransferTokens() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        g3m.initPool(initAmountX, initAmountY);

        uint256 userPreBalanceX = tokenX.balanceOf(address(this));
        uint256 userPreBalanceY = tokenY.balanceOf(address(this));
        uint256 g3mPreBalanceX = tokenX.balanceOf(address(g3m));
        uint256 g3mPreBalanceY = tokenY.balanceOf(address(g3m));

        (uint256 amountX, uint256 amountY,) =
            g3m.removeLiquidity(true, initAmountX / 2);

        uint256 userPostBalanceX = tokenX.balanceOf(address(this));
        uint256 userPostBalanceY = tokenY.balanceOf(address(this));
        uint256 g3mPostBalanceX = tokenX.balanceOf(address(g3m));
        uint256 g3mPostBalanceY = tokenY.balanceOf(address(g3m));

        assertEq(userPostBalanceX, userPreBalanceX + amountX);
        assertEq(userPostBalanceY, userPreBalanceY + amountY);
        assertEq(g3mPostBalanceX, g3mPreBalanceX - amountX);
        assertEq(g3mPostBalanceY, g3mPreBalanceY - amountY);
    }
}
