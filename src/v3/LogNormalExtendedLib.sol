// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./LogNormalLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

/// @dev Computes reserves y given L(x, S).
/// @return y(x, s) = K * L_x(x, S) * Gaussian.cdf[d2(S, K, sigma, tau)]
function computeYGivenL(
    uint256 totalLiquidity,
    uint256 priceWad,
    Parameters memory params
) pure returns (uint256) {
    int256 d2 = computeD2({ priceWad: priceWad, params: params });
    int256 cdf = Gaussian.cdf(d2);
    uint256 unsignedCdf = toUint(cdf);
    return params.strikePriceWad.mulWadUp(totalLiquidity).mulWadUp(unsignedCdf);
}

/// @dev Computes reserves x given L(y, S).
/// @return x(y, s) = L_y(y, S) * (WAD - Gaussian.cdf[d1(S, K, sigma, tau)])
function computeXGivenL(
    uint256 totalLiquidity,
    uint256 priceWad,
    Parameters memory params
) pure returns (uint256) {
    int256 d1 = computeD1({ priceWad: priceWad, params: params });
    int256 cdf = Gaussian.cdf(d1);
    uint256 unsignedCdf = toUint(cdf);
    return totalLiquidity.mulWadUp(ONE - unsignedCdf);
}

/// @param priceWad The price of X in Y, in WAD units.
/// @param params Parameters of the Log Normal distribution.
/// @return d1 (ln(price / K) + tau * sigma^2 / 2)) / (sigma * sqrt(tau))
function computeD1(
    uint256 priceWad,
    Parameters memory params
) pure returns (int256 d1) {
    // sigma * sqrt(tau)
    uint256 sigmaSqrtTauWad = computeSigmaSqrtTau({
        sigmaPercentWad: params.sigmaPercentWad,
        tauYearsWad: params.tauYearsWad
    });
    // sigma^2 / 2
    uint256 halfSigmaSquaredWad =
        computeHalfSigmaSquared({ sigmaPercentWad: params.sigmaPercentWad });
    // ln(price / K), round UP because ln(1) = 0, but ln(0) = undefined.
    int256 logPriceOverStrikeWad = FixedPointMathLib.lnWad(
        int256(priceWad.divWadUp(params.strikePriceWad))
    );
    // Round up because the division is truncation in the lnWad function.
    logPriceOverStrikeWad++;
    // (ln(price / K) + tau * sigma^2 * tau / 2))
    int256 numerator = logPriceOverStrikeWad
        + int256(halfSigmaSquaredWad.mulWadDown(params.tauYearsWad));
    // sigma * sqrt(tau)
    int256 denominator = int256(sigmaSqrtTauWad);
    // (ln(price / K) + tau * sigma^2 / 2)) / (sigma * sqrt(tau))
    d1 = mulidivUp(numerator, int256(ONE), denominator);
}

/// @param priceWad The price of X in Y, in WAD units.
/// @param params Parameters of the Log Normal distribution.
/// @return d2 = d1 - sigma * sqrt(tau), alternatively d2 = (ln(S/K) - tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD2(
    uint256 priceWad,
    Parameters memory params
) pure returns (int256 d2) {
    d2 = computeD1(priceWad, params)
        - int256(
            computeSigmaSqrtTau({
                sigmaPercentWad: params.sigmaPercentWad,
                tauYearsWad: params.tauYearsWad
            })
        );
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(
    bytes memory data,
    uint256 reserveYWad
) pure returns (int256) {
    (
        uint256 rx,
        uint256 liquidity,
        int256 swapConstant,
        Parameters memory params
    ) = abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({
        reserveXWad: rx,
        reserveYWad: reserveYWad,
        totalLiquidity: liquidity,
        params: params
    }) - swapConstant;
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(
    bytes memory data,
    uint256 reserveXWad
) pure returns (int256) {
    (
        uint256 ry,
        uint256 liquidity,
        int256 swapConstant,
        Parameters memory params
    ) = abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({
        reserveXWad: reserveXWad,
        reserveYWad: ry,
        totalLiquidity: liquidity,
        params: params
    }) - swapConstant;
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 liquidity
) pure returns (int256) {
    (uint256 rx, uint256 ry,, Parameters memory params) =
        abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({
        reserveXWad: rx,
        reserveYWad: ry,
        totalLiquidity: liquidity,
        params: params
    });
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
