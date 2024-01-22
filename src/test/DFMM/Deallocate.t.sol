// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract DFMMDeallocateTest is DFMMSetUp {
    /*
    function test_DFMM_deallocate_TransfersTokens() public {
        (uint256 poolId,,,) = dfmm.init(
            IDFMM.InitParams({
                strategy: address(strategy),
                tokenX: address(tokenX),
                tokenY: address(tokenY),
                data: abi.encode(uint256(2))
            })
        );

        uint256 preDFMMBalanceTokenX = tokenX.balanceOf(address(dfmm));
        uint256 preDFMMBalanceTokenY = tokenY.balanceOf(address(dfmm));
        uint256 preBalanceTokenX = tokenX.balanceOf(address(this));
        uint256 preBalanceTokenY = tokenY.balanceOf(address(this));

        console.log("Pre DFMM Balance Token X: %s", preDFMMBalanceTokenX);
        console.log("Pre DFMM Balance Token Y: %s", preDFMMBalanceTokenY);

        console.log("Pre Balance Token X: %s", preBalanceTokenX);
        console.log("Pre Balance Token Y: %s", preBalanceTokenY);

        (uint256 reserveX, uint256 reserveY, uint256 totalLiquidity) =
            dfmm.getReservesAndLiquidity(poolId);

        console.log("reserve X", reserveX);
        console.log("reserve Y", reserveY);
        console.log("total liquidity", totalLiquidity);

        (uint256 deltaX, uint256 deltaY,) =
            dfmm.deallocate(poolId, abi.encode(uint256(1)));

        console.log("deltaX: %s", deltaX);
        console.log("deltaY: %s", deltaY);

        assertEq(tokenX.balanceOf(address(dfmm)), preDFMMBalanceTokenX - deltaX);
        assertEq(tokenY.balanceOf(address(dfmm)), preDFMMBalanceTokenY - deltaY);
        assertEq(tokenX.balanceOf(address(this)), preBalanceTokenX + deltaX);
        assertEq(tokenY.balanceOf(address(this)), preBalanceTokenY + deltaY);

        (reserveX, reserveY, totalLiquidity) =
            dfmm.getReservesAndLiquidity(poolId);

        console.log("reserve X", reserveX);
        console.log("reserve Y", reserveY);
        console.log("total liquidity", totalLiquidity);
    }
    */

    function test_DFMM_deallocate_CannotDrainReserveX() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: abi.encode(uint256(1))
        });

        (uint256 poolId,,,) = dfmm.init(params);

        tokenX.mint(address(dfmm), 1000 ether);
        tokenY.mint(address(dfmm), 1000 ether);

        vm.expectRevert(stdError.arithmeticError);
        dfmm.deallocate(poolId, abi.encode(uint256(8)));
    }

    function test_DFMM_deallocate_CannotDrainReserveY() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: abi.encode(uint256(1))
        });

        (uint256 poolId,,,) = dfmm.init(params);

        tokenX.mint(address(dfmm), 1000 ether);
        tokenY.mint(address(dfmm), 1000 ether);

        vm.expectRevert(stdError.arithmeticError);
        dfmm.deallocate(poolId, abi.encode(uint256(9)));
    }
}
