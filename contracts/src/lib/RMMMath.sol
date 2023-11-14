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

function computeLGivenY(
    uint256 y,
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
        Gaussian.cdf((lnSDivK - int256(halfSigmaPowTwo)) * 1e18 / int256(sigma));
    L = FixedPointMathLib.divWadUp(
        y, FixedPointMathLib.mulWadUp(K, uint256(cdf))
    );
}

function computeXGivenL(
    uint256 L,
    uint256 S,
    uint256 K,
    uint256 sigma
) pure returns (uint256 x) {
    int256 lnSDivK =
        FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
    uint256 halfSigmaPowTwo = FixedPointMathLib.mulWadUp(
        HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO)))
    );
    int256 cdf =
        Gaussian.cdf((lnSDivK + int256(halfSigmaPowTwo)) * 1e18 / int256(sigma));
    x = FixedPointMathLib.mulWadUp(L, uint256(int256(ONE) - cdf));
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
    int256 minus = lnSDivK - int256(halfSigmaPowTwo);
    int256 div = minus * 1e18 / int256(sigma);
    int256 cdf = Gaussian.cdf(div);
    y = FixedPointMathLib.mulWadUp(
        K, FixedPointMathLib.mulWadUp(L, uint256(cdf))
    );
}

function computeSpotPrice(
    uint256 x,
    uint256 L,
    uint256 K,
    uint256 sigma,
    uint256 tau
) pure returns (uint256) {
    uint256 halfSigmaPower2Tau = FixedPointMathLib.mulWadDown(
        HALF,
        FixedPointMathLib.mulWadDown(
            uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO))), tau
        )
    );

    uint256 sigmaSqrtTau = FixedPointMathLib.mulWadDown(
        uint256(sigma), FixedPointMathLib.sqrt(tau)
    ) * 10 ** 9;

    uint256 R1 = FixedPointMathLib.divWadDown(x, L);

    return FixedPointMathLib.mulWadUp(
        K,
        uint256(
            FixedPointMathLib.expWad(
                int256(
                    FixedPointMathLib.mulWadDown(
                        uint256(Gaussian.ppf(int256(ONE - R1))), sigmaSqrtTau
                    ) - halfSigmaPower2Tau
                )
            )
        )
    );
}

function computeOutputYGivenX(
    uint256 x,
    uint256 deltaX,
    uint256 y,
    uint256 deltaY,
    uint256 L,
    uint256 deltaL,
    uint256 K,
    uint256 sigma
) pure returns (int256) {
    uint256 KL = FixedPointMathLib.mulWadDown(K, L + deltaL);

    int256 cdf = Gaussian.cdf(
        -int256(sigma)
            - Gaussian.ppf(
                int256(FixedPointMathLib.divWadDown(x + deltaX, L + deltaL))
            )
    );

    return int256(FixedPointMathLib.mulWadDown(KL, uint256(cdf))) - int256(y)
        - int256(deltaY);
}

function computeInvariant(
    uint256 reserveX,
    uint256 liquidity,
    uint256 reserveY,
    uint256 strikePrice
) pure returns (int256) {
    return Gaussian.ppf(int256(reserveX / liquidity))
        + Gaussian.ppf(
            int256(reserveY / FixedPointMathLib.mulWadDown(liquidity, strikePrice))
        );
}
