// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "src/strategies/G3M/G3M.sol";
import "forge-std/console2.sol";
import "../BisectionLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeLGivenX(
    uint256 x,
    uint256 S,
    G3M.G3MParams memory params
) pure returns (uint256) {
    int256 a = int256(params.wY.divWadUp(params.wX).mulWadUp(S));
    int256 b = a.powWad(int256(params.wY));
    return x.mulWadUp(uint256(b));
}

function computeLGivenY(
    uint256 y,
    uint256 S,
    G3M.G3MParams memory params
) pure returns (uint256) {
    return y.mulWadUp(params.wX).divWadUp(params.wY.mulWadUp(S));
}

function computeXGivenL(
    uint256 L,
    uint256 S,
    G3M.G3MParams memory params
) pure returns (uint256) {
    return params.wX.mulWadUp(L).divWadUp(params.wY.mulWadUp(S));
}

function computeYGivenL(
    uint256 L,
    uint256 S,
    G3M.G3MParams memory params
) pure returns (uint256) {
    return params.wY.mulWadUp(L).divWadUp(params.wX.mulWadUp(S));
}

function computeY(
    uint256 x,
    uint256 S,
    G3M.G3MParams memory params
) pure returns (uint256) {
    return params.wY.divWadDown(params.wX).mulWadDown(S).mulWadDown(x);
}

function computeX(
    uint256 y,
    uint256 S,
    G3M.G3MParams memory params
) pure returns (uint256) {
    return params.wX.divWadDown(params.wY.mulWadDown(S)).mulWadDown(y);
}

function computeL(
    uint256 x,
    uint256 y,
    G3M.G3MParams memory params
) pure returns (uint256) {
    uint256 a = uint256(int256(x).powWad(int256(params.wX)));
    uint256 b = uint256(int256(y).powWad(int256(params.wY)));

    return a.mulWadUp(b);
}

function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    G3M.G3MParams memory params
) pure returns (bytes memory) {
    uint256 rY = computeY(amountX, initialPrice, params);
    uint256 L = computeL(amountX, rY, params);

    int256 invariant =
        G3MLib.tradingFunction({ rX: amountX, rY: rY, L: L, params: params });

    L = computeNextLiquidity(amountX, rY, invariant, L, params);

    return
        abi.encode(amountX, rY, L, params.wX, params.swapFee, params.controller);
}

/// @dev Finds the root of the swapConstant given the independent variable rX.
function computeNextRy(
    uint256 rX,
    uint256 liquidity,
    G3M.G3MParams memory params
) pure returns (uint256 rY) {
    rY = uint256(
        int256(
            liquidity.divWadUp(uint256(int256(rX).powWad(int256(params.wX))))
        ).powWad(int256(ONE.divWadUp(params.wY)))
    );
}

/// @dev Finds the root of the swapConstant given the independent variable rY.
function computeNextRx(
    uint256 rY,
    uint256 liquidity,
    G3M.G3MParams memory params
) pure returns (uint256 rX) {
    rX = uint256(
        int256(
            liquidity.divWadUp(uint256(int256(rY).powWad(int256(params.wY))))
        ).powWad(int256(ONE.divWadUp(params.wX)))
    );
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePrice(
    uint256 rX,
    uint256 rY,
    G3M.G3MParams memory params
) pure returns (uint256 price) {
    uint256 n = rY.divWadDown(params.wY);
    uint256 d = rX.divWadDown(params.wX);
    price = n.divWadDown(d);
}
/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.

function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rX, uint256 rY,, G3M.G3MParams memory params) =
        abi.decode(data, (uint256, uint256, int256, G3M.G3MParams));
    return G3MLib.tradingFunction({ rX: rX, rY: rY, L: L, params: params });
}

function findRootLower(bytes memory data, uint256 v) pure returns (int256) {
    (uint256 S, uint256 rX, uint256 rY, uint256 L, G3M.G3MParams memory params)
    = abi.decode(data, (uint256, uint256, uint256, uint256, G3M.G3MParams));
    return diffLower({ S: S, rX: rX, rY: rY, L: L, v: v, params: params });
}

function findRootRaise(bytes memory data, uint256 v) pure returns (int256) {
    (uint256 S, uint256 rX, uint256 rY, uint256 L, G3M.G3MParams memory params)
    = abi.decode(data, (uint256, uint256, uint256, uint256, G3M.G3MParams));
    return diffRaise({ S: S, rX: rX, rY: rY, L: L, v: v, params: params });
}

