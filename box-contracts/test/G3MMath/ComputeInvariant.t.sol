// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/G3MMath.sol";

contract ComputeInvariant is Test {
    function test_computeInvariant_ComputesInvariant() public view {
        uint256 invariant =
            computeInvariantUp(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(invariant);
    }
}
