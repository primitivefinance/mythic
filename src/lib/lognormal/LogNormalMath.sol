pragma solidity ^0.8.13;

import "solstat/Gaussian.sol";

/// @dev Taking the square root of a WAD value returns a value with units of 1E9.
/// Multiplying the result by SQRT_WAD will normalize it back to WAD units.
uint256 constant SQRT_WAD = 1e9;
uint256 constant TWO = 2e18;
uint256 constant HALF = 0.5e18;
uint256 constant ONE = 1e18;
uint256 constant INFINITY_IS_NOT_REAL = type(uint256).max;
uint256 constant ZERO = 0;

/// @dev the swap constant should never fall outside of range [-EPSILON, EPSILON]
int256 constant EPSILON = 10;

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeLnSDivK(uint256 S, uint256 K) pure returns (int256 lnSDivK) {
    lnSDivK = FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
}

function computeSigmaSqrtTau(
    uint256 sigma,
    uint256 tau
) pure returns (uint256 sigmaSqrtTau) {
    uint256 sqrtTau = FixedPointMathLib.sqrt(tau) * 10 ** 9;
    sigmaSqrtTau = FixedPointMathLib.mulWadDown(sigma, sqrtTau);
}

function computeHalfSigmaPower2Tau(
    uint256 sigma,
    uint256 tau
) pure returns (uint256 halfSigmaPower2Tau) {
    uint256 innerTerm = FixedPointMathLib.mulWadDown(
        uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO))), tau
    );

    halfSigmaPower2Tau = FixedPointMathLib.mulWadDown(HALF, innerTerm);
}

/// @dev Signed mul div, rounding up if the modulo quotient is non-zero.
function mulidivUp(
    int256 x,
    int256 y,
    int256 denominator
) pure returns (int256 z) {
    z = mulidiv(x, y, denominator);
    if ((x * y) % denominator != 0) {
        require(z < type(int256).max, "mulidivUp overflow");
        z += 1;
    }
}

/// @notice Mul div signed integers.
/// @dev From Solmate, but not in assembly.
function mulidiv(
    int256 x,
    int256 y,
    int256 denominator
) pure returns (int256 z) {
    unchecked {
        z = x * y;
        require(denominator != 0 && (x == 0 || z / x == y), "mulidiv invalid");
        z = z / denominator;
    }
}
