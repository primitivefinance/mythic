// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../helpers/SetUp.sol";
import "../helpers/MockStrategy.sol";

contract DFMMSetUp is SetUp {
    MockStrategy strategy;

    function setUp() public {
        globalSetUp();
        strategy = new MockStrategy(address(dfmm));
    }
}
