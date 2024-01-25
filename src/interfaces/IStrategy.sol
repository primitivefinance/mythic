// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @dev Contract that holds the strategy parameterization and validation function.
interface IStrategy {
    error InvalidUpdateCode();
    error NotDFMM();

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

    function update(uint256 poolId, bytes calldata data) external;

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
