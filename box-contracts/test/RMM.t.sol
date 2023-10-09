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

        tokenX.mint(address(this), type(uint256).max);
        tokenY.mint(address(this), type(uint256).max);
        tokenX.approve(address(rmm), type(uint256).max);
        tokenY.approve(address(rmm), type(uint256).max);
    }

    function test_initExactX() public {
        uint256 amountX = 2000 ether;
        (uint256 l, uint256 amountY) = rmm.initExactX(amountX, 2000 ether);

        assertEq(rmm.liquidity(), l);
        assertEq(rmm.reserveX(), amountX);
        assertEq(rmm.reserveY(), amountY);
    }

    function test_initExactY() public {
        uint256 amountY = 2000 ether;
        (uint256 l, uint256 amountX) = rmm.initExactY(amountY, 2000 ether);

        assertEq(rmm.liquidity(), l);
        assertEq(rmm.reserveX(), amountX);
        assertEq(rmm.reserveY(), amountY);
    }
}
