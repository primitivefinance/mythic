// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../src/G3M.sol";

contract Setup is Test {
    G3M public g3m;
    MockERC20 public tokenX;
    MockERC20 public tokenY;

    function setUp() public {
        tokenX = new MockERC20("TokenX", "X", 18);
        tokenY = new MockERC20("TokenY", "Y", 18);

        tokenX.mint(address(this), 20000 ether);
        tokenY.mint(address(this), 20000 ether);

        g3m = new G3M(address(tokenX), address(tokenY), 0.5 ether);

        tokenX.approve(address(g3m), 20000 ether);
        tokenY.approve(address(g3m), 20000 ether);
    }
}
