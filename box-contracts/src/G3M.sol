// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./G3MMath.sol";
import "./IG3M.sol";

// TODO:
// - Fix fees on swap
// - Add single asset deposit
// - Add single asset withdraw
// - Add min or max amounts on swap
// - Add max amounts on deposit
// - Add min amounts on withdraw
// - Add burnt liquidity on first deposit
// - Fix invariant check on swap
contract G3M is IG3M {
    error InvalidSwap(uint256 expectedInvariant, uint256 actualInvariant);

    address public admin;
    address public tokenX;
    address public tokenY;
    uint256 public primaryWeight;
    uint256 public totalLiquidity;
    uint256 public reserveX;
    uint256 public reserveY;

    uint256 public constant MAX_INVARIANT_DELTA = 3e25;
    uint256 public constant SWAP_FEE = 30; // 0.3%

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

    function initPool(
        uint256 amountX,
        uint256 amountY
    ) external returns (uint256 liquidity) {
        require(totalLiquidity == 0, "Pool not initialized");
        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        amountX *= FixedPoint.ONE;
        amountY *= FixedPoint.ONE;

        uint256 invariant = G3MMath.computeInvariant(
            amountX, primaryWeight, amountY, FixedPoint.ONE - primaryWeight
        );
        liquidity = FixedPoint.mulDown(invariant, 2);

        totalLiquidity += liquidity;
        balanceOf[msg.sender] += liquidity;
        reserveX += amountX;
        reserveY += amountY;
    }

    function addLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY)
    {
        require(totalLiquidity > 0, "Pool not initialized");

        amountX = G3MMath.computeAmountInGivenExactLiquidity(
            totalLiquidity, liquidity, reserveX
        );
        amountY = G3MMath.computeAmountInGivenExactLiquidity(
            totalLiquidity, liquidity, reserveY
        );

        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        emit AddLiquidity(msg.sender, liquidity, amountX, amountY);

        reserveX += G3MMath.toWad(amountX);
        reserveY += G3MMath.toWad(amountY);
        balanceOf[msg.sender] += liquidity;
        totalLiquidity += liquidity;
    }

    function removeLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY)
    {
        require(balanceOf[msg.sender] >= liquidity, "Insufficient liquidity");

        amountX = G3MMath.computeAmountOutGivenExactLiquidity(
            totalLiquidity, liquidity, reserveX
        );
        amountY = G3MMath.computeAmountOutGivenExactLiquidity(
            totalLiquidity, liquidity, reserveY
        );

        ERC20(tokenX).transfer(msg.sender, amountX);
        ERC20(tokenY).transfer(msg.sender, amountY);

        balanceOf[msg.sender] -= liquidity;
        totalLiquidity -= liquidity;
        reserveX -= G3MMath.toWad(amountX);
        reserveY -= G3MMath.toWad(amountY);

        emit RemoveLiquidity(msg.sender, liquidity, amountX, amountY);
    }

    function swapAmountIn(
        bool swapDirection,
        uint256 amountIn
    ) external returns (uint256 amountOut) {
        uint256 fees = amountIn * SWAP_FEE / 10_000;
        uint256 amountInWithoutFees = amountIn - fees;

        amountOut = G3MMath.computeOutGivenIn(
            G3MMath.toWad(amountInWithoutFees),
            swapDirection ? reserveX : reserveY,
            swapDirection ? reserveY : reserveX,
            swapDirection ? primaryWeight : FixedPoint.ONE - primaryWeight,
            swapDirection ? FixedPoint.ONE - primaryWeight : primaryWeight
        );

        uint256 invariant = G3MMath.computeInvariant(
            reserveX, primaryWeight, reserveY, FixedPoint.ONE - primaryWeight
        );

        if (swapDirection) {
            reserveX += G3MMath.toWad(amountIn);
            reserveY -= G3MMath.toWad(amountOut);
        } else {
            reserveX -= G3MMath.toWad(amountOut);
            reserveY += G3MMath.toWad(amountIn);
        }

        uint256 newInvariant = G3MMath.computeInvariant(
            reserveX, primaryWeight, reserveY, FixedPoint.ONE - primaryWeight
        );

        uint256 delta;

        if (invariant > newInvariant) delta = invariant - newInvariant;
        else delta = newInvariant - invariant;

        if (delta > MAX_INVARIANT_DELTA) {
            revert InvalidSwap(invariant, newInvariant);
        }

        ERC20(swapDirection ? tokenX : tokenY).transferFrom(
            msg.sender, address(this), amountIn
        );
        ERC20(swapDirection ? tokenY : tokenX).transfer(msg.sender, amountOut);

        emit Swap(msg.sender, swapDirection, amountIn, amountOut);
    }

    function swapAmountOut(
        bool swapDirection,
        uint256 amountOut
    ) external returns (uint256 amountIn) {
        amountIn = G3MMath.computeInGivenOut(
            G3MMath.toWad(amountOut),
            swapDirection ? reserveX : reserveY,
            swapDirection ? reserveY : reserveX,
            swapDirection ? primaryWeight : FixedPoint.ONE - primaryWeight,
            swapDirection ? FixedPoint.ONE - primaryWeight : primaryWeight
        );

        uint256 invariant = G3MMath.computeInvariant(
            reserveX, primaryWeight, reserveY, FixedPoint.ONE - primaryWeight
        );

        if (swapDirection) {
            reserveX += G3MMath.toWad(amountIn);
            reserveY -= G3MMath.toWad(amountOut);
        } else {
            reserveX -= G3MMath.toWad(amountOut);
            reserveY += G3MMath.toWad(amountIn);
        }

        uint256 newInvariant = G3MMath.computeInvariant(
            reserveX, primaryWeight, reserveY, FixedPoint.ONE - primaryWeight
        );

        uint256 delta;

        if (invariant > newInvariant) delta = invariant - newInvariant;
        else delta = newInvariant - invariant;

        if (delta > MAX_INVARIANT_DELTA) {
            revert InvalidSwap(invariant, newInvariant);
        }

        ERC20(swapDirection ? tokenX : tokenY).transferFrom(
            msg.sender, address(this), amountIn
        );
        ERC20(swapDirection ? tokenY : tokenX).transfer(msg.sender, amountOut);

        emit Swap(msg.sender, swapDirection, amountIn, amountOut);
    }

    function getPrimaryWeight() external view returns (uint256) {
        return primaryWeight;
    }

    function getSecondaryWeight() external view returns (uint256) {
        return FixedPoint.ONE - primaryWeight;
    }

    function getSpotPrice() external view returns (uint256) {
        return G3MMath.computeSpotPrice(
            reserveX, primaryWeight, reserveY, FixedPoint.ONE - primaryWeight
        );
    }
}
