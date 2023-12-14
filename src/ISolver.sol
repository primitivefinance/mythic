// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

/// @dev Contract that holds the strategy parameterization and validation function.
interface ISolver {
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

    function internalPrice(
        uint256 reserveXWad,
        uint256 totalLiquidity
    ) external view returns (uint256 price);

    function getNextLiquidity(
        uint256 reserveXWad,
        uint256 reserveYWad,
        uint256 totalLiquidity
    ) external view returns (uint256);

    function getParams()
        external
        view
        returns (uint256 strikePrice, uint256 sigma, uint256 tau);
}
