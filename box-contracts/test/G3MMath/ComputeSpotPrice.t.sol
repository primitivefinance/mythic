// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/G3MMath.sol";

contract ComputeSpotPrice is Test {
    function test_computeSpotPrice_ComputesSpotPrice() public view {
        uint256 spotPrice =
            computeSpotPrice(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(spotPrice);
    }
}
