/// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "solmate/utils/SafeTransferLib.sol";
import "./interfaces/IStrategy.sol";

error Min();

function abs(int256 input) pure returns (uint256 output) {
    if (input == type(int256).min) revert Min();
    if (input < 0) {
        assembly {
            output := add(not(input), 1)
        }
    } else {
        assembly {
            output := input
        }
    }
}

interface NewCore {
    // Errors

    /// @dev Thrown when the pool has already been initialized.
    error AlreadyInitialized();

    /// @dev Thrown when the pool has not been initialized yet.
    error NotInitialized();

    /// @dev Thrown when a new call is made during a locked state.
    error Locked();

    /// @dev Thrown when the invariant is invalid.
    error Invalid(bool negative, uint256 swapConstantGrowth);

    /// @dev Thrown when the transfer of the input amount is invalid.
    error InvalidSwapInputTransfer();

    /// @dev Thrown when the transfer of the output amount is invalid.
    error InvalidSwapOutputTransfer();

    // Events

    /**
     * @notice Emitted when the pool is initialized.
     * @param account Address initializing the pool.
     * @param reserveX Initial reserve of token X in the pool.
     * @param reserveY Initial reserve of token Y in the pool.
     * @param totalLiquidity Initial liquidity in the pool.
     */
    event Init(
        address account,
        uint256 reserveX,
        uint256 reserveY,
        uint256 totalLiquidity
    );

    /**
     * @notice Emitted when liquidity is allocated into the pool.
     * @param account Address allocating liquidity.
     * @param deltaX Amount of token X being allocated.
     * @param deltaY Amount of token Y being allocated.
     * @param deltaL Amount of liquidity received by the allocator.
     */
    event Allocate(
        address indexed account, uint256 deltaX, uint256 deltaY, uint256 deltaL
    );

    /**
     * @notice Emitted when liquidity is deallocated from the pool.
     * @param account Address deallocating liquidity.
     * @param deltaX Amount of token X being deallocated.
     * @param deltaY Amount of token Y being deallocated.
     * @param deltaL Amount of liquidity being returned to the pool.
     */
    event Deallocate(
        address indexed account, uint256 deltaX, uint256 deltaY, uint256 deltaL
    );

    /**
     * @notice Emitted when a swap is made.
     * @param account Address making the swap.
     * @param isSwapXForY True if token X are being swapped for token Y.
     * @param inputAmount Amount of token sent by the swapper.
     * @param outputAmount Amount of token received by the swapper.
     */
    event Swap(
        address indexed account,
        bool isSwapXForY,
        uint256 inputAmount,
        uint256 outputAmount
    );

    // Getters

    // Setters
    function init(bytes calldata data)
        external
        returns (uint256, uint256, uint256);

    function allocate(bytes calldata data)
        external
        returns (uint256, uint256, uint256);

    function deallocate(bytes calldata data)
        external
        returns (uint256, uint256, uint256);
}

