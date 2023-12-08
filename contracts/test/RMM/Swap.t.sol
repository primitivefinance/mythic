// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";
import "../../src/lib/RMMMath.sol";

contract RMMSwap is RMMSetUp {
    using stdStorage for StdStorage;
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for uint128;

    function testSwapWithAtomicArb() public {
        uint256 amountIn = 10506288307160081;
        // uint256 amountIn = 555588307160079;
        bool swapDirection = true; // swap in X get Y out

        (uint256 initX, uint256 initY, uint256 initL) =
            rmm.computeInitialPoolState(1 ether, initialPrice);

        rmm.initExactTokensAndLiquidity(initX, initY, initL);
        lex.setPrice(992047873705309300);

        uint256 lexPrice = lex.price();
        uint256 computedInput = amountIn * lexPrice / 1e18;
        uint256 nextLiquidity = rmm.getNextLiquidity();
        uint256 currLiquidity = rmm.totalLiquidity();
        console2.log("nextLiquidity", nextLiquidity);
        console2.log("currLiquidity", currLiquidity);

        // uint256 amountOut =
        //     rmm.getAmountOut(swapDirection, nextLiquidity, amountIn);
        // arb.lower_exchange_price(computedInput, amountOut, nextLiquidity);
        uint256 amountOut =
            rmm.getAmountOut(swapDirection, nextLiquidity, computedInput);
        uint256 amountY =
            rmm.swap(swapDirection, nextLiquidity, computedInput, amountOut);
        console2.log("amountY", amountY);
        uint256 amountInValue = amountIn * lexPrice / 1e18;
        console2.log("amountInValue", amountInValue);
        uint256 amountOutValue = amountOut;
        console2.log("amountOutValue", amountOutValue);
        int256 profit = int256(amountOutValue) - int256(amountInValue);
        console2.log("profit", profit);

        uint256 spotPriceFromX = rmm.getSpotPrice();
        uint256 spotPriceFromY = rmm.getSpotPriceFromY();
        console2.log("spotPriceFromX", spotPriceFromX);
        console2.log("spotPriceFromY", spotPriceFromY);
        console2.log("lex price", lexPrice);
    }

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
