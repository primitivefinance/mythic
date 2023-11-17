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

function computeSigmaSqrtTau(uint256 sigma, uint256 tau) pure returns (uint256 sigmaSqrtTau) {
    uint256 sqrtTau = FixedPointMathLib.sqrt(tau) * 10 ** 9;
    sigmaSqrtTau = FixedPointMathLib.mulWadDown(
        sigma, sqrtTau
    );
}

function computeHalfSigmaPower2Tau(uint256 sigma, uint256 tau) pure returns (uint256 halfSigmaPower2Tau) {
    uint256 innerTerm = FixedPointMathLib.mulWadDown(
        uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO))), tau
    );

    halfSigmaPower2Tau = FixedPointMathLib.mulWadDown(
        HALF,
        innerTerm
    );
}

function computeLnSDivK(
    uint256 S,
    uint256 K
) pure returns (int256 lnSDivK) {
    lnSDivK = FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
}

function computeD1(
    uint256 S,
    uint256 K,
    uint256 sigma,
    uint256 tau // time to expiry
) pure returns (int256 d1) {
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);

    uint256 halfSigmaPowTwoTau = computeHalfSigmaPower2Tau(sigma, tau);

    d1 = (lnSDivK + int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

function computeD2(
    uint256 S,
    uint256 K,
    uint256 sigma,
    uint256 tau // time to expiry
) pure returns (int256 d2) {
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaPower2Tau(sigma, tau);

    d2 = (lnSDivK - int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

function computeLGivenX(
    uint256 x,
    uint256 S,
    uint256 K,
    uint256 sigma,
    uint256 tau // time to expiry
) pure returns (uint256 L) {
    int256 denominator = int256(ONE) - Gaussian.cdf(computeD1(S, K, sigma, tau));
    
    L = FixedPointMathLib.divWadUp(x, uint256(denominator));
}

function computeLGivenY(
    uint256 y,
    uint256 S,
    uint256 K,
    uint256 sigma,
    uint256 tau // time to expiry
) pure returns (uint256 L) {
    uint256 denominator = FixedPointMathLib.mulWadUp(K, uint256(Gaussian.cdf(computeD2(S, K, sigma, tau))));

    L = FixedPointMathLib.divWadUp(y, denominator);
}

function computeXGivenL(
    uint256 L,
    uint256 S,
    uint256 K,
    uint256 sigma,
    uint256 tau // time to expiry
) pure returns (uint256 x) {
    int256 cdf = Gaussian.cdf(computeD1(S, K, sigma, tau));
    x = FixedPointMathLib.mulWadUp(L, uint256(int256(ONE) - cdf));
}

function computeYGivenL(
    uint256 L,
    uint256 S,
    uint256 K,
    uint256 sigma,
    uint256 tau // time to expiry
) pure returns (uint256 y) {
    int256 cdf = Gaussian.cdf(computeD2(S, K, sigma, tau));
    y = FixedPointMathLib.mulWadUp(
        K, FixedPointMathLib.mulWadUp(L, uint256(cdf))
    );
}

// p = Ke^{\Phi^{-1}(1-R_1)}\sigma\sqrt{T - \frac{1}{2}\sigma^2 \tau}.
function computeSpotPrice(
    uint256 x,
    uint256 L,
    uint256 K,
    uint256 sigma,
    uint256 tau
) pure returns (uint256 spotPrice) {
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    uint256 halfSigmaPower2Tau = computeHalfSigmaPower2Tau(sigma, tau);
    uint256 R1 = FixedPointMathLib.divWadDown(x, L);

    spotPrice = FixedPointMathLib.mulWadUp(
        K,
        uint256(
            FixedPointMathLib.expWad(
                Gaussian.ppf(int256(ONE - R1)) * int256(sigmaSqrtTau) / int256(ONE) - int256(halfSigmaPower2Tau)
            )
        )
    );
}

// The formula for computing the change in y (deltaY) is as follows:
// deltaY = K(L + deltaL) * Phi(-sigma - Phi^-1((x + deltaX) / (L + deltaL))) - y 
// where Phi is the cumulative distribution function of the standard normal distribution,
// Phi^-1 is the inverse of the Phi function,
// sigma is the volatility,
// L is the liquidity,
// deltaL is the change in liquidity,
// K is the strike price,
// x is the reserve x,
// deltaX is the x amount in,
// y is the reserve y,
// deltaY is the y amount out.
function computeOutputYGivenX(
    uint256 x, //reserve x
    uint256 y, // reserve y
    uint256 deltaX, // x amount in
    uint256 L, // liquidity
    uint256 deltaL, // change in liquidity
    uint256 K, // strike price
    uint256 sigma, // volatility
    uint256 tau // time to expiry
) pure returns (int256 deltaY) {
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    uint256 KL = FixedPointMathLib.mulWadDown(K, L + deltaL);

    int256 cdf = Gaussian.cdf(
        -int256(sigmaSqrtTau)
            - Gaussian.ppf(
                int256(FixedPointMathLib.divWadDown(x + deltaX, L + deltaL))
            )
    );

    deltaY = int256(FixedPointMathLib.mulWadDown(KL, uint256(cdf))) - int256(y);
}

// The formula for computing the change in x (deltaX) is as follows:
// deltaX = (L + deltaL) * Phi(-sigma - Phi^-1((y + deltaY) / (K * (L + deltaL)))) - x 
// where Phi is the cumulative distribution function of the standard normal distribution,
// Phi^-1 is the inverse of the Phi function,
// sigma is the volatility,
// L is the liquidity,
// deltaL is the change in liquidity,
// K is the strike price,
// y is the reserve y,
// deltaY is the y amount in,
// x is the reserve x,
// deltaX is the x amount in.
function computeOutputXGivenY(
    uint256 x, //reserve x
    uint256 y, // reserve y
    uint256 deltaY, // y amount in
    uint256 L, // liquidity
    uint256 deltaL, // change in liquidity
    uint256 K, // strike price
    uint256 sigma, // volatility
    uint256 tau // time to expiry
) pure returns (int256 deltaX) {
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    uint256 KL = FixedPointMathLib.mulWadDown(K, L + deltaL);

    int256 cdf = Gaussian.cdf(
        -int256(sigmaSqrtTau)
            - Gaussian.ppf(
                int256(FixedPointMathLib.divWadDown(y + deltaY, KL))
            )
    );

    deltaX = int256(FixedPointMathLib.mulWadDown(L + deltaL, uint256(cdf))) - int256(x);
}

function computeInvariant(
    uint256 reserveX,
    uint256 liquidity,
    uint256 reserveY,
    uint256 strikePrice
) pure returns (int256 invariant) {
    invariant = Gaussian.ppf(int256(reserveX / liquidity))
        + Gaussian.ppf(
            int256(reserveY / FixedPointMathLib.mulWadDown(liquidity, strikePrice))
        );
}
