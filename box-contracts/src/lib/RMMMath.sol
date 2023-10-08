// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solstat/Gaussian.sol";

uint256 constant ONE = 1e18;

uint256 constant HALF = 0.5e18;

uint256 constant TWO = 2e18;

function toWad(uint256 a) pure returns (uint256) {
    return a * ONE;
}

function fromWad(uint256 a) pure returns (uint256) {
    return a / ONE;
}

function computeLGivenX(
    uint256 x,
    uint256 S,
    uint256 K,
    uint256 sigma
) pure returns (uint256 L) {
    int256 lnSDivK =
        FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
    uint256 halfSigmaPowTwo = FixedPointMathLib.mulWadUp(
        HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO)))
    );
    int256 cdf =
        Gaussian.cdf((lnSDivK + int256(halfSigmaPowTwo)) * 1e18 / int256(sigma));
    int256 denominator = int256(1e18) - cdf;
    L = FixedPointMathLib.divWadUp(x, uint256(denominator));
}

function computeYGivenL(
    uint256 L,
    uint256 S,
    uint256 K,
    uint256 sigma
) pure returns (uint256 y) {
    int256 lnSDivK =
        FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
    uint256 halfSigmaPowTwo = FixedPointMathLib.mulWadUp(
        HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO)))
    );
    int256 cdf =
        Gaussian.cdf((lnSDivK - int256(halfSigmaPowTwo)) * 1e18 / int256(sigma));
    y = FixedPointMathLib.mulWadUp(
        K, FixedPointMathLib.mulWadUp(L, uint256(cdf))
    );
}
