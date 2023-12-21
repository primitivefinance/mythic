// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

/// @dev Contract that holds the strategy parameterization and validation function.
interface IStrategy {
    function init(bytes calldata data)
        external
        returns (
            bool valid,
            int256 swapConstantGrowth,
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );

    function validate(bytes calldata data)
        external
        view
        returns (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta,
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );

    function dynamicSlot() external view returns (bytes memory data);
}
