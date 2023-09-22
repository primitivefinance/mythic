// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./FixedPoint.sol";

library G3MMath {
    function fromWad(uint256 a) internal pure returns (uint256 b) {
        b = a / FixedPoint.ONE;
    }

    function toWad(uint256 a) internal pure returns (uint256 b) {
        b = a * FixedPoint.ONE;
    }

    /**
     * @dev Computes the invariant of the pool using the following formula:
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
    function computeInvariant(uint256 rX, uint256 wX, uint256 rY, uint256 wY) internal pure returns (uint256 k) {
        k = FixedPoint.mulDown(FixedPoint.powDown(rX, wX), FixedPoint.powDown(rY, wY));
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
    function computeSpotPrice(uint256 rI, uint256 wI, uint256 rO, uint256 wO) internal pure returns (uint256 p) {
        p = FixedPoint.divDown(FixedPoint.divDown(rI, wI), FixedPoint.divDown(rO, wO));
    }

    /**
     * @dev Computes the required amount of tokens needed to mint an exact amount of liquidity using the following formula:
     *
     *     ⎛⎛t + l⎞    ⎞
     * d = ⎜⎜─────⎟ - 1⎟ ⋅ r
     *     ⎝⎝  t  ⎠    ⎠
     *
     * @param t Total amount of liquidity in the pool
     * @param l Exact amount of liquidity to deposit
     * @param r Reserve of the input token
     * @return i Required amount of tokens
     */
    function computeAmountInGivenExactLiquidity(uint256 t, uint256 l, uint256 r) internal pure returns (uint256 i) {
        i = fromWad(FixedPoint.mulDown(FixedPoint.sub(FixedPoint.divDown(FixedPoint.add(t, l), t), FixedPoint.ONE), r));
    }

    /**
     * @dev Computes the received amount of tokens after removing an exact amount of liquidity using the following formula:
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
    function computeAmountOutGivenExactLiquidity(uint256 t, uint256 l, uint256 r) internal pure returns (uint256 o) {
        o = fromWad(FixedPoint.mulDown(FixedPoint.sub(FixedPoint.ONE, FixedPoint.divDown(FixedPoint.sub(t, l), t)), r));
    }

    /**
     * @dev Formula:
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
    function computeOutGivenIn(uint256 aI, uint256 bI, uint256 bO, uint256 wI, uint256 wO)
        internal
        pure
        returns (uint256 aO)
    {
        aO = fromWad(
            FixedPoint.mulDown(
                bO,
                FixedPoint.sub(
                    FixedPoint.ONE,
                    FixedPoint.powDown(FixedPoint.divDown(bI, FixedPoint.add(bI, aI)), FixedPoint.divDown(wI, wO))
                )
            )
        );
    }

    function computeInGivenOut(uint256 aO, uint256 bI, uint256 bO, uint256 wI, uint256 wO)
        internal
        pure
        returns (uint256 aI)
    {
        aI = fromWad(
            FixedPoint.mulDown(
                bI,
                FixedPoint.sub(
                    FixedPoint.powDown(FixedPoint.divDown(bO, FixedPoint.sub(bO, aO)), FixedPoint.divDown(wO, wI)),
                    FixedPoint.ONE
                )
            )
        );
    }
}
