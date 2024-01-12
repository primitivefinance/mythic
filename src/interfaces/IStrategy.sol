// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

/// @dev Contract that holds the strategy parameterization and validation function.
interface IStrategy {
    function init(
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

    function validateAllocateOrDeallocate(
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

    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256, uint256, uint256);

    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256);
}