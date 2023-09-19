// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../src/G3M.sol";

contract G3MTest is Test {
    G3M public g3m;
    MockERC20 public tokenX;
    MockERC20 public tokenY;

    function setUp() public {
        tokenX = new MockERC20("TokenX", "X", 18);
        tokenY = new MockERC20("TokenY", "Y", 18);
        g3m = new G3M(address(tokenX), address(tokenY), 0.5 ether);
    }

    function test_updatePrimaryWeight(uint256 newPrimaryWeight) public {
        vm.assume(newPrimaryWeight <= g3m.WAD());
        g3m.updatePrimaryWeight(newPrimaryWeight);
        assertEq(g3m.getPrimaryWeight(), newPrimaryWeight);
        assertEq(g3m.getSecondaryWeight(), g3m.WAD() - newPrimaryWeight);
    }

    function test_addLiquidity() public {
        tokenX.mint(address(this), 100 ether);
        tokenY.mint(address(this), 100 ether);
        tokenX.approve(address(g3m), 100 ether);
        tokenY.approve(address(g3m), 100 ether);

        g3m.addLiquidity(10 ether);

        assertEq(tokenX.balanceOf(address(g3m)), 5 ether);
        assertEq(tokenY.balanceOf(address(g3m)), 5 ether);
        assertEq(tokenX.balanceOf(address(this)), 95 ether);
        assertEq(tokenY.balanceOf(address(this)), 95 ether);
        assertEq(g3m.balanceOf(address(this)), 10 ether);
    }

    function test_removeLiquidity() public {
        tokenX.mint(address(this), 100 ether);
        tokenY.mint(address(this), 100 ether);
        tokenX.approve(address(g3m), 100 ether);
        tokenY.approve(address(g3m), 100 ether);

        g3m.addLiquidity(10 ether);
        g3m.removeLiquidity(5 ether);

        assertEq(g3m.balanceOf(address(this)), 5 ether);
    }
}
