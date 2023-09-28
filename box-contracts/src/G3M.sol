// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./G3MMath.sol";
import "./IG3M.sol";

/**
 * @notice Geometric Mean Market Maker.
 */
contract G3M is IG3M {
    /// @notice Thrown when the old invariant is greater than the new one.
    error InvalidSwap(uint256 oldInvariant, uint256 newInvariant);

    /// @notice Address of the admin of the contract. Note that the current
    /// only access  control is to update the weights of the pool.
    address public admin;

    /// @inheritdoc IG3M
    address public tokenX;

    /// @inheritdoc IG3M
    address public tokenY;

    /// @inheritdoc IG3M
    uint256 public weightX;

    /// @inheritdoc IG3M
    uint256 public reserveX;

    /// @inheritdoc IG3M
    uint256 public reserveY;

    /// @inheritdoc IG3M
    uint256 public totalLiquidity;

    mapping(address => uint256) public balanceOf;

    /**
     * @notice Amount of liquidity burnt when a pool is initialized for the
     * first time. Main reason is mainly to avoid the case where the pool
     * gets totally drained and someone calls `initPool` again.
     * @custom:todo Check if the amount is correct?
     */
    uint256 public constant BURNT_LIQUIDITY = 1_000;

    /// @notice Current swap fee (expressed in 10,000).
    uint256 public constant SWAP_FEE = 30; // 0.3%

    /// @notice Minimum weight of a token in the pool.
    uint256 public constant MIN_WEIGHT = 0.01e18;

    /// @notice Maximum weight of a token in the pool.
    uint256 public constant MAX_WEIGHT = FixedPoint.ONE - MIN_WEIGHT;

    /// @dev Reverts if the sender is not the admin.
    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    /// @param tokenX_ Address of token X.
    /// @param tokenY_ Address of token Y.
    /// @param weightX_ Weight of token X, expressed in WAD (note that `weightY`
    /// will be computed as `1 WAD - weightX`).
    constructor(address tokenX_, address tokenY_, uint256 weightX_) {
        require(tokenX_ != tokenY_, "Invalid tokens");
        require(weightX_ >= MIN_WEIGHT, "Weight X too low");
        require(weightX_ <= MAX_WEIGHT, "Weight X too high");
        tokenX = tokenX_;
        tokenY = tokenY_;
        weightX = weightX_;
        admin = msg.sender;
    }

    /// @inheritdoc IG3M
    function updateWeightX(uint256 newWeightX) external onlyAdmin {
        emit UpdateWeightX(weightX, newWeightX);
        weightX = newWeightX;
    }

    /// @inheritdoc IG3M
    function initPool(
        uint256 amountX,
        uint256 amountY
    ) external returns (uint256) {
        require(totalLiquidity == 0, "Pool already initialized");
        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        amountX *= FixedPoint.ONE;
        amountY *= FixedPoint.ONE;

        uint256 invariant = computeInvariantDown(
            amountX, weightX, amountY, FixedPoint.ONE - weightX
        );
        uint256 liquidity = FixedPoint.mulDown(invariant, 2);

        totalLiquidity += liquidity;
        balanceOf[msg.sender] += liquidity - BURNT_LIQUIDITY;
        balanceOf[address(0)] += BURNT_LIQUIDITY;
        reserveX += amountX;
        reserveY += amountY;

        return liquidity - BURNT_LIQUIDITY;
    }

    /// @inheritdoc IG3M
    function addLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY)
    {
        require(totalLiquidity > 0, "Pool not initialized");

        amountX = computeAmountInGivenExactLiquidity(
            totalLiquidity, liquidity, reserveX
        );
        amountY = computeAmountInGivenExactLiquidity(
            totalLiquidity, liquidity, reserveY
        );

        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        emit AddLiquidity(msg.sender, liquidity, amountX, amountY);

        reserveX += toWad(amountX);
        reserveY += toWad(amountY);
        balanceOf[msg.sender] += liquidity;
        totalLiquidity += liquidity;
    }

    /// @inheritdoc IG3M
    function removeLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY)
    {
        require(balanceOf[msg.sender] >= liquidity, "Insufficient liquidity");

        amountX = computeAmountOutGivenExactLiquidity(
            totalLiquidity, liquidity, reserveX
        );
        amountY = computeAmountOutGivenExactLiquidity(
            totalLiquidity, liquidity, reserveY
        );

        ERC20(tokenX).transfer(msg.sender, amountX);
        ERC20(tokenY).transfer(msg.sender, amountY);

        balanceOf[msg.sender] -= liquidity;
        totalLiquidity -= liquidity;
        reserveX -= toWad(amountX);
        reserveY -= toWad(amountY);

        emit RemoveLiquidity(msg.sender, liquidity, amountX, amountY);
    }

    /// @inheritdoc IG3M
    function swapAmountIn(
        bool swapDirection,
        uint256 amountIn
    ) external returns (uint256 amountOut) {
        uint256 invariant = computeInvariantDown(
            reserveX, weightX, reserveY, FixedPoint.ONE - weightX
        );

        uint256 fees = amountIn * SWAP_FEE / 10_000;
        uint256 amountInWithoutFees = amountIn - fees;

        amountOut = computeOutGivenIn(
            toWad(amountInWithoutFees),
            swapDirection ? reserveX : reserveY,
            swapDirection ? reserveY : reserveX,
            swapDirection ? weightX : FixedPoint.ONE - weightX,
            swapDirection ? FixedPoint.ONE - weightX : weightX
        );

        uint256 newInvariant = computeInvariantUp(
            swapDirection
                ? reserveX + toWad(amountInWithoutFees)
                : reserveX - toWad(amountOut),
            weightX,
            swapDirection
                ? reserveY - toWad(amountOut)
                : reserveY + toWad(amountInWithoutFees),
            FixedPoint.ONE - weightX
        );

        if (invariant > newInvariant) {
            revert InvalidSwap(invariant, newInvariant);
        }

        if (swapDirection) {
            reserveX += toWad(amountIn);
            reserveY -= toWad(amountOut);
        } else {
            reserveX -= toWad(amountOut);
            reserveY += toWad(amountIn);
        }

        ERC20(swapDirection ? tokenX : tokenY).transferFrom(
            msg.sender, address(this), amountIn
        );
        ERC20(swapDirection ? tokenY : tokenX).transfer(msg.sender, amountOut);

        emit Swap(msg.sender, swapDirection, amountIn, amountOut);
    }

    /// @inheritdoc IG3M
    function swapAmountOut(
        bool swapDirection,
        uint256 amountOut
    ) external returns (uint256 amountInWithFees) {
        uint256 invariant = computeInvariantUp(
            reserveX, weightX, reserveY, FixedPoint.ONE - weightX
        );

        uint256 amountIn = computeInGivenOut(
            toWad(amountOut),
            swapDirection ? reserveX : reserveY,
            swapDirection ? reserveY : reserveX,
            swapDirection ? weightX : FixedPoint.ONE - weightX,
            swapDirection ? FixedPoint.ONE - weightX : weightX
        );

        uint256 newInvariant = computeInvariantUp(
            swapDirection
                ? reserveX + toWad(amountIn)
                : reserveX - toWad(amountOut),
            weightX,
            swapDirection
                ? reserveY - toWad(amountOut)
                : reserveY + toWad(amountIn),
            FixedPoint.ONE - weightX
        );

        if (invariant > newInvariant) {
            revert InvalidSwap(invariant, newInvariant);
        }

        amountInWithFees = amountIn * 10_000 / (10_000 - SWAP_FEE);

        if (swapDirection) {
            reserveX += toWad(amountInWithFees);
            reserveY -= toWad(amountOut);
        } else {
            reserveX -= toWad(amountOut);
            reserveY += toWad(amountInWithFees);
        }

        ERC20(swapDirection ? tokenX : tokenY).transferFrom(
            msg.sender, address(this), amountInWithFees
        );
        ERC20(swapDirection ? tokenY : tokenX).transfer(msg.sender, amountOut);

        emit Swap(msg.sender, swapDirection, amountInWithFees, amountOut);
    }

    /// @inheritdoc IG3M
    function weightY() external view returns (uint256) {
        return FixedPoint.ONE - weightX;
    }

    /// @inheritdoc IG3M
    function getSpotPrice() external view returns (uint256) {
        return computeSpotPrice(
            reserveX, weightX, reserveY, FixedPoint.ONE - weightX
        );
    }
}
