// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";
import "../../src/lib/RMMMath.sol";

contract RMMSwap is RMMSetUp {
    using stdStorage for StdStorage;
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for uint128;

    function test_rmm_swap_x_in() public {
        uint256 amountIn = 0.5 ether;
        bool swapDirection = true; // swap in X get Y out

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 preReserveX = rmm.reserveX();
        uint256 preReserveY = rmm.reserveY();

        // uint256 nextLiquidity = rmm.totalLiquidity();
        // console2.log("nextLiquidity", nextLiquidity);
        uint256 nextLiquidity = rmm.getNextLiquidity();
        console2.log("nextLiquidity", nextLiquidity);

        uint256 deltaL = rmm.getDeltaL(swapDirection, nextLiquidity, amountIn);
        console2.log("deltaL", deltaL);
        uint256 amountOut =
            rmm.getAmountOut(swapDirection, nextLiquidity, amountIn);
        console2.log("amountOut", amountOut);

        uint256 amountY =
            rmm.swap(swapDirection, nextLiquidity, amountIn, amountOut);
        console2.log("amountY", amountY);
    }

    function test_rmm_swap_y_in() public {
        uint256 amountIn = 0.5 ether;
        bool swapDirection = false; // swap in Y get X out

        rmm.initExactX(5_000 ether, initialPrice);

        // uint256 nextLiquidity = rmm.totalLiquidity();
        // console2.log("nextLiquidity", nextLiquidity);
        uint256 nextLiquidity = rmm.getNextLiquidity();
        console2.log("nextLiquidity", nextLiquidity);

        uint256 deltaL = rmm.getDeltaL(swapDirection, nextLiquidity, amountIn);
        console2.log("deltaL", deltaL);
        uint256 amountOut =
            rmm.getAmountOut(swapDirection, nextLiquidity, amountIn);
        console2.log("amountOut", amountOut);

        uint256 amountX =
            rmm.swap(swapDirection, nextLiquidity, amountIn, amountOut);
        console2.log("amountX", amountX);
    }

    function test_rmm_swap_x_in_after_update_strike() public {
        uint256 amountIn = 5 ether;
        bool swapDirection = true; // swap in X get Y out

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 strike = rmm.strikePrice();
        stdstore.target(address(rmm)).sig(rmm.targetStrike.selector)
            .checked_write(1.1 ether);
        uint256 nextLiquidity = rmm.getNextLiquidity();
        console2.log("nextLiquidity", nextLiquidity);

        uint256 deltaL = rmm.getDeltaL(swapDirection, nextLiquidity, amountIn);
        console2.log("deltaL", deltaL);
        uint256 amountOut =
            rmm.getAmountOut(swapDirection, nextLiquidity, amountIn);
        console2.log("amountOut", amountOut);

        uint256 amountY =
            rmm.swap(swapDirection, nextLiquidity, amountIn, amountOut);
        console2.log("amountY", amountY);
    }

    function test_rmm_swap_y_in_after_update_strike() public {
        uint256 amountIn = 5 ether;
        bool swapDirection = false; // swap in X get Y out

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 strike = rmm.strikePrice();
        stdstore.target(address(rmm)).sig(rmm.targetStrike.selector)
            .checked_write(1.1 ether);
        uint256 nextLiquidity = rmm.getNextLiquidity();
        console2.log("nextLiquidity", nextLiquidity);

        uint256 deltaL = rmm.getDeltaL(swapDirection, nextLiquidity, amountIn);
        console2.log("deltaL", deltaL);
        uint256 amountOut =
            rmm.getAmountOut(swapDirection, nextLiquidity, amountIn);
        console2.log("amountOut", amountOut);

        uint256 amountX =
            rmm.swap(swapDirection, nextLiquidity, amountIn, amountOut);
        console2.log("amountX", amountX);
    }

    // function test_rmm_swap_UpdatesReserves() public {
    //     uint256 amountIn = 500 ether;
    //     bool swapDirection = true; // swap in X get Y out

    //     rmm.initExactX(5_000 ether, initialPrice);

    //     uint256 preReserveX = rmm.reserveX();
    //     uint256 preReserveY = rmm.reserveY();

    //     uint256 nextLiquidity = rmm.totalLiquidity();

    //     uint256 virtualAmountOut = rmm.virtualSwap(swapDirection, nextLiquidity, amountIn);
    //     uint256 deltaL = rmm.getDeltaL(swapDirection, nextLiquidity, amountIn);
    //     uint256 amountOut = rmm.getAmountOut(swapDirection, nextLiquidity, amountIn);

    //     uint256 amountY = rmm.swap(swapDirection, nextLiquidity, amountIn, amountOut);

    //     uint256 postReserveX = rmm.reserveX();
    //     uint256 postReserveY = rmm.reserveY();

    //     assertEq(preReserveX + amountIn, postReserveX);
    //     assertEq(preReserveY - amountY, postReserveY);
    // }

    // function test_rmm_swap_UpdatesRMMBalances() public {
    //     uint256 amountIn = 500 ether;
    //     bool swapDirection = true; // swap in X get Y out

    //     rmm.initExactX(5_000 ether, initialPrice);

    //     uint256 preBalanceX = tokenX.balanceOf(address(rmm));
    //     uint256 preBalanceY = tokenY.balanceOf(address(rmm));

    //     uint256 nextLiquidity = rmm.totalLiquidity();
    //     console2.log("nextLiquidity", nextLiquidity);

    //     uint256 deltaL = rmm.getDeltaL(swapDirection, nextLiquidity, amountIn);
    //     console2.log("deltaL", deltaL);
    //     uint256 amountOut = rmm.getAmountOut(swapDirection, nextLiquidity, amountIn);
    //     console2.log("amountOut", amountOut);

    //     uint256 amountY = rmm.swapAmountIn(true, nextLiquidity, 500 ether);

    //     uint256 postBalanceX = tokenX.balanceOf(address(rmm));
    //     uint256 postBalanceY = tokenY.balanceOf(address(rmm));

    //     assertEq(preBalanceX + amountIn, postBalanceX);
    //     assertEq(preBalanceY - amountY, postBalanceY);
    // }

    // function test_rmm_swap_UpdatesSenderBalances() public {
    //     uint256 deltaX = 500 ether;

    //     rmm.initExactX(5_000 ether, initialPrice);

    //     uint256 preBalanceX = tokenX.balanceOf(address(this));
    //     uint256 preBalanceY = tokenY.balanceOf(address(this));

    //     uint256 nextLiquidity = rmm.totalLiquidity();

    //     uint256 amountY = rmm.swapAmountIn(true, nextLiquidity, 500 ether);

    //     uint256 postBalanceX = tokenX.balanceOf(address(this));
    //     uint256 postBalanceY = tokenY.balanceOf(address(this));

    //     assertEq(preBalanceX - deltaX, postBalanceX);
    //     assertEq(preBalanceY + amountY, postBalanceY);
    // }
}
