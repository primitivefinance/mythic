// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract DFMMAllocateTest is DFMMSetUp {
    function test_DFMM_allocate_RevertsWhenPoolNotInitialized() public {
        bytes memory empty;
        vm.expectRevert(stdError.indexOOBError);
        dfmm.allocate(32, empty);
    }

    function test_DFMM_allocate_RevertsWhenNoPoolWasInitialized() public {
        dfmm = new DFMM(address(0));
        bytes memory empty;
        vm.expectRevert(stdError.indexOOBError);
        dfmm.allocate(0, empty);
    }
}
