pragma solidity ^0.8.19;

import { SD59x18, convert } from "@prb/math/SD59x18.sol";

contract SD59x18Math {

function pow(int256 x, int256 y) external pure returns (SD59x18 z) {
    SD59x18 converted_x = convert(x);
    SD59x18 converted_y = convert(y);
    return converted_x.pow(converted_y);
}

function div(int256 x, int256 y) external pure returns (SD59x18 z) {
    SD59x18 converted_x = convert(x);
    SD59x18 converted_y = convert(y);
    return converted_x.div(converted_y);
}

function mul(int256 x, int256 y) external pure returns (SD59x18 z) {
    SD59x18 converted_x = convert(x);
    SD59x18 converted_y = convert(y);
    return converted_x.mul(converted_y);
}

}