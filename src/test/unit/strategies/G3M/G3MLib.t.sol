// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "src/strategies/G3M/G3MLib.sol";

contract G3MLibTest is Test {
    function testFuzz_G3MLib_encodeFeeUpdate(uint256 swapFee) public {
        bytes memory data = G3MLib.encodeFeeUpdate(swapFee);
        assertEq(swapFee, G3MLib.decodeFeeUpdate(data));
    }

    function testFuzz_G3MLib_encodeWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public {
        bytes memory data =
            G3MLib.encodeWeightXUpdate(targetWeightX, targetTimestamp);

        (uint256 decodedTargetWeightX, uint256 decodedTargetTimestamp) =
            G3MLib.decodeWeightXUpdate(data);
        assertEq(targetWeightX, decodedTargetWeightX);
        assertEq(targetTimestamp, decodedTargetTimestamp);
    }

    function testFuzz_G3MLib_encodeControllerUpdate(address controller)
        public
    {
        bytes memory data = G3MLib.encodeControllerUpdate(controller);
        assertEq(controller, G3MLib.decodeControllerUpdate(data));
    }

    function test_G3MLib_tradingFunction() public {
        // TODO: Add a differential test here
    }

    function test_G3MLib_computeLiquidity() public {
        // TODO: Add a differential test here
    }
}
