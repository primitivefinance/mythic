pragma solidity ^0.8.13;

import "forge-std/console2.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "../StrategyLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

/// @dev Parameterization of the Log Normal curve.
struct G3mParameters {
    uint256 wx;
    uint256 wy;
}

function tradingFunction(
    uint256 rx,
    uint256 ry,
    uint256 L,
    G3mParameters memory params
) pure returns (int256) {
    uint256 a = uint256(int256(rx).powWad(int256(params.wx)));
    uint256 b = uint256(int256(ry).powWad(int256(params.wy)));

    return int256(a.mulWadUp(b)) - int256(L);
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePrice(
    uint256 rx,
    uint256 ry,
    G3mParameters memory params
) pure returns (uint256 price) {
    uint256 n = ry.divWadDown(params.wy);
    uint256 d = rx.divWadDown(params.wx);
    price = n.divWadDown(d);
}
