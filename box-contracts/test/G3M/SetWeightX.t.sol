// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract SetWeightX is SetUp {
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

    function test_Revert_UpdateEndPasted() public {
        vm.expectRevert("Update end pasted");
        g3m.setWeightX(0.5 ether, block.timestamp - 1);
    }

    function testFuzz_setWeightX_Revert_WeightXTooLow(uint256 newTargetWeightX)
        public
    {
        vm.assume(newTargetWeightX < MIN_WEIGHT);
        vm.expectRevert("Weight X too low");
        g3m.setWeightX(newTargetWeightX, block.timestamp);
    }

    function testFuzz_setWeightX_Revert_WeightXTooHigh(uint256 newTargetWeightX)
        public
    {
        vm.assume(newTargetWeightX > MAX_WEIGHT);
        vm.expectRevert("Weight X too high");
        g3m.setWeightX(newTargetWeightX, block.timestamp);
    }
}
