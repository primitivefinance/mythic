// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../G3MTest.t.sol";

contract ComputeAmountOutGivenExactLiquidity is G3MTest {
    function test_computeAmountOutGivenExactLiquidity_ComputesTokenXAmountOut()
        public
    {
        UD60x18 reserveX = convert(750 ether);
        UD60x18 reserveY = convert(250 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        UD60x18 liquidity =
            computeInvariant(reserveX, weightX, reserveY, weightY) * ud(2);

        uint256 amountOut =
            computeAmountOutGivenExactLiquidity(liquidity, liquidity, reserveX);
        assertEq(amountOut, convert(reserveX));
    }

    function test_computeAmountOutGivenExactLiquidity_ComputesTokenYAmountOut()
        public
    {
        UD60x18 reserveX = convert(750 ether);
        UD60x18 reserveY = convert(250 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        UD60x18 liquidity =
            computeInvariant(reserveX, weightX, reserveY, weightY) * ud(2);

        uint256 amountOut =
            computeAmountOutGivenExactLiquidity(liquidity, liquidity, reserveY);
        assertEq(amountOut, convert(reserveY));
    }
}
