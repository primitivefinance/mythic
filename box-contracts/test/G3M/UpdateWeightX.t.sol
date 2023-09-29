// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract UpdateWeightX is SetUp {
    function test_updateWeightX_InstantaneousUpdatesWeights(uint256 newWeightX)
        public
    {
        vm.assume(newWeightX >= MIN_WEIGHT && newWeightX <= MAX_WEIGHT);
        g3m.updateWeightX(newWeightX, block.timestamp);
        assertEq(g3m.weightX(), newWeightX);
        uint256 newWeightY = FixedPoint.ONE - newWeightX;
        assertEq(g3m.weightY(), newWeightY);
    }

    function test_updateWeightX_UpdatesLastWeightX() public {
        uint256 newTargetWeightX = 0.75 ether;
        g3m.updateWeightX(newTargetWeightX, block.timestamp + 1 weeks);
    }

    function test_updateWeightX_Revert_NotAdmin() public {
        vm.expectRevert("Not admin");
        vm.prank(address(0xbeef));
        g3m.updateWeightX(0.5 ether, block.timestamp);
    }
}
