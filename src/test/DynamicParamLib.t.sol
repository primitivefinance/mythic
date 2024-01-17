// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../lib/DynamicParamLib.sol";

contract DynamicParamLibTest is Test {
    using DynamicParamLib for DynamicParam;

    DynamicParam storedParam = DynamicParam({
        lastComputedValue: 0,
        updateEnd: 0,
        lastUpdateAt: 0,
        updatePerSecond: 0
    });

    function initStoredParam(
        uint256 lastComputedValue,
        uint256 updateEnd,
        uint256 lastUpdateAt,
        int256 updatePerSecond
    ) internal {
        storedParam.lastComputedValue = lastComputedValue;
        storedParam.updateEnd = updateEnd;
        storedParam.lastUpdateAt = lastUpdateAt;
        storedParam.updatePerSecond = updatePerSecond;
    }

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

        vm.warp(20);
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

        vm.warp(20);
        assertEq(param.actualized(), 0);
    }

    function test_DynamicParamLib_sync_SyncsValue() public {
        initStoredParam(10, 10, 0, -1);
        vm.warp(5);
        storedParam.sync();
        assertEq(storedParam.lastComputedValue, 5);
        assertEq(storedParam.lastUpdateAt, 5);
    }

    function test_DynamicParamLib_set_SetsValueIncrease() public {
        initStoredParam(10, 0, 0, 0);
        storedParam.set(20, 10);
        assertEq(storedParam.lastComputedValue, 10);
        assertEq(storedParam.updateEnd, 10);
        assertEq(storedParam.lastUpdateAt, block.timestamp);
        assertEq(storedParam.updatePerSecond, 1);
    }

    function test_DynamicParamLib_set_SetsValueDecrease() public {
        initStoredParam(20, 0, 0, 0);
        storedParam.set(10, 10);
        assertEq(storedParam.lastComputedValue, 20);
        assertEq(storedParam.updateEnd, 10);
        assertEq(storedParam.lastUpdateAt, block.timestamp);
        assertEq(storedParam.updatePerSecond, -1);
    }

    function test_DynamicParamLib_set_RevertsWhenUpdateEndIsPast() public {
        initStoredParam(10, 0, 0, 0);
        vm.warp(0);
        vm.expectRevert(DynamicParamLib.InvalidUpdateEnd.selector);
        storedParam.set(20, 0);
    }
}
