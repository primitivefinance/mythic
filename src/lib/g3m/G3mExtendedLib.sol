// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./G3mLib.sol";
import "../BisectionLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeLGivenX(
    uint256 x,
    uint256 S,
    G3mParameters memory params
) pure returns (uint256) {
    return x.mulWadUp(params.wy.divWadUp(params.wx * S));
}

function computeLGivenY(
    uint256 y,
    uint256 S,
    G3mParameters memory params
) pure returns (uint256) {
    return y.mulWadUp(params.wx).divWadUp(params.wy * S);
}

function computeXGivenL(
    uint256 L,
    uint256 S,
    G3mParameters memory params
) pure returns (uint256) {
    return params.wx.mulWadUp(L).divWadUp(params.wy * S);
}

function computeYGivenL(
    uint256 L,
    uint256 S,
    G3mParameters memory params
) pure returns (uint256) {
    return params.wy.mulWadUp(L).divWadUp(params.wx * S);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(bytes memory data, uint256 ry) pure returns (int256) {
    (uint256 rx, uint256 L,, G3mParameters memory params) =
        abi.decode(data, (uint256, uint256, int256, G3mParameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(bytes memory data, uint256 rx) pure returns (int256) {
    (uint256 ry, uint256 L,, G3mParameters memory params) =
        abi.decode(data, (uint256, uint256, int256, G3mParameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rx, uint256 ry,, G3mParameters memory params) =
        abi.decode(data, (uint256, uint256, int256, G3mParameters));
    return tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    G3mParameters memory params
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
    G3mParameters memory params
) pure returns (uint256 L) {
    uint256 lower;
    uint256 upper;
    uint256 iters;

    if (swapConstant < EPSILON && swapConstant > -(EPSILON)) {
        return currentLiquidity;
    } else if (swapConstant < 0) {
        upper = currentLiquidity;
        lower = 100;
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
    G3mParameters memory params
) pure returns (uint256 ry) {
    uint256 lower = 10;
    uint256 upper = 1e27;
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
    G3mParameters memory params
) pure returns (uint256 rx) {
    uint256 lower = 10;
    uint256 upper = 1e27; // max x = 1 - x / l, so l - x
    rx = bisection(
        abi.encode(reserveYWad, liquidity, swapConstant, params),
        lower,
        upper,
        uint256(EPSILON),
        256,
        findRootX
    );
}
