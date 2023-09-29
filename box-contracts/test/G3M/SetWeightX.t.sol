// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract SetWeightX is SetUp {
    /*
    function test_setWeightX_InstantaneousUpdatesWeights(uint256 newWeightX)
        public
    {
        vm.assume(newWeightX >= MIN_WEIGHT && newWeightX <= MAX_WEIGHT);
        g3m.setWeightX(newWeightX, block.timestamp);
        assertEq(g3m.weightX(), newWeightX);
        uint256 newWeightY = FixedPoint.ONE - newWeightX;
        assertEq(g3m.weightY(), newWeightY);
    }
    */

    function test_setWeightX_ReachesTargetWeightAfterTargetDate() public {
        uint256 newTargetWeightX = 0.75 ether;
        g3m.setWeightX(newTargetWeightX, block.timestamp + 1 weeks);
        assertEq(g3m.weightX(), 0.5 ether);
        assertEq(g3m.weightY(), 0.5 ether);
        vm.warp(block.timestamp + 1 weeks);
        assertEq(g3m.weightX(), 0.75 ether);
        assertEq(g3m.weightY(), 0.25 ether);
    }

    function test_setWeightX_UpdatesWeightGradually() public {
        uint256 newTargetWeightX = 0.7 ether;
        g3m.setWeightX(newTargetWeightX, block.timestamp + 20_000);
        assertEq(g3m.weightX(), 0.5 ether);
        assertEq(g3m.weightY(), 0.5 ether);
        vm.warp(block.timestamp + 10_000);
        assertEq(g3m.weightX(), 0.6 ether);
        assertEq(g3m.weightY(), 0.4 ether);
    }

    function test_setWeightX_Revert_NotAdmin() public {
        vm.expectRevert("Not admin");
        vm.prank(address(0xbeef));
        g3m.setWeightX(0.5 ether, block.timestamp);
    }
}
