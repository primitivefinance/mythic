// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../lib/DynamicParamLib.sol";

contract DynamicParamLibTest is Test {
    using DynamicParamLib for DynamicParam;

    function test_DynamicParamLib_actualized_SameValueWhenEmptyStruct()
        public
    {
        DynamicParam memory param = DynamicParam({
            lastComputedValue: 100,
            updateEnd: 0,
            lastUpdateAt: 0,
            updatePerSecond: 0
        });

        assertEq(param.actualized(), 100);
        vm.warp(block.timestamp + 1 days);
        assertEq(param.actualized(), 100);
    }

    function test_DynamicParamLib_actualized_ValueIncreasesOverTime() public {
        DynamicParam memory param = DynamicParam({
            lastComputedValue: 1,
            updateEnd: 10,
            lastUpdateAt: 0,
            updatePerSecond: 1
        });

        vm.warp(5);
        assertEq(param.actualized(), 6);
    }

    function test_DynamicParamLib_actualized_ValueIncreasesUntilEnd() public {
        DynamicParam memory param = DynamicParam({
            lastComputedValue: 1,
            updateEnd: 10,
            lastUpdateAt: 0,
            updatePerSecond: 1
        });

        vm.warp(10);
        assertEq(param.actualized(), 11);
    }

    function test_DynamicParamLib_actualized_ValueDecreasesOverTime() public {
        DynamicParam memory param = DynamicParam({
            lastComputedValue: 10,
            updateEnd: 10,
            lastUpdateAt: 0,
            updatePerSecond: -1
        });

        vm.warp(5);
        assertEq(param.actualized(), 5);
    }

    function test_DynamicParamLib_actualized_ValueDecreasesUntilEnd() public {
        DynamicParam memory param = DynamicParam({
            lastComputedValue: 10,
            updateEnd: 10,
            lastUpdateAt: 0,
            updatePerSecond: -1
        });

        vm.warp(10);
        assertEq(param.actualized(), 0);
    }
}
