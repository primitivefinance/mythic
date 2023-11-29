// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";
import "../../src/lib/RMMMath.sol";

contract RMMParams is RMMSetUp {
    using stdStorage for StdStorage;
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for uint128;

    function test_rmm_new_liquidity_down_strike() public {
        rmm.initExactX(5_000 ether, initialPrice);

        uint256 strike = rmm.strikePrice();
        stdstore.target(address(rmm)).sig(rmm.targetStrike.selector)
            .checked_write(1780 ether);
        uint256 nextLiquidity = rmm.getNextLiquidity();

        int256 swapConstant = rmm.getSwapConstantGivenLiquidity(nextLiquidity);

        assertApproxEqAbs(swapConstant, 0, 100);
    }

    function test_rmm_new_liquidity_up_strike() public {
        rmm.initExactX(5_000 ether, initialPrice);

        uint256 strike = rmm.strikePrice();
        stdstore.target(address(rmm)).sig(rmm.targetStrike.selector)
            .checked_write(1820 ether);
        uint256 nextLiquidity = rmm.getNextLiquidity();

        int256 swapConstant = rmm.getSwapConstantGivenLiquidity(nextLiquidity);

        assertApproxEqAbs(swapConstant, 0, 100);
    }

    function test_rmm_new_liquidity_up_strike_down_sigma() public {
        rmm.initExactX(5_000 ether, initialPrice);

        uint256 strike = rmm.strikePrice();
        uint256 sigma = rmm.sigma();
        stdstore.target(address(rmm)).sig(rmm.targetStrike.selector)
            .checked_write(1820 ether);
        stdstore.target(address(rmm)).sig(rmm.targetSigma.selector)
            .checked_write(0.04 ether);
        uint256 nextLiquidity = rmm.getNextLiquidity();

        int256 swapConstant = rmm.getSwapConstantGivenLiquidity(nextLiquidity);

        assertApproxEqAbs(swapConstant, 0, 100);
    }

    function test_rmm_new_liquidity_up_strike_up_sigma() public {
        rmm.initExactX(5_000 ether, initialPrice);

        uint256 strike = rmm.strikePrice();
        uint256 sigma = rmm.sigma();
        stdstore.target(address(rmm)).sig(rmm.targetStrike.selector)
            .checked_write(1820 ether);
        stdstore.target(address(rmm)).sig(rmm.targetSigma.selector)
            .checked_write(0.06 ether);
        uint256 nextLiquidity = rmm.getNextLiquidity();

        int256 swapConstant = rmm.getSwapConstantGivenLiquidity(nextLiquidity);

        assertApproxEqAbs(swapConstant, 0, 100);
    }
}