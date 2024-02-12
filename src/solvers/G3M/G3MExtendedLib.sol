// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "src/strategies/G3M/G3M.sol";
import "../BisectionLib.sol";
import "forge-std/console2.sol";

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
/// @dev Finds the optimal x input for liquidity adjustment using bisection method.

// function findOptimalX(
//     uint256 L,
//     uint256 w,
//     uint256 x,
//     uint256 y,
//     uint256 v,
//     uint256 gamma
// ) pure returns (uint256 S) {
//     int256 iw = int256(w);
//     int256 ix = int256(x);
//     int256 iy = int256(y);
//     int256 iv = int256(v);
//     int256 igamma = int256(gamma);
//     int256 one = int256(1 ether);

//     int256 numerator = (
//         L.mulWadUp(w).mulWadUp(ix).mulWadUp(int256(y.divWadUp(x)).powWad(iw))
//             + (iv - iv.mulWadUp(w) + ix).mulWadUp(iy).mulWadUp(one - igamma)
//     );
//     int256 denominator = (one - iw).mulWadUp(iv + ix).mulWadUp(
//         -L.mulWadUp(ix).mulWadUp(iy.divWadUp(ix).powWad(iw))
//             + iv.mulWadUp(iy).mulWadUp(one - igamma)
//     );
//     int256 exponent = one.divWadUp(one - iw);
//     int256 base = (iv + ix).powWad(-iw).mulWadUp(
//         L - iv.mulWadUp(iy.divWadUp(ix).powWad(one - iw)).mulWadUp(one - igamma)
//     );

//     S = uint256(numerator.mulWadUp(base.powWad(exponent)).divWadUp(denominator));
// }

// function find_optimal_x_input(
//     uint256 S,
//     uint256 L,
//     uint256 y,
//     uint256 v,
//     uint256 gamma,
//     uint256 upperBound,
//     G3M.G3MParams memory params
// ) pure returns (uint256 x) {
//     uint256 w = params.wX;
//     int256 iw = int256(w);
//     int256 iy = int256(y);
//     int256 iL = int256(L);
//     int256 iv = int256(v);
//     int256 negGamma = int256(gamma) - int256(ONE);
//     uint256 lowerBound = 1;

//     int256 vPowW = iv.powWad(iw);
//     int256 yOverXPowW = int256(y.divWadDown(v)).powWad(iw);
//     int256 a = -int256(v) + iL * xPowW * yOverXPowW + iv
//         - int256(v.mulWadDown(w)) + int256(xPowW * iy * negGamma);
//     int256 b = a - int256(v + v).powWad(-iw) * iL
//         - (iv * yOverXPowW).powWad(int256(ONE) - iw)
//             * (negGamma).powWad(int256(ONE.divWadDown(ONE - w)));
//     int256 c = b
//         / (
//             (-int256(ONE) + iw) * (int256(v) + int256(mid))
//                 * (-iL * int256(mid) * yOverXPowW + iv * iy * negGamma)
//         );
// }

function diffLower(
    uint256 S,
    uint256 L,
    uint256 x,
    uint256 y,
    G3M.G3MParams memory params,
    uint256 v
) pure returns (int256) {
    (int256 wx, int256 wy, int256 swapFee) =
        (int256(params.wX), int256(params.wY), int256(params.swapFee));
    int256 igamma = int256(ONE) - swapFee;
    uint256 gamma = ONE - params.swapFee;
    int256 yOverX = int256(y.divWadDown(x));
    int256 yOverXPowWx = yOverX.powWad(wx);
    int256 yOverXPowWy = yOverX.powWad(wy);
    console2.log("gamma", gamma);

    uint256 numerator;
    {
        int256 first = int256(
            L.mulWadDown(params.wX).mulWadDown(x).mulWadDown(
                uint256(yOverXPowWx)
            )
        );
        console2.log("first", first);

        int256 second = -int256(
            (v - v.mulWadDown(params.wX) + x).mulWadDown(y).mulWadDown(
                ONE - gamma
            )
        );
        console2.log("2nd", second);

        int256 third = int256(v + x).powWad(-wx);
        console2.log("3nd", third);

        int256 fourth = -int256(L - v.mulWadDown(uint256(yOverXPowWy)).mulWadDown(ONE - gamma));
        console2.log("4nd", fourth);

        numerator = uint256(first + second).mulWadDown(
            uint256(
                (third * -fourth / int256(ONE)).powWad(
                    int256(ONE.divWadDown(ONE - params.wX))
                )
            )
        );

        console2.log("num", numerator);
    }

    int256 denominator;
    {
        int256 dFirst = -int256(ONE) + wx;
        console2.log("d1", dFirst);
        int256 dSecond = int256(v + x);
        console2.log("d2", dSecond);
        int256 dThird = -(
            int256(L.mulWadDown(x).mulWadDown(uint256(yOverXPowWx)))
                + (
                    (int256(v.mulWadDown(y)) * (-int256(ONE) + igamma))
                        / int256(ONE)
                )
        );
        console2.log("d3", dThird);
        denominator = dFirst * dSecond * dThird / int256(ONE) / int256(ONE);
        console2.log("den", denominator);
    }

    int256 result =
        -int256(S) + int256(numerator.divWadDown(uint256(denominator)));
    console2.log("res", result);
    return result;
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
