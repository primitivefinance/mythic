// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../DFMM.sol";
import "../../interfaces/IDFMM.sol";
import "./SetUp.sol";

contract DFMMInit is DFMMSetUp {
    function test_DFMM_init_RevertsWhenSameTokens() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(0xbeef),
            tokenY: address(0xbeef),
            data: ""
        });

        vm.expectRevert(IDFMM.InvalidTokens.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenNotValid() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(0xbeef),
            tokenY: address(0xbeef),
            data: ""
        });

        vm.expectRevert(IDFMM.InvalidTokens.selector);
        dfmm.init(params);
    }
}
