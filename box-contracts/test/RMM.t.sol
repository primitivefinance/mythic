// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../src/RMM.sol";

contract RMMTest is Test {
    MockERC20 public tokenX;
    MockERC20 public tokenY;
    RMM public rmm;

    uint256 initialPrice = 2000 ether;
    uint256 strikePrice = 1800 ether;
    uint256 sigma = 0.05 ether;
    uint256 tau = 1 ether;

    function setUp() public {
        tokenX = new MockERC20("TokenX", "X", 18);
        tokenY = new MockERC20("TokenY", "Y", 18);
        rmm = new RMM(tokenX, tokenY, sigma, strikePrice, tau);

        tokenX.mint(address(this), type(uint256).max);
        tokenY.mint(address(this), type(uint256).max);
        tokenX.approve(address(rmm), type(uint256).max);
        tokenY.approve(address(rmm), type(uint256).max);
    }

    function test_initExactX() public {
        uint256 amountX = 5000 ether;
        (uint256 l, uint256 amountY) = rmm.initExactX(amountX, initialPrice);

        console.log("l:", l);
        console.log("amountY:", amountY);

        assertEq(rmm.totalLiquidity(), l);
        assertEq(rmm.reserveX(), amountX);
        assertEq(rmm.reserveY(), amountY);
    }

    function test_initExactY() public {
        uint256 amountY = 2000 ether;
        (uint256 l, uint256 amountX) = rmm.initExactY(amountY, 2000 ether);

        assertEq(rmm.totalLiquidity(), l);
        assertEq(rmm.reserveX(), amountX);
        assertEq(rmm.reserveY(), amountY);
    }

    function test_addLiquidityExactX() public {
        uint256 amountX = 5_000 ether;
        (uint256 l, uint256 amountY) = rmm.initExactX(amountX, initialPrice);
        (uint256 l2, uint256 amountY2) = rmm.addLiquidityExactX(amountX);
    }

    function test_rmm_swap() public {
        uint256 amountX = 5_000 ether;
        rmm.initExactX(amountX, initialPrice);

        uint256 amountY = rmm.swap(500 ether);
        console.log(amountY);
    }
}
