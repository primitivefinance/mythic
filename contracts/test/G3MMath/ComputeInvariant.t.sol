// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../utils/G3MTest.t.sol";

contract G3MComputeInvariant is G3MTest {
    function test_computeInvariant_ComputesInvariant() public view {
        UD60x18 reserveX = convert(750 ether);
        UD60x18 reserveY = convert(250 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        UD60x18 invariant =
            computeInvariant(reserveX, weightX, reserveY, weightY);
        console.log(UD60x18.unwrap(invariant));
    }
}
