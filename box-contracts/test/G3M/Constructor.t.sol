// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract Constructor is SetUp {
    function test_constructor_MaxWeight() public {
        g3m = new G3M(address(tokenX), address(tokenY), MAX_WEIGHT);
        assertEq(g3m.weightX(), MAX_WEIGHT);
        assertEq(g3m.weightY(), MIN_WEIGHT);
    }

    function test_constructor_MinWeight() public {
        g3m = new G3M(address(tokenX), address(tokenY), MIN_WEIGHT);
        assertEq(g3m.weightX(), MIN_WEIGHT);
        assertEq(g3m.weightY(), MAX_WEIGHT);
    }

    function test_constructor_WeightsSumEqualsOne(uint256 weightX) public {
        vm.assume(ud(weightX) >= MIN_WEIGHT && ud(weightX) <= MAX_WEIGHT);
        g3m = new G3M(address(tokenX), address(tokenY), ud(weightX));
        assertEq(g3m.weightX() + g3m.weightY(), UNIT);
    }

    function test_constructor_Revert_InvalidTokens() public {
        vm.expectRevert("Invalid tokens");
        g3m = new G3M(address(tokenX), address(tokenX), ud(0.5 ether));
    }

    function test_constructor_Revert_WeightXTooHigh() public {
        vm.expectRevert("Weight X too high");
        g3m = new G3M(address(tokenX), address(tokenY), MAX_WEIGHT + ud(1));
    }

    function test_constructor_Revert_WeightXTooLow() public {
        vm.expectRevert("Weight X too low");
        g3m = new G3M(address(tokenX), address(tokenY), MIN_WEIGHT - ud(1));
    }
}
