pragma solidity ^0.8.19;

import { SD59x18, convert } from "@prb/math/SD59x18.sol";

contract SD59x18Math {

function pow(SD59x18 x, SD59x18 y) external pure returns (SD59x18 z) {
    return x.pow(y);
}

function div(SD59x18 x, SD59x18 y) external pure returns (SD59x18 z) {
    return x.div(y);
}

function mul(SD59x18 x, SD59x18 y) external pure returns (SD59x18 z) {
    return x.mul(y);
}

}