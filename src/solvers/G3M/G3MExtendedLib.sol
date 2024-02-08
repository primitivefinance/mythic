// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "src/strategies/G3M/G3M.sol";
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

    int256 invariant = G3MLib.tradingFunction({
        rX: amountX,
        rY: rY,
        L: L,
        params: params
    });

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
            upper = upper.mulDivUp(1_001, 1_000);
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
