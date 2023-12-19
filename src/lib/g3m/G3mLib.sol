pragma solidity ^0.8.13;

import "forge-std/console2.sol";
import "solmate/utils/FixedPointMathLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

/// @dev Taking the square root of a WAD value returns a value with units of 1E9.
/// Multiplying the result by SQRT_WAD will normalize it back to WAD units.
uint256 constant SQRT_WAD = 1e9;
uint256 constant TWO = 2e18;
uint256 constant HALF = 0.5e18;
uint256 constant ONE = 1e18;
uint256 constant INFINITY_IS_NOT_REAL = type(uint256).max;
uint256 constant ZERO = 0;

int256 constant EPSILON = 10;

/// @dev Parameterization of the Log Normal curve.
struct Parameters {
    uint256 wx;
    uint256 wy;
}

/// @dev Structure to hold reserve information
struct Reserves {
    uint256 rx;
    uint256 ry;
    uint256 L;
}

function tradingFunction(
    uint256 rx,
    uint256 ry,
    uint256 L,
    Parameters memory params
) pure returns (int256) {
    int256 a = FixedPointMathLib.powWad(int256(rx), int256(params.wx));
    int256 b = FixedPointMathLib.powWad(int256(ry), int256(params.wy));

    return a + b - int256(L);
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePrice(
    uint256 rx,
    uint256 ry,
    Parameters memory params
) pure returns (uint256 price) {
    uint256 n = ry.divWadDown(params.wy);
    uint256 d = rx.divWadDown(params.wx);
    price = n.divWadDown(d);
}
