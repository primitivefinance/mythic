// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract DeallocateTest is DFMMSetUp {
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
