// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./Setup.t.sol";

contract G3MTest is Setup {
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
}
