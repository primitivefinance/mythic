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
    k = n * d;
}

/**
 * @dev Computes the spot price of a pool using the following formula:
 *
 *       rO
 *       ──
 *       wO
 * p =  ────
 *       rI
 *       ──
 *       wI
 *
 * @param rO Reserve of the output token
 * @param wO Weight of the output token
 * @param rI Reserve of the input token
 * @param wI Weight of the input token
 * @return p Spot price of the pool
 */
function computeSpotPrice(
    UD60x18 rI,
    UD60x18 wI,
    UD60x18 rO,
    UD60x18 wO
) pure returns (uint256 p) {
    UD60x18 n = rO / wO;
    UD60x18 d = rI / wI;
    p = UD60x18.unwrap(n / d);
}

/**
 * @dev Computes the required amount of tokens needed to mint an exact
 * amount of liquidity. Amounts are rounded up in favor of the contract.
 *
 * The following formula is used:
 *
 *     ⎛⎛t + l⎞    ⎞
 * i = ⎜⎜─────⎟ - 1⎟ ⋅ r
 *     ⎝⎝  t  ⎠    ⎠
 *
 * @param t Total amount of liquidity in the pool
 * @param l Exact amount of liquidity to deposit
 * @param r Reserve of the input token
 * @return i Required amount of tokens
 */
function computeAmountInGivenExactLiquidity(
    UD60x18 t,
    UD60x18 l,
    UD60x18 r
) pure returns (uint256 i) {
    i = convert(((t + l) / t - UNIT) * r);
}

/**
 * @dev Computes the received amount of tokens after removing an exact
 * amount of liquidity. Amounts are rounded down in favor of the contract.
 *
 * The following formula is used:
 *
 *     ⎛    ⎛t - l⎞⎞
 * o = ⎜1 - ⎜─────⎟⎟ ⋅ r
 *     ⎝    ⎝  t  ⎠⎠
 *
 * @param t Total amount of liquidity in the pool
 * @param l Exact amount of liquidity to withdraw
 * @param r Reserve amount of output token
 * @return o Received amount of tokens
 */
function computeAmountOutGivenExactLiquidity(
    UD60x18 t,
    UD60x18 l,
    UD60x18 r
) pure returns (uint256 o) {
    o = convert((UNIT - (t - l) / t) * r);
}

/**
 * @dev Computes the amount of token Y relative to a specific amount of tokenX.
 * This function can be used to compute either the required tokens when adding
 * liquidity or the received tokens when removing liquidity.
 *
 * The following formula is used:
 *
 *      rY
 * dY = ── ⋅ (rX + dX) - rY
 *      rX
 *
 * @param rX Reserve of token X
 * @param rY Reserve of token Y
 * @param dX Differential amount of token X
 * @return dY Differential amount of token Y
 */
function computeDeltaYGivenDeltaX(
    UD60x18 rX,
    UD60x18 rY,
    uint256 dX
) pure returns (uint256 dY) {
    dY = convert((rY / rX) * (rX + convert(dX)) - rY);
}

/**
 * @dev Computes the amount of token X relative to a specific amount of tokenY.
 * This function can be used to compute either the required tokens when adding
 * liquidity or the received tokens when removing liquidity.
 *
 * The following formula is used:
 *
 *      rX
 * dX = ── ⋅ (rY + dY) - rX
 *      rY
 *
 * @param rX Reserve of token X
 * @param rY Reserve of token Y
 * @param dY Differential amount of token Y
 * @return dX Differential amount of token X
 */
function computeDeltaXGivenDeltaY(
    UD60x18 rX,
    UD60x18 rY,
    uint256 dY
) pure returns (uint256 dX) {
    dX = convert((rX / rY) * (rY + convert(dY)) - rX);
}

/**
 * @dev Computes the amount of output tokens received for an exact amount of
 * input tokens. Amounts are rounded down in favor of the contract.
 *
 * The following formula is used:
 *
 *           ⎛             wI⎞
 *           ⎜             ──⎟
 *           ⎜             wO⎟
 *           ⎜    ⎛  bI   ⎞  ⎟
 * aO = bO ⋅ ⎜1 - ⎜───────⎟  ⎟
 *           ⎝    ⎝bI + aI⎠  ⎠
 *
 * @param aI Amount of input token
 * @param bI Balance of the input token
 * @param wI Weight of the input token
 * @param bO Balance of the output token
 * @param wO Weight of the output token
 */
function computeOutGivenIn(
    uint256 aI,
    UD60x18 bI,
    UD60x18 wI,
    UD60x18 bO,
    UD60x18 wO
) pure returns (uint256 aO) {
    UD60x18 f = bI / (bI + convert(aI));
    aO = convert(bO * (UNIT - f.pow(wI / wO)));
}

/**
 * @dev Computes the amount of input tokens required for an exact amount of
 * output tokens. Amounts are rounded up in favor of the contract.
 *
 * The following formula is used:
 *
 *           ⎛         wO    ⎞
 *           ⎜         ──    ⎟
 *           ⎜         wI    ⎟
 *           ⎜⎛  bO   ⎞      ⎟
 * aI = bI ⋅ ⎜⎜───────⎟   - 1⎟
 *           ⎝⎝bO - aO⎠      ⎠
 *
 * @param aO Exact amount of expected output tokens
 * @param bI Balance of the input token
 * @param wI Weight of the input token
 * @param bO Balance of the output token
 * @param wO Weight of the output token
 * @return aI Amount of input tokens required
 */
function computeInGivenOut(
    uint256 aO,
    UD60x18 bI,
    UD60x18 wI,
    UD60x18 bO,
    UD60x18 wO
) pure returns (uint256 aI) {
    UD60x18 f = bO / (bO - convert(aO));
    aI = convert(bI * (f.pow(wO / wI) - UNIT));
}
