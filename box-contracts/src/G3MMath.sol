// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./FixedPoint.sol";

/**
 * @dev Converts a fixed-point number to an unsigned integer.
 * @param a Fixed-point number to convert
 * @return b Unsigned integer representation of the fixed-point number
 */
function fromWad(uint256 a) pure returns (uint256 b) {
    b = a / FixedPoint.ONE;
}

/**
 * @dev Converts an unsigned integer to a fixed-point number.
 * @param a Unsigned integer to convert
 * @return b Fixed-point representation of the unsigned integer
 */
function toWad(uint256 a) pure returns (uint256 b) {
    b = a * FixedPoint.ONE;
}

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
function computeInvariantDown(
    uint256 rX,
    uint256 wX,
    uint256 rY,
    uint256 wY
) pure returns (uint256 k) {
    k = FixedPoint.mulDown(
        FixedPoint.powDown(rX, wX), FixedPoint.powDown(rY, wY)
    );
}

/**
 * @dev Computes the invariant of the pool (rounding up) using the following
 * formula:
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
function computeInvariantUp(
    uint256 rX,
    uint256 wX,
    uint256 rY,
    uint256 wY
) pure returns (uint256 k) {
    k = FixedPoint.mulUp(FixedPoint.powUp(rX, wX), FixedPoint.powUp(rY, wY));
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
    uint256 rI,
    uint256 wI,
    uint256 rO,
    uint256 wO
) pure returns (uint256 p) {
    p = FixedPoint.divDown(
        FixedPoint.divDown(rI, wI), FixedPoint.divDown(rO, wO)
    );
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
    uint256 t,
    uint256 l,
    uint256 r
) pure returns (uint256 i) {
    i = fromWad(
        FixedPoint.mulUp(
            FixedPoint.sub(
                FixedPoint.divUp(FixedPoint.add(t, l), t), FixedPoint.ONE
            ),
            r
        )
    );
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
    uint256 t,
    uint256 l,
    uint256 r
) pure returns (uint256 o) {
    o = fromWad(
        FixedPoint.mulDown(
            FixedPoint.sub(
                FixedPoint.ONE, FixedPoint.divDown(FixedPoint.sub(t, l), t)
            ),
            r
        )
    );
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
 * @param bO Balance of the output token
 * @param wI Weight of the input token
 * @param wO Weight of the output token
 */
function computeOutGivenIn(
    uint256 aI,
    uint256 bI,
    uint256 bO,
    uint256 wI,
    uint256 wO
) pure returns (uint256 aO) {
    aO = fromWad(
        FixedPoint.mulDown(
            bO,
            FixedPoint.sub(
                FixedPoint.ONE,
                FixedPoint.powDown(
                    FixedPoint.divDown(bI, FixedPoint.add(bI, aI)),
                    FixedPoint.divDown(wI, wO)
                )
            )
        )
    );
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
 * @param bO Balance of the output token
 * @param wI Weight of the input token
 * @param wO Weight of the output token
 * @return aI Amount of input tokens required
 */
function computeInGivenOut(
    uint256 aO,
    uint256 bI,
    uint256 bO,
    uint256 wI,
    uint256 wO
) pure returns (uint256 aI) {
    aI = fromWad(
        FixedPoint.mulDown(
            bI,
            FixedPoint.sub(
                FixedPoint.powUp(
                    FixedPoint.divUp(bO, FixedPoint.sub(bO, aO)),
                    FixedPoint.divUp(wO, wI)
                ),
                FixedPoint.ONE
            )
        )
    );
}
