// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { UD60x18, ud, UNIT, convert } from "@prb/math/UD60x18.sol";

/**
 * @dev Amount of liquidity burnt when a pool is initialized for the
 * first time. Main reason is mainly to avoid the case where the pool
 * gets totally drained and someone calls `initPool` again.
 * @custom:todo Check if the amount is correct?
 */
UD60x18 constant BURNT_LIQUIDITY = UD60x18.wrap(1_000);

/// @dev Current swap fee (expressed in 10,000).
uint256 constant SWAP_FEE = 30; // 0.3%

/// @dev Minimum weight of a token in the pool.
UD60x18 constant MIN_WEIGHT = UD60x18.wrap(0.01e18);

/// @dev Maximum weight of a token in the pool.
UD60x18 constant MAX_WEIGHT = UD60x18.wrap(990000000000000000);

/**
 * @dev Computes the invariant of the pool (rounding down) using the
 * following formula:
 *
 *        ⎛  wX⎞   ⎛  wY⎞
 *    k = ⎝rX  ⎠ ⋅ ⎝rY  ⎠
 *
 * @param rX Reserve of token X
 * @param wX Weight of token X
 * @param rY Reserve of token Y
 * @param wY Weight of token Y
 * @return k Invariant of the pool
 */
function computeInvariant(
    UD60x18 rX,
    UD60x18 wX,
    UD60x18 rY,
    UD60x18 wY
) pure returns (UD60x18 k) {
    UD60x18 n = rX.pow(wX);
    UD60x18 d = rY.pow(wY);
    k = n.mul(d);
}

/**
 * @dev Computes the spot price of a pool using the following formula:
 *
 *       rI
 *       ──
 *       wI
 * p =  ────
 *       rO
 *       ──
 *       wO
 *
 * @param rI Reserve of the input token
 * @param wI Weight of the input token
 * @param rO Reserve of the output token
 * @param wO Weight of the output token
 * @return p Spot price of the pool
 */
function computeSpotPrice(
    UD60x18 rI,
    UD60x18 wI,
    UD60x18 rO,
    UD60x18 wO
) pure returns (uint256 p) {
    UD60x18 n = rI / wI;
    UD60x18 d = rO / wO;
    p = convert(n / d);
}
