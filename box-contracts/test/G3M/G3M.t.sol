// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./Setup.t.sol";

contract G3MTest is Setup {
    function test_computeInvariant() public view {
        uint256 invariant =
            computeInvariantUp(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(invariant);
    }

    function test_computeSpotPrice() public view {
        uint256 spotPrice =
            computeSpotPrice(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(spotPrice);
    }

    function test_computeAmountInGivenExactLiquidity() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 liquidity = g3m.initPool(amountX, amountY);

        uint256 amountIn = computeAmountInGivenExactLiquidity(
            g3m.totalLiquidity(), liquidity, g3m.reserveX()
        );
        console.log(amountIn);
    }

    function test_computeAmountOutGivenExactLiquidity() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 liquidity = g3m.initPool(amountX, amountY);

        uint256 amountOut = computeAmountOutGivenExactLiquidity(
            g3m.totalLiquidity(), liquidity / 2, g3m.reserveX()
        );
        console.log(amountOut);
    }

    function test_computeOutGivenIn() public {
        g3m.initPool(750 ether, 250 ether);
        uint256 amountOut = computeOutGivenIn(
            toWad(50 ether),
            g3m.reserveX(),
            g3m.reserveY(),
            g3m.weightX(),
            g3m.weightY()
        );

        console.log(amountOut);
    }

    function test_swapAmountOut() public {
        uint256 liquidityX = 750 ether;
        uint256 liquidityY = 250 ether;
        g3m.initPool(liquidityX, liquidityY);

        uint256 initialBalanceX = tokenX.balanceOf(address(g3m));
        uint256 initialBalanceY = tokenY.balanceOf(address(g3m));
        assertEq(initialBalanceX, liquidityX);
        assertEq(initialBalanceY, liquidityY);

        uint256 amountOut = 15 ether;
        uint256 amountIn = g3m.swapAmountOut(true, amountOut);
        assertEq(tokenX.balanceOf(address(g3m)), initialBalanceX + amountIn);
        assertEq(tokenY.balanceOf(address(g3m)), initialBalanceY - amountOut);
    }
}
