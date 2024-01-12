// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract LogNormalAllocateTest is LogNormalSetUp {
    function test_LogNormal_allocate_GivenX() public init {
        uint256 amountX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 deltaLiquidity) =
            solver.allocateGivenX(POOL_ID, amountX);

        uint256 preLiquidityBalance = dfmm.balanceOf(address(this), POOL_ID);
        (,,,,,,, uint256 preTotalLiquidity,) = dfmm.pools(POOL_ID);

        bytes memory data = abi.encode(reserveX, reserveY, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        (,,,,,,, uint256 postTotalLiquidity,) = dfmm.pools(POOL_ID);
        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        assertEq(
            preLiquidityBalance + deltaTotalLiquidity,
            dfmm.balanceOf(address(this), POOL_ID)
        );
    }

    function test_LogNormal_allocate_GivenY() public init {
        uint256 amountX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 deltaLiquidity) =
            solver.allocateGivenY(POOL_ID, amountX);

        uint256 preLiquidityBalance = dfmm.balanceOf(address(this), POOL_ID);
        (,,,,,,, uint256 preTotalLiquidity,) = dfmm.pools(POOL_ID);

        bytes memory data = abi.encode(reserveX, reserveY, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        (,,,,,,, uint256 postTotalLiquidity,) = dfmm.pools(POOL_ID);
        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        assertEq(
            preLiquidityBalance + deltaTotalLiquidity,
            dfmm.balanceOf(address(this), POOL_ID)
        );
    }
}
