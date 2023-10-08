// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

import "../src/lib/RMMMath.sol";

contract RMMMathTest is Test {
    function test_computeLGivenX() public view {
        uint256 x = 2000 ether;
        uint256 K = 2000 ether;
        uint256 S = 1800 ether;
        int256 sigma = 1e18;

        console.log(computeLGivenX(x, S, K, uint256(sigma)));
    }
}
