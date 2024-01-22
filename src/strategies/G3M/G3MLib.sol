// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/utils/FixedPointMathLib.sol";
import "./G3M.sol";
import "../../lib/StrategyLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function tradingFunction(
    uint256 rX,
    uint256 rY,
    uint256 L,
    G3M.G3MParams memory params
) pure returns (int256) {
    uint256 a = uint256(int256(rX).powWad(int256(params.wX)));
    uint256 b = uint256(int256(rY).powWad(int256(params.wY)));

    return int256(a.mulWadUp(b)) - int256(L);
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePrice(
    uint256 rX,
    uint256 rY,
    G3M.G3MParams memory params
) pure returns (uint256 price) {
    uint256 n = rX.divWadDown(params.wX);
    uint256 d = rY.divWadDown(params.wY);
    price = n.divWadDown(d);
}

/// @dev Finds the root of the swapConstant given the independent variable liquidity.
function computeNextLiquidity(
    uint256 rX,
    uint256 rY,
    G3M.G3MParams memory params
) pure returns (uint256 L) {
    return uint256(int256(rX).powWad(int256(params.wX))).mulWadUp(
        uint256(int256(rY).powWad(int256(params.wY)))
    );
}

