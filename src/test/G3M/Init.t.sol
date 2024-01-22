// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MInitTest is G3MSetUp {
    function test_G3M_init_StoresPoolParameters() public init {
        (
            bool inited,
            address controller,
            address strategy,
            address tokenX,
            address tokenY,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity,
        ) = dfmm.pools(POOL_ID);

        assertEq(inited, true);
        assertEq(controller, address(this));
        assertEq(strategy, address(g3m));
        assertEq(tokenX, address(tokenX));
        assertEq(tokenY, address(tokenY));
        assertEq(reserveX, defaultReserveX);
        assertEq(reserveY, reserveY);
        assertEq(totalLiquidity, totalLiquidity);
    }

    function test_G3M_init_RevertsIfInvalidTokens() public {
        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(g3m),
            tokenX: address(tokenX),
            tokenY: address(tokenX),
            data: defaultInitialPoolData
        });

        vm.expectRevert(IDFMM.InvalidTokens.selector);
        dfmm.init(initParams);
    }
}
