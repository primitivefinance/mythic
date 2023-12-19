// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./LogNormalLib.sol";
import "../BisectionLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeLGivenX(
    uint256 rx,
    uint256 S,
    Parameters memory params
) pure returns (uint256 L) {
    int256 denominator = int256(ONE) - Gaussian.cdf(computeD1(S, params));

    L = FixedPointMathLib.divWadUp(rx, uint256(denominator));
}

/// @dev Computes reserves y given L(x, S).
/// @return y(x, s) = K * L_x(x, S) * Gaussian.cdf[d2(S, K, sigma, tau)]
function computeYGivenL(
    uint256 L,
    uint256 S,
    Parameters memory params
) pure returns (uint256) {
    int256 d2 = computeD2(S, params);
    int256 cdf = Gaussian.cdf(d2);
    uint256 unsignedCdf = toUint(cdf);
    return params.strike.mulWadUp(L).mulWadUp(unsignedCdf);
}

/// @dev Computes reserves x given L(y, S).
/// @return x(y, s) = L_y(y, S) * (WAD - Gaussian.cdf[d1(S, K, sigma, tau)])
function computeXGivenL(
    uint256 L,
    uint256 S,
    Parameters memory params
) pure returns (uint256) {
    int256 d1 = computeD1(S, params);
    int256 cdf = Gaussian.cdf(d1);
    uint256 unsignedCdf = toUint(cdf);
    return L.mulWadUp(ONE - unsignedCdf);
}

function computeD1(
    uint256 S,
    Parameters memory params
) pure returns (int256 d1) {
    (uint256 K, uint256 sigma, uint256 tau) =
        (params.strike, params.sigma, params.tau);
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaPower2Tau(sigma, tau);

    d1 = (lnSDivK + int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

/// @param S The price of X in Y, in WAD units.
/// @param params Parameters of the Log Normal distribution.
/// @return d2 = d1 - sigma * sqrt(tau), alternatively d2 = (ln(S/K) - tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD2(
    uint256 S,
    Parameters memory params
) pure returns (int256 d2) {
    (uint256 K, uint256 sigma, uint256 tau) =
        (params.strike, params.sigma, params.tau);
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaPower2Tau(sigma, tau);

    d2 = (lnSDivK - int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(bytes memory data, uint256 ry) pure returns (int256) {
    (uint256 rx, uint256 L,, Parameters memory params) =
        abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(bytes memory data, uint256 rx) pure returns (int256) {
    (uint256 ry, uint256 L,, Parameters memory params) =
        abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rx, uint256 ry,, Parameters memory params) =
        abi.decode(data, (uint256, uint256, int256, Parameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    Parameters memory params
) pure returns (bytes memory) {
    uint256 L = computeLGivenX(amountX, initialPrice, params);
    uint256 ry = computeYGivenL(L, initialPrice, params);
    int256 swapConstant =
        tradingFunction({ rx: amountX, ry: ry, L: L, params: params });
    L = computeNextLiquidity(amountX, ry, swapConstant, L, params);
    return abi.encode(amountX, ry, L, params);
}
/// @dev Finds the root of the swapConstant given the independent variable liquidity.

function computeNextLiquidity(
    uint256 reserveXWad,
    uint256 reserveYWad,
    int256 swapConstant,
    uint256 currentLiquidity,
    Parameters memory params
) pure returns (uint256 L) {
    uint256 lower;
    uint256 upper;
    uint256 iters;
    uint256 yOverK = reserveYWad.divWadDown(params.strike);

    if (swapConstant < EPSILON && swapConstant > -(EPSILON)) {
        return currentLiquidity;
    } else if (swapConstant < 0) {
        upper = currentLiquidity;
        lower = reserveXWad > yOverK ? reserveXWad + 1 : yOverK + 1;
        iters = 128;
    } else {
        upper = 1e27;
        lower = currentLiquidity;
        iters = 128;
    }
    L = bisection(
        abi.encode(reserveXWad, reserveYWad, swapConstant, params),
        lower,
        upper,
        uint256(EPSILON),
        iters,
        findRootLiquidity
    );
}

/// @dev Finds the root of the swapConstant given the independent variable reserveXWad.
function computeNextRy(
    uint256 reserveXWad,
    uint256 liquidity,
    int256 swapConstant,
    Parameters memory params
) pure returns (uint256 ry) {
    uint256 lower = 10;
    uint256 upper = liquidity.mulWadUp(params.strike) - 10;
    ry = bisection(
        abi.encode(reserveXWad, liquidity, swapConstant, params),
        lower,
        upper,
        uint256(EPSILON),
        256,
        findRootY
    );
}

/// @dev Finds the root of the swapConstant given the independent variable reserveYWad.
function computeNextRx(
    uint256 reserveYWad,
    uint256 liquidity,
    int256 swapConstant,
    Parameters memory params
) pure returns (uint256 rx) {
    uint256 lower = 10;
    uint256 upper = liquidity - 10; // max x = 1 - x / l, so l - x
    rx = bisection(
        abi.encode(reserveYWad, liquidity, swapConstant, params),
        lower,
        upper,
        uint256(EPSILON),
        256,
        findRootX
    );
}
