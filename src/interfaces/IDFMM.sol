// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @dev Contract that holds the reserve and liquidity state.
interface IDFMM {
    // Structs

    struct Pool {
        bool inited;
        address controller;
        address strategy;
        address tokenX;
        address tokenY;
        uint256 reserveX;
        uint256 reserveY;
        uint256 totalLiquidity;
        uint256 feeGrowth;
    }

    struct InitParams {
        address strategy;
        address tokenX;
        address tokenY;
        bytes data;
    }

    // Errors

    /// @dev Thrown when the pool has not been initialized yet.
    error NotInitialized();

    /// @dev Thrown when the invariant is invalid.
    error Invalid(bool negative, uint256 swapConstantGrowth);

    error InvalidTokens();

    /// @dev Thrown when a new call is made during a locked state.
    error Locked();

    /// @dev Thrown when the transfer of the input amount is invalid.
    error InvalidSwapInputTransfer();

    /// @dev Thrown when the transfer of the output amount is invalid.
    error InvalidSwapOutputTransfer();

    error NotController();

    // Events

    event LogPoolStats(
        uint256 rx,
        uint256 ry,
        uint256 L,
        int256 invariant,
        uint256 sigma,
        uint256 strike,
        uint256 tau,
        uint256 timestamp
    );

    /**
     * @notice Emitted when the pool is initialized.
     * @param account Address initializing the pool.
     * @param reserveX Initial reserve of token X in the pool.
     * @param reserveY Initial reserve of token Y in the pool.
     * @param totalLiquidity Initial liquidity in the pool.
     */
    event Init(
        address indexed account,
        address indexed strategy,
        uint256 poolId,
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
        address indexed account,
        uint256 poolId,
        uint256 deltaX,
        uint256 deltaY,
        uint256 deltaL
    );

    /**
     * @notice Emitted when liquidity is deallocated from the pool.
     * @param account Address deallocating liquidity.
     * @param deltaX Amount of token X being deallocated.
     * @param deltaY Amount of token Y being deallocated.
     * @param deltaL Amount of liquidity being returned to the pool.
     */
    event Deallocate(
        address indexed account,
        uint256 poolId,
        uint256 deltaX,
        uint256 deltaY,
        uint256 deltaL
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
        uint256 indexed poolId,
        bool isSwapXForY,
        uint256 inputAmount,
        uint256 outputAmount
    );

    // Setters

    function init(InitParams calldata params)
        external
        returns (uint256, uint256, uint256, uint256);

    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256, uint256, uint256);

    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256, uint256, uint256);

    function swap(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256, uint256);

    function update(uint256 poolId, bytes calldata data) external;

    // Getters

    function liquidityOf(
        address account,
        uint256 poolId
    ) external view returns (uint256 liquidity);

    function lastFeeGrowthOf(
        address account,
        uint256 poolId
    ) external view returns (uint256 lastFeeGrowth);

    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );

    function pools(uint256 poolId)
        external
        view
        returns (
            bool inited,
            address controller,
            address strategy,
            address tokenX,
            address tokenY,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity,
            uint256 feeGrowth
        );
}
