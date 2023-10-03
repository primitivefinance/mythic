// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import { UD60x18 } from "@prb/math/UD60x18.sol";

contract G3MTest is Test {
    function assertEq(UD60x18 a, UD60x18 b) internal {
        assertEq(UD60x18.unwrap(a), UD60x18.unwrap(b));
    }

    function assertEq(UD60x18 a, UD60x18 b, string memory err) internal {
        assertEq(UD60x18.unwrap(a), UD60x18.unwrap(b), err);
    }
}
