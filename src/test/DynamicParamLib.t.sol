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
}
