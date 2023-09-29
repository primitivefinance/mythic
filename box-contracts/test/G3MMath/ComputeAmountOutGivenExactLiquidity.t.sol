// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/G3MMath.sol";

contract ComputeAmountOutGivenExactLiquidity is Test {
    function test_computeAmountInGivenExactLiquidity_ComputesTokenXAmountIn()
        public
        view
    {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;
        uint256 liquidity =
            computeInvariantUp(amountX, 0.5 ether, amountY, 0.5 ether) * 2;

        uint256 amountIn = computeAmountInGivenExactLiquidity(
            liquidity, liquidity, toWad(amountX)
        );
        console.log(amountIn);
    }
}
