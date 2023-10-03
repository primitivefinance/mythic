// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/lib/G3MPRBMath.sol";

contract ComputeAmountInGivenExactLiquidity is Test {
    function test_computeAmountInGivenExactLiquidity_ComputesTokenXAmountIn()
        public
    {
        UD60x18 reserveX = convert(750 ether);
        UD60x18 reserveY = convert(250 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        UD60x18 liquidity =
            computeInvariant(reserveX, weightX, reserveY, weightY) * ud(2);

        uint256 amountIn =
            computeAmountInGivenExactLiquidity(liquidity, liquidity, reserveX);
        assertEq(amountIn, convert(reserveX));
    }

    function test_computeAmountInGivenExactLiquidity_ComputesTokenYAmountIn()
        public
    {
        UD60x18 reserveX = convert(750 ether);
        UD60x18 reserveY = convert(250 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        UD60x18 liquidity =
            computeInvariant(reserveX, weightX, reserveY, weightY) * ud(2);

        uint256 amountIn =
            computeAmountInGivenExactLiquidity(liquidity, liquidity, reserveY);
        assertEq(amountIn, convert(reserveY));
    }
}
