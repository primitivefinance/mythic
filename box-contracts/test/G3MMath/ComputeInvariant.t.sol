// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../../src/lib/G3MPRBMath.sol";

contract ComputeInvariant is Test {
    function test_computeInvariant_ComputesInvariant() public view {
        UD60x18 reserveX = convert(750);
        UD60x18 reserveY = convert(250);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        UD60x18 invariant =
            computeInvariant(reserveX, weightX, reserveY, weightY);
        console.log(UD60x18.unwrap(invariant));
    }
}
