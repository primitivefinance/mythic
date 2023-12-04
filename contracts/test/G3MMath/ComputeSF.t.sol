// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../utils/G3MTest.t.sol";

contract ComputeSF is G3MTest {
    function test_computeSF() public view {
        UD60x18 t = UD60x18.wrap(0.5 ether);
        UD60x18 weightX = computeISF(t);
        console.log(UD60x18.unwrap(weightX));
    }

    function test_computeISF() public view {
        UD60x18 w0 = UD60x18.wrap(0.05 ether);
        UD60x18 w1 = UD60x18.wrap(0.9 ether);
        UD60x18 t = UD60x18.wrap(0.5 ether);
        UD60x18 fw0 = computeISF(w0);
        UD60x18 fw1 = computeISF(w1);
        UD60x18 weightX = computeSF(t, fw1 - fw0, fw0);
        console.log(UD60x18.unwrap(weightX));
    }
}
