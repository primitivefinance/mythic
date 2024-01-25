// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract LogNormalInitTest is LogNormalSetUp {
    function test_LogNormal_init_StoresPoolParameters() public init {
        (
            address controller,
            address strategy,
            address tokenX,
            address tokenY,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity,
        ) = dfmm.pools(POOL_ID);

        assertEq(controller, address(this));
        assertEq(strategy, address(logNormal));
        assertEq(tokenX, address(tokenX));
        assertEq(tokenY, address(tokenY));
        assertEq(reserveX, defaultReserveX);
        assertEq(reserveY, reserveY);
        assertEq(totalLiquidity, totalLiquidity);
    }

    function test_LogNormal_init_RevertsIfInvalidTokens() public {
        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: address(tokenX),
            tokenY: address(tokenX),
            data: defaultInitialPoolData
        });

        vm.expectRevert(IDFMM.InvalidTokens.selector);
        dfmm.init(initParams);
    }
}
