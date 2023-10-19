// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

/// @dev Draft of a generic interface that we could reuse for all strategies.
interface IStrategy {
    // TODO: This could be a generic function that simply takes some `bytes` as
    // an argument, then each strategy contract will decode the data as they want.
    function initPool(bytes calldata data)
        external
        returns (uint256 amountX, uint256 amountY, uint256 liquidity);

    function addLiquidity(
        bool isExactX,
        uint256 amount
    ) external returns (uint256 amountX, uint256 amountY, uint256 liquidity);

    function removeLiquidity(
        bool isExactX,
        uint256 amount
    ) external returns (uint256 amountX, uint256 amountY, uint256 liquidity);

    function swap(
        bool xForY,
        bool exactIn,
        uint256 amount
    ) external returns (uint256 input, uint256 output);
}
