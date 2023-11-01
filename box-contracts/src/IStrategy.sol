// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

/// @dev Draft of a generic interface that we could reuse for all strategies.
interface IStrategy {
    // Actions
    function instantiate(
        uint256 initial_x_wad,
        uint256 initial_price_wad
    ) external;

    // Getters
    function getSpotPrice() external view returns (uint256);
    function getSwapFee() external view returns (uint256);
    function getReserveX() external view returns (uint256);
    function getReserveY() external view returns (uint256);
    function getInvariant() external view returns (int256);
    function getStrategyData() external view returns (bytes memory);

    event AddLiquidity(
        address indexed sender,
        uint256 liquidity,
        uint256 amountX,
        uint256 amountY
    );

    event RemoveLiquidity(
        address indexed sender,
        uint256 liquidity,
        uint256 amountX,
        uint256 amountY
    );

    event Swap(
        address indexed sender,
        bool swapDirection,
        uint256 input,
        uint256 output,
        uint256 newPrice
    );
}
