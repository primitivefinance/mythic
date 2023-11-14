// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../utils/G3MTest.t.sol";

contract G3MComputeDeltaXGivenDeltaY is G3MTest {
    function test_ComputeDeltaXGivenDeltaY() public view {
        UD60x18 reserveX = convert(750 ether);
        UD60x18 reserveY = convert(250 ether);
        uint256 deltaY = 250 ether;

        uint256 deltaX = computeDeltaXGivenDeltaY(reserveX, reserveY, deltaY);
        console.log(deltaX);
    }
}
