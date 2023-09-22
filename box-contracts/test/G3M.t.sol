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

    /*
    function test_updatePrimaryWeight(uint256 newPrimaryWeight) public {
        vm.assume(newPrimaryWeight <= FixedPoint.ONE);
        g3m.updatePrimaryWeight(newPrimaryWeight);
        assertEq(g3m.weightX(), newPrimaryWeight);
        assertEq(g3m.weightY(), FixedPoint.ONE - newPrimaryWeight);
    }*/

    function test_computeInvariant() public {
        uint256 invariant = G3MMath.computeInvariant(5 ether, 0.5 ether, 5 ether, 0.5 ether);
        console.log(invariant);
    }

    function test_initPool() public {
        tokenX.mint(address(this), 100 ether);
        tokenY.mint(address(this), 100 ether);
        tokenX.approve(address(g3m), 100 ether);
        tokenY.approve(address(g3m), 100 ether);

        uint256 invariant = G3MMath.computeInvariant(5 ether, 0.5 ether, 5 ether, 0.5 ether);
        uint256 liquidity = g3m.initPool(5 ether, 5 ether);

        assertEq(tokenX.balanceOf(address(g3m)), 5 ether);
        assertEq(tokenY.balanceOf(address(g3m)), 5 ether);
        assertEq(g3m.reserveX(), 5 ether * 10 ** 18);
        assertEq(g3m.reserveY(), 5 ether * 10 ** 18);
        assertEq(g3m.totalLiquidity(), liquidity);
        assertApproxEqRel(g3m.totalLiquidity(), invariant * 2, 1_000);
        assertEq(g3m.getSpotPrice(), FixedPoint.ONE);
    }

    function test_addLiquidity() public {
        tokenX.mint(address(this), 100 ether);
        tokenY.mint(address(this), 100 ether);
        tokenX.approve(address(g3m), 100 ether);
        tokenY.approve(address(g3m), 100 ether);

        uint256 liquidity = g3m.initPool(5 ether, 5 ether);

        (uint256 amountX, uint256 amountY) = g3m.addLiquidity(liquidity);
        assertEq(g3m.reserveX(), 10 ether * 10 ** 18);
        assertEq(g3m.reserveY(), 10 ether * 10 ** 18);
        assertEq(g3m.totalLiquidity(), liquidity * 2);
        assertEq(amountX, 5 ether);
        assertEq(amountY, 5 ether);
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

    function test_swap() public {
        test_addLiquidity();

        g3m.updatePrimaryWeight(0.75 ether);
        g3m.swap(true, 2.5 ether);
    }
}
