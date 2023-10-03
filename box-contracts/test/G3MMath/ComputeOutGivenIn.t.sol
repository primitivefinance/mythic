// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/lib/G3MPRBMath.sol";

contract ComputeOutGivenIn is Test {
    function test_computeOutGivenIn_ComputesAmountOut() public view {
        uint256 amountIn = 50 ether;
        UD60x18 reserveX = convert(750 ether);
        UD60x18 reserveY = convert(250 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        uint256 amountOut =
            computeOutGivenIn(amountIn, reserveX, weightX, reserveY, weightY);
        console.log(amountOut);
    }
}
