// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solstat/Gaussian.sol";

uint256 constant ONE = 1e18;

uint256 constant HALF = 0.5e18;

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
    uint256 lnSK = uint256(
        FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)))
    );
    uint256 halfSigmaPow = FixedPointMathLib.mulWadUp(
        HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(2)))
    );

    L = FixedPointMathLib.divWadUp(
        toWad(x),
        ONE
            - uint256(
                Gaussian.cdf(
                    int256(FixedPointMathLib.divWadUp(lnSK + halfSigmaPow, sigma))
                )
            )
    );
}
