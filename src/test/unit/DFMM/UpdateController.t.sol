// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract DFMMUpdateControllerTest is DFMMSetUp {
    function test_DFMM_updateController_UpdatesPoolController() public init {
        address newPoolController = address(0xbeef);
        dfmm.updateController(POOL_ID, newPoolController);
        assertEq(getPoolController(POOL_ID), newPoolController);
    }

    function test_DFMM_updateController_RevertsWhenSenderNotController()
        public
        init
    {
        address newPoolController = address(0xbeef);
        vm.prank(newPoolController);
        vm.expectRevert(IDFMM.NotController.selector);
        dfmm.updateController(POOL_ID, newPoolController);
    }
}