contract Core is NewCore {
    using FixedPointMathLib for uint256;

    address public immutable strategy;
    address public immutable tokenX;
    address public immutable tokenY;

    bool public inited;

    uint256 public reserveX;
    uint256 public reserveY;
    uint256 public totalLiquidity;
    uint256 public feeGrowth = FixedPointMathLib.WAD;
    uint256 public swapFee;

    mapping(address account => uint256 balance) public balanceOf;
    mapping(address account => uint256 lastFeeGrowth) public lastFeeGrowthOf;

    uint256 internal locked = 1;

    modifier initialized() {
        if (!inited) revert NotInitialized();
        _;
    }

    modifier lock() {
        if (locked == 2) revert Locked();
        locked = 2;
        _;
        locked = 1;
    }

    constructor(
        address strategy_,
        address tokenX_,
        address tokenY_,
        uint256 swapFee_
    ) {
        strategy = strategy_;
        tokenX = tokenX_;
        tokenY = tokenY_;
        swapFee = swapFee_;
    }

    function init(bytes calldata data)
        public
        lock
        returns (uint256, uint256, uint256)
    {
        if (inited) revert AlreadyInitialized();

        (
            bool valid,
            int256 swapConstantGrowth,
            uint256 initialReserveX,
            uint256 initialReserveY,
            uint256 initialTotalLiquidity
        ) = IStrategy(strategy).init(data);

        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        inited = true;
        reserveX = initialReserveX;
        reserveY = initialReserveY;
        totalLiquidity = initialTotalLiquidity;

        balanceOf[msg.sender] = initialTotalLiquidity;
        lastFeeGrowthOf[msg.sender] = feeGrowth;

        SafeTransferLib.safeTransferFrom(
            ERC20(tokenX), msg.sender, address(this), initialTotalLiquidity
        );

        SafeTransferLib.safeTransferFrom(
            ERC20(tokenY), msg.sender, address(this), initialReserveY
        );

        emit Init(
            msg.sender, initialReserveX, initialReserveY, initialTotalLiquidity
        );

        return (initialReserveX, initialReserveY, initialTotalLiquidity);
    }

    function allocate(bytes calldata data)
        public
        lock
        initialized
        returns (uint256, uint256, uint256)
    {
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePool(true, data);

        _updateBalance();

        balanceOf[msg.sender] += deltaL;
        lastFeeGrowthOf[msg.sender] = feeGrowth;

        SafeTransferLib.safeTransferFrom(
            ERC20(tokenX), msg.sender, address(this), deltaX
        );
        SafeTransferLib.safeTransferFrom(
            ERC20(tokenY), msg.sender, address(this), deltaY
        );

        emit Allocate(msg.sender, deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    function deallocate(bytes calldata data)
        public
        lock
        initialized
        returns (uint256, uint256, uint256)
    {
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePool(false, data);

        _updateBalance();

        balanceOf[msg.sender] -= deltaL;
        lastFeeGrowthOf[msg.sender] = feeGrowth;

        SafeTransferLib.safeTransfer(ERC20(tokenX), msg.sender, deltaX);
        SafeTransferLib.safeTransfer(ERC20(tokenY), msg.sender, deltaY);

        emit Deallocate(msg.sender, deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    function swap(bytes calldata data) public lock initialized {
        (
            bool valid,
            int256 swapConstantGrowth,
            int256 deltaLiquidity,
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedTotalLiquidity
        ) = IStrategy(strategy).validateSwap(data);

        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        uint256 preLiquidity = totalLiquidity;
        totalLiquidity = adjustedTotalLiquidity;

        uint256 growth = totalLiquidity.divWadDown(preLiquidity);
        feeGrowth = feeGrowth.mulWadDown(growth);

        (bool isSwapXForY, uint256 inputAmount, uint256 outputAmount) =
            _settle(adjustedReserveX, adjustedReserveY);

        emit Swap(msg.sender, isSwapXForY, inputAmount, outputAmount);
    }

    function _updatePool(
        bool isAllocate,
        bytes calldata data
    ) private returns (uint256, uint256, uint256) {
        (
            bool valid,
            int256 invariant,
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedTotalLiquidity
        ) = IStrategy(strategy).validateAllocateOrDeallocate(data);

        if (!valid) revert Invalid(invariant < 0, abs(invariant));

        uint256 deltaX = isAllocate
            ? adjustedReserveX - reserveX
            : reserveX - adjustedReserveX;
        uint256 deltaY = isAllocate
            ? adjustedReserveY - reserveY
            : reserveY - adjustedReserveY;
        uint256 deltaL = isAllocate
            ? adjustedTotalLiquidity - totalLiquidity
            : totalLiquidity - adjustedTotalLiquidity;

        reserveX = adjustedReserveX;
        reserveY = adjustedReserveY;
        totalLiquidity = adjustedTotalLiquidity;

        return (deltaX, deltaY, deltaL);
    }

    function _updateBalance() private {
        if (
            balanceOf[msg.sender] > 0
                && feeGrowth != lastFeeGrowthOf[msg.sender]
        ) {
            uint256 growth = feeGrowth.mulWadDown(lastFeeGrowthOf[msg.sender]);
            balanceOf[msg.sender] = balanceOf[msg.sender].mulWadDown(growth);
        }
    }

    function _settle(
        uint256 adjustedReserveX,
        uint256 adjustedReserveY
    ) private returns (bool, uint256, uint256) {
        (uint256 originalReserveXWad, uint256 originalReserveYWad) =
            (reserveX, reserveY);

        bool isSwapXForY = adjustedReserveX > reserveX;

        address inputToken = isSwapXForY ? tokenX : tokenY;
        address outputToken = isSwapXForY ? tokenY : tokenX;
        uint256 inputAmount = isSwapXForY
            ? adjustedReserveX - originalReserveXWad
            : adjustedReserveY - originalReserveYWad;
        uint256 outputAmount = isSwapXForY
            ? originalReserveYWad - adjustedReserveY
            : originalReserveXWad - adjustedReserveX;

        // TODO: Improve these lines, using custom errors would be nice but the
        // syntax might get a bit ugly.
        if (isSwapXForY) {
            require(originalReserveYWad > adjustedReserveY, "invalid swap");
        } else {
            require(originalReserveXWad > adjustedReserveX, "invalid swap");
        }

        reserveX = adjustedReserveX;
        reserveY = adjustedReserveY;

        uint256 preInputBalance = ERC20(inputToken).balanceOf(address(this));
        uint256 preOutputBalance = ERC20(outputToken).balanceOf(address(this));

        SafeTransferLib.safeTransferFrom(
            ERC20(inputToken), msg.sender, address(this), inputAmount
        );
        SafeTransferLib.safeTransfer(
            ERC20(outputToken), msg.sender, outputAmount
        );

        uint256 postInputBalance = ERC20(inputToken).balanceOf(address(this));
        uint256 postOutputBalance = ERC20(outputToken).balanceOf(address(this));

        if (postInputBalance < preInputBalance + inputAmount) {
            revert InvalidSwapInputTransfer();
        }

        if (postOutputBalance < preOutputBalance - outputAmount) {
            revert InvalidSwapOutputTransfer();
        }

        return (isSwapXForY, inputAmount, outputAmount);
    }
}
