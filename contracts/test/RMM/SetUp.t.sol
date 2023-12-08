// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../../src/RMM.sol";
import "../../src/RMMAtomicArbitrage.sol";
import "../../src/Lex.sol";

contract RMMSetUp is Test {
    MockERC20 public tokenX;
    MockERC20 public tokenY;
    RMM public rmm;
    RMMAtomicArbitrage public arb;
    Lex public lex;

    // uint256 initialPrice = 2000 ether;
    // uint256 strikePrice = 1800 ether;
    // uint256 sigma = 0.05 ether;
    // uint256 tau = 1 ether;
    // uint256 swapFee = 0.997 ether;

    uint256 initialPrice = 1 ether;
    uint256 strikePrice = 1 ether;
    uint256 sigma = 1 ether;
    uint256 tau = 1 ether;
    uint256 swapFee = 0.997 ether;

    function setUp() public {
        tokenX = new MockERC20("TokenX", "X", 18);
        tokenY = new MockERC20("TokenY", "Y", 18);
        rmm =
        new RMM(address(tokenX), address(tokenY), sigma, strikePrice, tau, swapFee);
        lex = new Lex(address(tokenX), address(tokenY), 1e18);
        arb =
        new RMMAtomicArbitrage(address(rmm), address(lex), address(tokenX), address(tokenY));

        tokenX.mint(address(this), type(uint256).max);
        tokenY.mint(address(this), type(uint256).max);
        tokenX.transfer(address(lex), 100 ether);
        tokenY.transfer(address(lex), 100 ether);
        tokenX.approve(address(arb), type(uint256).max);
        tokenY.approve(address(arb), type(uint256).max);
        tokenX.approve(address(rmm), type(uint256).max);
        tokenY.approve(address(rmm), type(uint256).max);
    }
}
