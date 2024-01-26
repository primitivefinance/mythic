// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "src/LPToken.sol";

contract LPTokenTest is Test {
    LPToken lpToken;

    function setUp() public {
        lpToken = new LPToken();
    }

    function test_LPToken_initialize_InitializesTheContract() public {
        string memory name = "LPToken";
        string memory symbol = "LPT";
        lpToken.initialize(name, symbol);

        assertEq(lpToken.name(), name);
        assertEq(lpToken.symbol(), symbol);
        assertEq(lpToken.initialized(), true);
    }

    function test_LPToken_initialize_OnlyInitializesOnce() public {
        string memory name = "LPToken";
        string memory symbol = "LPT";
        lpToken.initialize(name, symbol);
        vm.expectRevert(LPToken.AlreadyInitialized.selector);
        lpToken.initialize(name, symbol);
    }
}
