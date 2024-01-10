// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/utils/FixedPointMathLib.sol";
import "../../lib/StrategyLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

/// @dev Parameterization of the Log Normal curve.
struct G3MParameters {
    uint256 wx;
    uint256 wy;
    uint256 swapFee;
}

function tradingFunction(
    uint256 rx,
    uint256 ry,
    uint256 L,
    G3MParameters memory params
) pure returns (int256) {
    uint256 a = uint256(int256(rx).powWad(int256(params.wx)));
    uint256 b = uint256(int256(ry).powWad(int256(params.wy)));

    return int256(a.mulWadUp(b)) - int256(L);
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePrice(
    uint256 rx,
    uint256 ry,
    G3MParameters memory params
) pure returns (uint256 price) {
    uint256 n = ry.divWadDown(params.wy);
    uint256 d = rx.divWadDown(params.wx);
    price = n.divWadDown(d);
}
