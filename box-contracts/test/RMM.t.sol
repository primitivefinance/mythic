// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../src/RMM.sol";

contract RMMTest is Test {
    MockERC20 public tokenX;
    MockERC20 public tokenY;
    RMM public rmm;

    function setUp() public {
        tokenX = new MockERC20("TokenX", "X", 18);
        tokenY = new MockERC20("TokenY", "Y", 18);
        rmm = new RMM(tokenX, tokenY, 0.10 ether, 2000 ether);
    }
}
