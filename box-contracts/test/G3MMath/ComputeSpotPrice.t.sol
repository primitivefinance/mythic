// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/lib/G3MPRBMath.sol";

contract ComputeSpotPrice is Test {
    function test_computeSpotPrice_ComputesSpotPrice() public {
        UD60x18 reserveX = convert(833_000 ether);
        UD60x18 reserveY = convert(500 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        uint256 spotPrice =
            computeSpotPrice(reserveX, weightX, reserveY, weightY);
        assertEq(spotPrice, 1666 ether);
    }
}
