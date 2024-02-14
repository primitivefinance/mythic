// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

/**
 * @title DFMM Interface
 * @author Primitive
 */
interface IDFMM {
    // Structs

    struct Pool {
        address strategy;
        address tokenX;
        address tokenY;
        uint256 reserveX;
        uint256 reserveY;
        uint256 totalLiquidity;
        address liquidityToken;
    }

    struct InitParams {
        address strategy;
        address tokenX;
        address tokenY;
        bytes data;
    }

    // Errors

    error OnlyWETH();

    /// @dev Thrown when the invariant is invalid.
    error Invalid(bool negative, uint256 swapConstantGrowth);

    /// @dev Thrown when pool tokens are identical.
    error InvalidTokens();

    /// @dev Thrown when a new call is made during a locked state.
    error Locked();

    /// @dev Thrown when the reserves are invalid after a swap.
    error InvalidSwap();

    /// @dev Thrown when the transfer of the input amount is invalid.
    error InvalidSwapInputTransfer();

    /// @dev Thrown when the transfer of the output amount is invalid.
    error InvalidSwapOutputTransfer();

    /// @dev Thrown when a clone contract could not be deployed.
    error ERC1167FailedCreateClone();

    // Events

    /**
     * @notice Emitted when the pool is initialized.
     * @param account Address initializing the pool.
     * @param reserveX Initial reserve of token X in the pool.
     * @param reserveY Initial reserve of token Y in the pool.
     * @param totalLiquidity Initial liquidity in the pool.
     */
    event Init(
        address indexed account,
        address strategy,
        address indexed tokenX,
        address indexed tokenY,
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
     * @param deltaL Amount of liquidity being deallocated.
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

    /**
     * @notice Intializes a new pool.
     * @param params A struct containing the initialization parameters.
     * @return poolId Id of the newly initialized pool.
     * @return reserveX Initial amount of token X in the pool.
     * @return reserveY Initial amount of token Y in the pool.
     * @return totalLiquidity Initial amount of liquidity in the pool.
     */
    function init(InitParams calldata params)
        external
        payable
        returns (
            uint256 poolId,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        );

    /**
     * @notice Allocates liquidity into the pool `poolId`.
     * @param poolId Id of the pool to allocate liquidity into.
     * @param data An array of bytes used by the strategy contract.
     * @return deltaX Amount of token X allocated into the pool.
     * @return deltaY Amount of token Y allocated into the pool.
     * @return deltaL Amount of liquidity received by the allocator.
     */
    function allocate(
        uint256 poolId,
        bytes calldata data
    )
        external
        payable
        returns (uint256 deltaX, uint256 deltaY, uint256 deltaL);

    /**
     * @notice Deallocates liquidity from the pool `poolId`.
     * @param poolId Id of the pool to deallocate liquidity from.
     * @param data An array of bytes used by the strategy contract.
     * @return deltaX Amount of token X deallocated from the pool.
     * @return deltaY Amount of token Y deallocated from the pool.
     * @return deltaL Amount of liquidity being deallocated.
     */
    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256 deltaX, uint256 deltaY, uint256 deltaL);

    /**
     * @notice Swaps tokens into pool `poolId`.
     * @param poolId Id of the pool to swap tokens into.
     * @param data An array of bytes used by the strategy contract.
     * @return inputAmount Amount of tokens sent to the DFMM contract.
     * @return outputAmount Amount of tokens received by the swapper.
     */
    function swap(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256 inputAmount, uint256 outputAmount);

    /**
     * @notice Updates pool `poolId` by calling the associated strategy.
     * @param poolId Id of the pool to update.
     * @param data An array of bytes used by the strategy contract.
     */
    function update(uint256 poolId, bytes calldata data) external;

    // Getters

    /// @notice Address of the implementation of the LPToken contract.
    function lpTokenImplementation() external view returns (address);

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
            address strategy,
            address tokenX,
            address tokenY,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity,
            address liquidityToken
        );
}
