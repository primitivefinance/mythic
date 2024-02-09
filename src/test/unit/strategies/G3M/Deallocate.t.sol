// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MDeallocateTest is G3MSetUp {
    function test_G3M_deallocate_GivenX_DecreasesTotalLiquidity() public init {
        uint256 amountX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 deltaLiquidity) =
            solver.deallocateGivenX(POOL_ID, amountX);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(reserveX, reserveY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
    }

    function test_G3M_deallocate_GivenX_UpdateReserves() public init {
        uint256 amountX = 0.1 ether;
        (uint256 preReserveX, uint256 preReserveY,) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        (uint256 reserveX, uint256 reserveY, uint256 deltaLiquidity) =
            solver.deallocateGivenX(POOL_ID, amountX);
        bytes memory data = abi.encode(reserveX, reserveY, deltaLiquidity);
        (uint256 deltaX, uint256 deltaY,) = dfmm.deallocate(POOL_ID, data);

        (uint256 postReserveX, uint256 postReserveY,) =
            dfmm.getReservesAndLiquidity(POOL_ID);
        assertEq(preReserveX - deltaX, postReserveX);
        assertEq(preReserveY - deltaY, postReserveY);
    }

    function test_G3M_deallocate_GivenX_TransfersTokens() public init {
        uint256 amountX = 0.1 ether;
        uint256 preBalanceX = tokenX.balanceOf(address(this));
        uint256 preBalanceY = tokenY.balanceOf(address(this));
        uint256 preBalanceXDFMM = tokenX.balanceOf(address(dfmm));
        uint256 preBalanceYDFMM = tokenY.balanceOf(address(dfmm));

        (uint256 reserveX, uint256 reserveY, uint256 deltaLiquidity) =
            solver.deallocateGivenX(POOL_ID, amountX);
        bytes memory data = abi.encode(reserveX, reserveY, deltaLiquidity);
        (uint256 deltaX, uint256 deltaY,) = dfmm.deallocate(POOL_ID, data);

        assertEq(preBalanceX + deltaX, tokenX.balanceOf(address(this)));
        assertEq(preBalanceY + deltaY, tokenY.balanceOf(address(this)));
        assertEq(preBalanceXDFMM - deltaX, tokenX.balanceOf(address(dfmm)));
        assertEq(preBalanceYDFMM - deltaY, tokenY.balanceOf(address(dfmm)));
    }

    function test_G3M_deallocate_GivenY() public init {
        uint256 amountX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 deltaLiquidity) =
            solver.deallocateGivenY(POOL_ID, amountX);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(reserveX, reserveY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
    }
}
