// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/lib/RMMMath.sol";

contract ComputeLGivenX is Test {
    function test_rmm_computeLGivenX() public view {
        uint256 x = 5000 ether;
        uint256 S = 2000 ether;
        uint256 K = 1800 ether;
        uint256 sigma = 0.05 ether;

        console.log(computeLGivenX(x, S, K, sigma));
    }
}
