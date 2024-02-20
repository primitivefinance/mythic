/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "src/lib/ScalingLib.sol";

contract ScalingLibTest is Test {
    function test_ScalingLib_computeScalingFactor_ComputesFor18Decimals()
        public
    {
        MockERC20 token = new MockERC20("Test", "TST", 18);
        assertEq(computeScalingFactor(address(token)), FixedPointMathLib.WAD);
    }

    function test_ScalingLib_computeScalingFactor_ComputesFor6Decimals()
        public
    {
        MockERC20 token = new MockERC20("Test", "TST", 6);
        assertEq(
            computeScalingFactor(address(token)),
            FixedPointMathLib.WAD * 10 ** 12
        );
    }

    function test_ScalingLib_upscale_ComputesFor18Decimals() public {
        MockERC20 token = new MockERC20("Test", "TST", 18);
        uint256 upscaledAmount =
            upscale(1 ether, computeScalingFactor(address(token)));
        assertEq(upscaledAmount, 1 ether);
    }

    function test_ScalingLib_upscale_ComputesFor6Decimals() public {
        MockERC20 token = new MockERC20("Test", "TST", 6);
        uint256 upscaledAmount =
            upscale(1 * 10 ** 6, computeScalingFactor(address(token)));
        assertEq(upscaledAmount, 1 ether);
    }

    function test_ScalingLib_downscaleDown_ComputesFor18Decimals() public {
        MockERC20 token = new MockERC20("Test", "TST", 18);
        uint256 initialAmount = 1 ether;
        uint256 upscaledAmount =
            upscale(initialAmount, computeScalingFactor(address(token)));
        assertEq(
            initialAmount,
            downscaleDown(upscaledAmount, computeScalingFactor(address(token)))
        );
    }

    function test_ScalingLib_downscaleDown_ComputesFor6Decimals() public {
        MockERC20 token = new MockERC20("Test", "TST", 6);
        uint256 initialAmount = 1 * 10 ** 6;
        uint256 upscaledAmount =
            upscale(initialAmount, computeScalingFactor(address(token)));
        assertEq(
            initialAmount,
            downscaleDown(upscaledAmount, computeScalingFactor(address(token)))
        );
    }

    function test_ScalingLib_downscaleUp_ComputesFor18Decimals() public {
        MockERC20 token = new MockERC20("Test", "TST", 18);
        uint256 initialAmount = 1 ether;
        uint256 upscaledAmount =
            upscale(initialAmount, computeScalingFactor(address(token)));
        assertEq(
            initialAmount,
            downscaleUp(upscaledAmount, computeScalingFactor(address(token)))
        );
    }

    function test_ScalingLib_downscaleUp_ComputesFor6Decimals() public {
        MockERC20 token = new MockERC20("Test", "TST", 6);
        uint256 initialAmount = 1 * 10 ** 6;
        uint256 upscaledAmount =
            upscale(initialAmount, computeScalingFactor(address(token)));
        assertEq(
            initialAmount,
            downscaleUp(upscaledAmount, computeScalingFactor(address(token)))
        );
    }
}
