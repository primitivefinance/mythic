// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./LogNormalLib.sol";
import "../BisectionLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

/// @dev Computes reserves L given rx, S.
/// @param rx The reserve of x.
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return L The reserve L computed as L(x, s) = K * L_x(x, S) * Gaussian.cdf[d2(S, K, sigma, tau)]
function computeLGivenX(
    uint256 rx,
    uint256 S,
    LogNormParameters memory params
) pure returns (uint256 L) {
    int256 d1 = computeD1({ S: S, params: params });
    int256 cdf = Gaussian.cdf(d1);
    uint256 unsignedCdf = toUint(cdf);

    L = rx.divWadUp(ONE - unsignedCdf);
}

/// @dev Computes reserves y given L(x, S).
/// @return ry The reserve y computed as y(x, s) = K * L_x(x, S) * cdf[d2(S, K, sigma, tau)]
function computeYGivenL(
    uint256 L,
    uint256 S,
    LogNormParameters memory params
) pure returns (uint256 ry) {
    int256 d2 = computeD2(S, params);
    int256 cdf = Gaussian.cdf(d2);
    uint256 unsignedCdf = toUint(cdf);

    ry = params.strike.mulWadUp(L).mulWadUp(unsignedCdf);
}

/// @dev Computes reserves x given L(y, S).
/// @return rx The reserve x computed as x(y, s) = L_y(y, S) * (WAD - cdf[d1(S, K, sigma, tau)])
function computeXGivenL(
    uint256 L,
    uint256 S,
    LogNormParameters memory params
) pure returns (uint256 rx) {
    int256 d1 = computeD1(S, params);
    int256 cdf = Gaussian.cdf(d1);
    uint256 unsignedCdf = toUint(cdf);
    rx = L.mulWadUp(ONE - unsignedCdf);
}

/// @dev Computes the d1 parameter for the Black-Scholes formula.
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return d1 = (ln(S/K) + tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD1(
    uint256 S,
    LogNormParameters memory params
) pure returns (int256 d1) {
    (uint256 K, uint256 sigma, uint256 tau) =
        (params.strike, params.sigma, params.tau);
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaTauSquared(sigma, tau);

    d1 = (lnSDivK + int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

/// @dev Computes the d2 parameter for the Black-Scholes formula.
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return d2 = d1 - sigma * sqrt(tau), alternatively d2 = (ln(S/K) - tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD2(
    uint256 S,
    LogNormParameters memory params
) pure returns (int256 d2) {
    (uint256 K, uint256 sigma, uint256 tau) =
        (params.strike, params.sigma, params.tau);
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaTauSquared(sigma, tau);

    d2 = (lnSDivK - int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(bytes memory data, uint256 ry) pure returns (int256) {
    (uint256 rx, uint256 L,, LogNormParameters memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormParameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(bytes memory data, uint256 rx) pure returns (int256) {
    (uint256 ry, uint256 L,, LogNormParameters memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormParameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rx, uint256 ry,, LogNormParameters memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormParameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev Computes the trading function given an amountX and an initialPrice.
function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    LogNormParameters memory params
) pure returns (bytes memory) {
    uint256 L = computeLGivenX(amountX, initialPrice, params);
    uint256 ry = computeYGivenL(L, initialPrice, params);
    int256 invariant =
        tradingFunction({ rx: amountX, ry: ry, L: L, params: params });
    L = computeNextLiquidity(amountX, ry, invariant, L, params);
    return abi.encode(amountX, ry, L, params);
}

function computeAllocationGivenX(
    bool add,
    uint256 amountX,
    uint256 rx,
    uint256 L
) pure returns (uint256 nextRx, uint256 nextL) {
    uint256 liquidityPerRx = L.divWadUp(rx);
    uint256 deltaL = amountX.mulWadUp(liquidityPerRx);
    nextRx = add ? rx + amountX : rx - amountX;
    nextL = add ? L + deltaL : L - deltaL;
}

function computeAllocationGivenY(
    bool add,
    uint256 amountY,
    uint256 ry,
    uint256 L
) pure returns (uint256 nextRy, uint256 nextL) {
    uint256 liquidityPerRy = L.divWadUp(ry);
    uint256 deltaL = amountY.mulWadUp(liquidityPerRy);
    nextRy = add ? ry + amountY : ry - amountY;
    nextL = add ? L + deltaL : L - deltaL;
}

/// @dev Finds the root of the invariant given the independent variables reserveXWad and reserveYWad.
function computeNextLiquidity(
    uint256 rx,
    uint256 ry,
    int256 invariant,
    uint256 currentL,
    LogNormParameters memory params
) pure returns (uint256 nextL) {
    uint256 lower;
    uint256 upper;
    uint256 iters;
    uint256 yOverK = ry.divWadDown(params.strike);

    if (invariant < EPSILON && invariant > -(EPSILON)) {
        return currentL;
    } else if (invariant < 0) {
        upper = currentL;
        lower = rx > yOverK ? rx + 1 : yOverK + 1;
        iters = 256;
    } else {
        upper = 1e27;
        lower = currentL;
        iters = 256;
    }
    nextL = bisection(
        abi.encode(rx, ry, invariant, params),
        lower,
        upper,
        uint256(EPSILON),
        iters,
        findRootLiquidity
    );
}

/// @dev Finds the root of the invariant given the independent variable reserveXWad.
function computeNextRy(
    uint256 rx,
    uint256 L,
    int256 invariant,
    uint256 currentRy,
    LogNormParameters memory params
) pure returns (uint256 ry) {
    uint256 upper;
    uint256 lower;
    if (invariant < 0) {
        lower = currentRy;
        upper = currentRy.mulDivUp(150, 100);
    } else {
        lower = currentRy.mulDivDown(50, 100);
        upper = currentRy; // Can use `currentRy` as upper because function is monotonic and this is only invoked if swapping x in --> must satisfy currentRy > nextRy
    }
    ry = bisection(
        abi.encode(rx, L, invariant, params),
        lower,
        upper,
        uint256(EPSILON),
        256,
        findRootY
    );
}

/// @dev Finds the root of the invariant given the independent variable reserveYWad.
function computeNextRx(
    uint256 ry,
    uint256 L,
    int256 invariant,
    uint256 currentRx,
    LogNormParameters memory params
) pure returns (uint256 rx) {
    uint256 upper;
    uint256 lower;
    if (invariant < 0) {
        lower = currentRx;
        upper = currentRx.mulDivUp(150, 100);
    } else {
        lower = currentRx.mulDivDown(50, 100);
        upper = currentRx; // can use `currentRx` as upper because function is monotonic and this is only invoked if swapping y in --> must satisfy currentRx > nextRx
    }
    rx = bisection(
        abi.encode(ry, L, invariant, params),
        lower,
        upper,
        uint256(EPSILON),
        256,
        findRootX
    );
}
