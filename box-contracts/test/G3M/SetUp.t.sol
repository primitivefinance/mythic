// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/test/utils/mocks/MockERC20.sol";

import "../G3MTest.t.sol";
import "../../src/G3M.sol";

contract SetUp is G3MTest {
    G3M public g3m;
    MockERC20 public tokenX;
    MockERC20 public tokenY;

    function setUp() public {
        tokenX = new MockERC20("TokenX", "X", 18);
        tokenY = new MockERC20("TokenY", "Y", 18);

        tokenX.mint(address(this), type(uint256).max);
        tokenY.mint(address(this), type(uint256).max);

        g3m = new G3M(address(tokenX), address(tokenY), ud(0.5 ether));

        tokenX.approve(address(g3m), type(uint256).max);
        tokenY.approve(address(g3m), type(uint256).max);
    }
}
