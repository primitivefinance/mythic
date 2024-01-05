// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "./IParams.sol";

/// @dev Contract that holds the reserve and liquidity state.
interface IMultiCore is IParams {
    error NotInitialized();

    error Invalid(bool negative, uint256 swapConstantGrowth);

    error InvalidSwap();

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

    event Init(
        address indexed swapper,
        address indexed strategy,
        uint256 XXXXXXX,
        uint256 YYYYYY,
        uint256 LLLLLL
    );

    event Swap(
        address indexed swapper,
        address source,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        int256 liquidityDelta
    );

    event Allocate(uint256 x, uint256 y, uint256 l);

    event Deallocate(uint256 x, uint256 y, uint256 l);

    function init(InitParams calldata params)
        external
        returns (uint256, uint256, uint256);

    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256, uint256, uint256);

    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256, uint256, uint256);

    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );
}
