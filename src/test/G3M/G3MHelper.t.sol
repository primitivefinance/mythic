/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../strategies/G3M/G3MHelper.sol";

contract G3MHelperTest is Test {
    function test_G3MHelper_encodeFeeUpdate() public {
        uint256 newSwapFee = 0.004 ether;
        bytes memory data = encodeFeeUpdate(newSwapFee);
        assertEq(newSwapFee, decodeFeeUpdate(data));
    }
}
