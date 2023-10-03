// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./lib/G3MPRBMath.sol";
import "./IG3M.sol";

/**
 * @notice Geometric Mean Market Maker.
 */
contract G3M is IG3M {
    /// @notice Thrown when the old invariant is greater than the new one.
    error InvalidSwap(UD60x18 oldInvariant, UD60x18 newInvariant);

    /**
     * @notice Address of the admin of the contract. Note that the current only
     * access control is to update the weights of the pool.
     */
    address public admin;

    /// @inheritdoc IG3M
    address public tokenX;

    /// @inheritdoc IG3M
    address public tokenY;

    /// @inheritdoc IG3M
    UD60x18 public reserveX;

    /// @inheritdoc IG3M
    UD60x18 public reserveY;

    /// @inheritdoc IG3M
    UD60x18 public totalLiquidity;

    /// @inheritdoc IG3M
    mapping(address => UD60x18) public balanceOf;

    /// @dev Last computed weight of token X.
    UD60x18 private lastWeightX;

    /// @dev Timestamp of the last weight X sync.
    uint256 private lastWeightXSync;

    /// @dev Target weight of token X.
    UD60x18 private targetWeightX;

    /// @dev Timestamp of the end of the weight X update.
    uint256 private weightXUpdateEnd;

    /**
     * @dev This value is used to increase or decrease the last weight X to
     * gradually reach the target weight X.
     */
    UD60x18 private weightXUpdatePerSecond;

    /// @dev Reverts if the sender is not the admin.
    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    /**
     *
     * @param tokenX_ Address of token X.
     * @param tokenY_ Address of token Y.
     * @param weightX_ Weight of token X, expressed in WAD (note that `weightY`
     * will be computed as `1 WAD - weightX`).
     */
    constructor(address tokenX_, address tokenY_, UD60x18 weightX_) {
        require(tokenX_ != tokenY_, "Invalid tokens");
        tokenX = tokenX_;
        tokenY = tokenY_;
        admin = msg.sender;

        // TODO: Maybe we can reuse an existing function to replace these lines?
        // It's a bit annoying because we need to initialize these values before
        // we can actually call `setWeightX`.
        require(weightX_ >= MIN_WEIGHT, "Weight X too low");
        require(weightX_ <= MAX_WEIGHT, "Weight X too high");
        targetWeightX = weightX_;
        weightXUpdateEnd = block.timestamp;

        // TODO: Not sure if we need to initialize these two values since the
        // `_syncWeightX` function will update them anyway?
        lastWeightX = weightX_;
        lastWeightXSync = block.timestamp;
    }

    /**
     * @dev Computes and stores the current weight of token X, as well as the
     * timestamp of the last weight sync.
     */
    function _syncWeightX() private {
        lastWeightX = weightX();
        lastWeightXSync = block.timestamp;
    }

    /// @inheritdoc IG3M
    function setWeightX(
        UD60x18 newTargetWeightX,
        uint256 newWeightXUpdateEnd
    ) public onlyAdmin {
        require(newTargetWeightX >= MIN_WEIGHT, "Weight X too low");
        require(newTargetWeightX <= MAX_WEIGHT, "Weight X too high");
        require(newWeightXUpdateEnd > block.timestamp, "Update end pasted");

        _syncWeightX();

        UD60x18 weightXDelta = lastWeightX > newTargetWeightX
            ? lastWeightX - newTargetWeightX
            : newTargetWeightX - lastWeightX;

        weightXUpdatePerSecond =
            weightXDelta / convert(newWeightXUpdateEnd - block.timestamp);
        targetWeightX = newTargetWeightX;
        weightXUpdateEnd = newWeightXUpdateEnd;
    }

    /// @inheritdoc IG3M
    function initPool(
        uint256 amountX,
        uint256 amountY
    ) external returns (UD60x18) {
        require(totalLiquidity.isZero(), "Pool already initialized");

        UD60x18 amountXUD60x18 = convert(amountX);
        UD60x18 amountYUD60x18 = convert(amountY);

        UD60x18 invariant = computeInvariant(
            convert(amountX), weightX(), convert(amountY), weightY()
        );
        UD60x18 liquidity = invariant * ud(2);

        totalLiquidity = totalLiquidity + liquidity;
        balanceOf[msg.sender] = liquidity - BURNT_LIQUIDITY;
        balanceOf[address(0)] = BURNT_LIQUIDITY;
        reserveX = amountXUD60x18;
        reserveY = amountYUD60x18;

        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        return liquidity - BURNT_LIQUIDITY;
    }

    /// @inheritdoc IG3M
    function addLiquidity(UD60x18 liquidity)
        external
        returns (uint256 amountX, uint256 amountY)
    {
        require(!totalLiquidity.isZero(), "Pool not initialized");

        amountX = computeAmountInGivenExactLiquidity(
            totalLiquidity, liquidity, reserveX
        );
        amountY = computeAmountInGivenExactLiquidity(
            totalLiquidity, liquidity, reserveY
        );

        ERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        emit AddLiquidity(msg.sender, liquidity, amountX, amountY);

        reserveX = reserveX + convert(amountX);
        reserveY = reserveY + convert(amountY);
        balanceOf[msg.sender] = balanceOf[msg.sender] + liquidity;
        totalLiquidity = totalLiquidity + liquidity;
    }

    /// @inheritdoc IG3M
    function removeLiquidity(UD60x18 liquidity)
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

        balanceOf[msg.sender] = balanceOf[msg.sender] - liquidity;
        totalLiquidity = totalLiquidity - liquidity;
        reserveX = reserveX - convert(amountX);
        reserveY = reserveY - convert(amountY);

        emit RemoveLiquidity(msg.sender, liquidity, amountX, amountY);
    }

    /// @inheritdoc IG3M
    function swapAmountIn(
        bool swapDirection,
        uint256 amountIn
    ) external returns (uint256 amountOut) {
        UD60x18 currentWeightX = weightX();
        UD60x18 currentWeightY = weightY();

        UD60x18 invariant =
            computeInvariant(reserveX, currentWeightX, reserveY, currentWeightY);

        uint256 fees = amountIn * SWAP_FEE / 10_000;
        uint256 amountInWithoutFees = amountIn - fees;

        amountOut = computeOutGivenIn(
            amountInWithoutFees,
            swapDirection ? reserveX : reserveY,
            swapDirection ? reserveY : reserveX,
            swapDirection ? currentWeightX : currentWeightY,
            swapDirection ? currentWeightY : currentWeightX
        );

        UD60x18 newInvariant = computeInvariant(
            swapDirection
                ? reserveX + convert(amountInWithoutFees)
                : reserveX - convert(amountOut),
            currentWeightX,
            swapDirection
                ? reserveY - convert(amountOut)
                : reserveY + convert(amountInWithoutFees),
            currentWeightY
        );

        if (invariant > newInvariant) {
            revert InvalidSwap(invariant, newInvariant);
        }

        if (swapDirection) {
            reserveX = reserveX + convert(amountIn);
            reserveY = reserveY - convert(amountOut);
        } else {
            reserveX = reserveX - convert(amountOut);
            reserveY = reserveY + convert(amountIn);
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
        UD60x18 currentWeightX = weightX();
        UD60x18 currentWeightY = weightY();

        UD60x18 invariant =
            computeInvariant(reserveX, currentWeightX, reserveY, currentWeightY);

        uint256 amountIn = computeInGivenOut(
            amountOut,
            swapDirection ? reserveX : reserveY,
            swapDirection ? reserveY : reserveX,
            swapDirection ? currentWeightX : currentWeightY,
            swapDirection ? currentWeightY : currentWeightX
        );

        UD60x18 newInvariant = computeInvariant(
            swapDirection
                ? reserveX + convert(amountIn)
                : reserveX - convert(amountOut),
            currentWeightX,
            swapDirection
                ? reserveY - convert(amountOut)
                : reserveY + convert(amountIn),
            currentWeightY
        );

        if (invariant > newInvariant) {
            revert InvalidSwap(invariant, newInvariant);
        }

        amountInWithFees = amountIn * 10_000 / (10_000 - SWAP_FEE);

        if (swapDirection) {
            reserveX = reserveX + convert(amountInWithFees);
            reserveY = reserveY - convert(amountOut);
        } else {
            reserveX = reserveX - convert(amountOut);
            reserveY = reserveY + convert(amountInWithFees);
        }

        ERC20(swapDirection ? tokenX : tokenY).transferFrom(
            msg.sender, address(this), amountInWithFees
        );
        ERC20(swapDirection ? tokenY : tokenX).transfer(msg.sender, amountOut);

        emit Swap(msg.sender, swapDirection, amountInWithFees, amountOut);
    }

    /// @inheritdoc IG3M
    function weightX() public view returns (UD60x18) {
        if (block.timestamp >= weightXUpdateEnd) {
            return targetWeightX;
        }

        uint256 timeElapsed = block.timestamp - lastWeightXSync;
        UD60x18 weightXDelta = convert(timeElapsed) * weightXUpdatePerSecond;

        if (lastWeightX > targetWeightX) {
            return lastWeightX - weightXDelta;
        } else {
            return lastWeightX + weightXDelta;
        }
    }

    /// @inheritdoc IG3M
    function weightY() public view returns (UD60x18) {
        return UNIT - weightX();
    }

    /// @inheritdoc IG3M
    function getSpotPrice() external view returns (uint256) {
        return computeSpotPrice(reserveX, weightX(), reserveY, weightY());
    }
}
