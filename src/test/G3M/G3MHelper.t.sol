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

    function test_G3MHelper_encodeWeightXUpdate() public {
        uint256 targetWeightX = 0.5 ether;
        uint256 targetTimestamp = 42;

        bytes memory data = encodeWeightXUpdate(targetWeightX, targetTimestamp);

        (uint256 decodedTargetWeightX, uint256 decodedTargetTimestamp) =
            decodeWeightXUpdate(data);
        assertEq(targetWeightX, decodedTargetWeightX);
        assertEq(targetTimestamp, decodedTargetTimestamp);
    }
}
