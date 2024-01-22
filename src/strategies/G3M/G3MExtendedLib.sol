// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./G3MLib.sol";
import "./G3M.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeLGivenX(
    uint256 x,
    uint256 S,
    G3M.G3MParams memory params
) pure returns (uint256) {
    return x.mulWadUp(params.wY.divWadUp(params.wX.mulWadUp(S)));
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

function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    G3M.G3MParams memory params
) pure returns (bytes memory) {
    uint256 L = computeLGivenX(amountX, initialPrice, params);
    uint256 rY = computeYGivenL(L, initialPrice, params);
    L = computeNextLiquidity(amountX, rY, params);
    return abi.encode(amountX, rY, L, params.wX, params.swapFee);
}

function computeInitialPoolData2(
    uint256 amountX,
    uint256 initialPrice,
    G3M.G3MParams memory params
) pure returns (bytes memory) {
    uint256 L = computeLGivenX(amountX, initialPrice, params);
    uint256 rY = computeYGivenL(L, initialPrice, params);
    L = computeNextLiquidity(amountX, rY, params);
    return abi.encode(amountX, rY, L, params);
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
