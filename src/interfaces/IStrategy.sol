// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

/**
 * @title Strategy Interface.
 * @author Primitive
 * @notice All the strategy contracts must implement this interface.
 */
interface IStrategy {
    // Errors

    /// @dev Thrown when the update code is invalid.
    error InvalidUpdateCode();

    /// @dev Thrown when the sender is not the DFMM contract.
    error NotDFMM();

    /// @dev Thrown when the sender is authorized.
    error InvalidSender();

    // Setters

    /**
     * @notice Intializes a new pool.
     * @param sender Address that called the DFMM contract.
     * @param poolId Id of the pool to initialize.
     * @param data Pool parameters encoded as bytes.
     * @return valid True if the initialization is valid.
     * @return swapConstantGrowth Initial swap growth.
     * @return reserveX Initial reserve of token X.
     * @return reserveY Initial reserve of token Y.
     * @return totalLiquidity Initial liquidity of the pool.
     */
    function init(
        address sender,
        uint256 poolId,
        bytes calldata data
    )
        external
        returns (
            bool valid,
            int256 swapConstantGrowth,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        );

    // Getters

    /**
     * @notice Returns the name of the strategy.
     * @dev The name of the strategy is included in the name of
     * the liquidity token.
     */
    function name() external view returns (string memory);

    function validateAllocateOrDeallocate(
        address sender,
        uint256 poolId,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        );

    function validateSwap(
        address sender,
        uint256 poolId,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        );

    function update(
        address sender,
        uint256 poolId,
        bytes calldata data
    ) external;

    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256);

    function dfmm() external view returns (address);

    function getPoolParams(uint256 poolId)
        external
        view
        returns (bytes calldata params);
}
