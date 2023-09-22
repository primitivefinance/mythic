// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./FixedPoint.sol";
import "./IG3M.sol";

library G3MMath {
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
     * @dev Computes the required amount of tokens to mint an exact amount of liquidity using the following formula:
     *
     *      ⎛⎛t + l⎞    ⎞
     *  d = ⎜⎜─────⎟ - 1⎟ ⋅ r
     *      ⎝⎝  t  ⎠    ⎠
     *
     * @param t Total amount of liquidity in the pool
     * @param l Exact amount of liquidity to deposit
     * @param r Reserve of the input token
     * @return d Required amount of tokens
     */
    function computeAmountInGivenExactLiquidity(uint256 t, uint256 l, uint256 r) internal pure returns (uint256 d) {
        return (
            FixedPoint.mulDown(FixedPoint.sub(FixedPoint.divDown(FixedPoint.add(t, l), t), FixedPoint.ONE), r)
                / FixedPoint.ONE
        );
    }
}

contract G3M is IG3M {
    error InvalidSwap(uint256 expectedInvariant, uint256 actualInvariant);

    address public admin;
    address public tokenX;
    address public tokenY;
    uint256 public primaryWeight;
    uint256 public totalLiquidity;
    uint256 public reserveX;
    uint256 public reserveY;

    mapping(address => uint256) public balanceOf;

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    constructor(address tokenX_, address tokenY_, uint256 primaryWeight_) {
        require(tokenX == tokenY, "Invalid tokens");
        require(primaryWeight_ <= FixedPoint.ONE, "Invalid weight");
        tokenX = tokenX_;
        tokenY = tokenY_;
        primaryWeight = primaryWeight_;
        admin = msg.sender;
    }

    function updatePrimaryWeight(uint256 newPrimaryWeight) external onlyAdmin {
        emit UpdatePrimaryWeight(primaryWeight, newPrimaryWeight);
        primaryWeight = newPrimaryWeight;
    }

    function initPool(uint256 amountX, uint256 amountY) external returns (uint256 liquidity) {
        require(totalLiquidity == 0, "Pool not initialized");
        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        amountX *= FixedPoint.ONE;
        amountY *= FixedPoint.ONE;

        uint256 invariant = G3MMath.computeInvariant(amountX, primaryWeight, amountY, FixedPoint.ONE - primaryWeight);
        liquidity = FixedPoint.mulDown(invariant, 2);

        totalLiquidity += liquidity;
        balanceOf[msg.sender] += liquidity;
        reserveX += amountX;
        reserveY += amountY;
    }

    function addLiquidity(uint256 liquidity) external returns (uint256 amountX, uint256 amountY) {
        require(totalLiquidity > 0, "Pool not initialized");

        amountX = G3MMath.computeAmountInGivenExactLiquidity(totalLiquidity, liquidity, reserveX);
        amountY = G3MMath.computeAmountInGivenExactLiquidity(totalLiquidity, liquidity, reserveY);
        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);
        emit AddLiquidity(msg.sender, liquidity, amountX, amountY);

        reserveX += amountX * FixedPoint.ONE;
        reserveY += amountY * FixedPoint.ONE;

        balanceOf[msg.sender] += liquidity;
        totalLiquidity += liquidity;
    }

    function removeLiquidity(uint256 liquidity) external returns (uint256 amountX, uint256 amountY) {
        require(balanceOf[msg.sender] >= liquidity, "Insufficient liquidity");

        // (amountX, amountY) = calculateRemoveAmounts(liquidity);

        balanceOf[msg.sender] -= liquidity;
        totalLiquidity -= liquidity;
        // reserveX -= amountX * WAD;
        // reserveY -= amountY * WAD;

        ERC20(tokenX).transfer(msg.sender, amountX);
        ERC20(tokenY).transfer(msg.sender, amountY);

        emit RemoveLiquidity(msg.sender, liquidity, amountX, amountY);
    }

    function swap(bool swapDirection, uint256 amountIn) external {
        uint256 invariant;

        if (swapDirection) {} else {}

        uint256 newInvariant;

        if (invariant != newInvariant) {
            revert InvalidSwap(invariant, newInvariant);
        }

        ERC20(swapDirection ? tokenX : tokenY).transferFrom(msg.sender, address(this), amountIn);
        // ERC20(swapDirection ? tokenY : tokenX).transfer(msg.sender, amountOut);

        // emit Swap(msg.sender, swapDirection, amountIn, amountOut);
    }

    function getPriWeight() external view returns (uint256) {
        return primaryWeight;
    }

    function getSecondaryWeight() external view returns (uint256) {
        return FixedPoint.ONE - primaryWeight;
    }

    function getSpotPrice() external view returns (uint256) {
        return G3MMath.computeSpotPrice(reserveX, primaryWeight, reserveY, FixedPoint.ONE - primaryWeight);
    }
}
