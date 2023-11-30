// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

/// @dev Draft of a generic interface that we could reuse for all strategies.
interface IStrategy {
    // Actions
    function initExactX(
        uint256 amountX,
        uint256 price
    ) external returns (uint256, uint256);

    // Getters
    function getSpotPrice() external view returns (uint256);
    function getSwapFee() external view returns (uint256);
    function getReserveX() external view returns (uint256);
    function getReserveY() external view returns (uint256);
    function getInvariant() external view returns (int256);
    function getLiquidity() external view returns (uint256);
    function getStrategyData() external view returns (bytes memory);
    function logData() external;
    function getPortfolioValue() external view returns (uint256);
    function getNextLiquidity() external view returns (uint256);

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

    event LogReserves(
        uint256 reserveX, uint256 reserveY, uint256 blockTimestamp
    );
}
