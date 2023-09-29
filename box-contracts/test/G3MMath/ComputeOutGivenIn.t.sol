// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/G3MMath.sol";

contract ComputeOutGivenIn is Test {
    function test_computeOutGivenIn_ComputesAmountOut() public view {
        uint256 amountOut = computeOutGivenIn(
            toWad(50 ether),
            toWad(750 ether),
            toWad(250 ether),
            0.5 ether,
            0.5 ether
        );

        console.log(amountOut);
    }
}
