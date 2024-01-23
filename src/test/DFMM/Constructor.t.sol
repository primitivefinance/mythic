/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract DFMMConstructorTest is DFMMSetUp {
    function test_DFMM_constructor_DeploysLPTokenImplementation() public {
        assertTrue(dfmm.lpTokenImplementation() != address(0));

        LPToken lpToken = LPToken(dfmm.lpTokenImplementation());
        assertTrue(lpToken.initialized());
    }
}
