// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../utils/G3MTest.t.sol";

contract Constants is G3MTest {
    function test_constants_WeightsSumIsOne() public {
        assertEq(MIN_WEIGHT + MAX_WEIGHT, UNIT);
    }
}