function diffLower(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 v,
    G3M.G3MParams memory params
) pure returns (int256) {
    (int256 wx, int256 wy) = (int256(params.wX), int256(params.wY));
    uint256 gamma = ONE - params.swapFee;
    int256 yOverX = int256(rY.divWadDown(rX));
    uint256 yOverXPowWx = uint256(yOverX.powWad(wx));
    uint256 yOverXPowWy = uint256(yOverX.powWad(wy));

    uint256 numerator;
    {
        uint256 first =
            L.mulWadDown(params.wX).mulWadDown(rX).mulWadDown(yOverXPowWx);
        uint256 second = (v - v.mulWadDown(params.wX) + rX).mulWadDown(rY)
            .mulWadDown(ONE - gamma);
        uint256 third = uint256(int256(v + rX).powWad(-wx));
        uint256 fourth = L + v.mulWadDown(yOverXPowWy).mulWadDown(ONE - gamma);
        numerator = (first - second).mulWadDown(
            uint256(
                int256(third.mulWadDown(fourth)).powWad(
                    int256(ONE.divWadDown(ONE - params.wX))
                )
            )
        );
    }

    uint256 denominator;
    {
        uint256 dFirst = ONE - params.wX;
        uint256 dSecond = v + rX;
        uint256 dThird = L.mulWadDown(rX).mulWadDown(uint256(yOverXPowWx));
        uint256 dFourth = v.mulWadDown(rY).mulWadDown(ONE - gamma);
        denominator = dFirst.mulWadDown(dSecond).mulWadDown(dThird + dFourth);
    }

    int256 result = -int256(S) + int256(numerator.divWadDown(denominator));
    return result;
}

function diffRaise(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 v,
    G3M.G3MParams memory params
) pure returns (int256) {
    int256 wx = int256(params.wX);
    uint256 gamma = ONE - params.swapFee;
    uint256 xOverYPowWx = uint256(int256(rX.divWadDown(rY)).powWad(wx));
    uint256 vPlusYPow = uint256(int256(v + rY).powWad(-int256(ONE) + wx));
    uint256 vTimesXOverYPowWx = v.mulWadDown(xOverYPowWx);
    uint256 lPlusVTimesXOverYPowWx =
        L + vTimesXOverYPowWx.mulWadDown(ONE - gamma);

    int256 numerator;
    {
        uint256 first = params.wX.mulWadDown(v + rY);
        console2.log("first", first);
        uint256 second = lPlusVTimesXOverYPowWx;
        console2.log("second", second);
        uint256 third = uint256(
            int256((uint256(vPlusYPow.mulWadDown(lPlusVTimesXOverYPowWx))))
                .powWad(int256(ONE.divWadDown(params.wX)))
        );
        console2.log("third", third);
        uint256 fourth = L.mulWadDown(ONE - params.wX);
        console2.log("fourth", fourth);
        uint256 fifth = xOverYPowWx.mulWadDown(v.mulWadDown(params.wX) + rY)
            .mulWadDown(ONE - gamma);
        console2.log("fifth", fifth);

        numerator = int256(first.mulWadDown(second))
            - int256(S.mulWadDown(third).mulWadDown(fourth - fifth));
        console2.log("num", numerator);
    }

    uint256 denominator;
    {
        uint256 first = params.wX.mulWadDown(v + rY);
        console2.log("first", first);
        uint256 second = L + vTimesXOverYPowWx.mulWadDown(ONE - gamma);
        console2.log("second", second);
        denominator = first.mulWadDown(second);
        console2.log("den", denominator);
    }

    if (numerator > 0) {
        return -int256(uint256(numerator).divWadDown(denominator));
    } else {
        return int256(uint256(-numerator).divWadDown(denominator));
    }
}

function computeOptimalLower(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 vUpper,
    G3M.G3MParams memory params
) pure returns (uint256 v) {
    uint256 upper = vUpper;
    uint256 lower = vUpper.mulDivDown(1, 100);
    v = bisection(
        abi.encode(S, rX, rY, L, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootLower
    );
}

function computeOptimalRaise(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 vUpper,
    G3M.G3MParams memory params
) pure returns (uint256 v) {
    uint256 upper = vUpper;
    uint256 lower = vUpper.mulDivDown(1, 100);
    v = bisection(
        abi.encode(S, rX, rY, L, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootRaise
    );
}

function computeNextLiquidity(
    uint256 rX,
    uint256 rY,
    int256 invariant,
    uint256 approximatedL,
    G3M.G3MParams memory params
) pure returns (uint256 L) {
    uint256 upper = approximatedL;
    uint256 lower = approximatedL;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant = G3MLib.tradingFunction({
                rX: rX,
                rY: rY,
                L: lower,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant = G3MLib.tradingFunction({
                rX: rX,
                rY: rY,
                L: upper,
                params: params
            });
        }
    }
    L = bisection(
        abi.encode(rX, rY, computedInvariant, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootLiquidity
    );
}
