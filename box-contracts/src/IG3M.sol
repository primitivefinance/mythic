// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IG3M {
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
        uint256 output
    );
    event UpdateWeightX(uint256 oldWeightX, uint256 newWeightX);

    function addLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY);
    function removeLiquidity(uint256 liquidity)
        external
        returns (uint256 amountX, uint256 amountY);
    function swapAmountIn(
        bool swapDirection,
        uint256 amountIn
    ) external returns (uint256 amountOut);
    function updateWeightX(uint256 newWeightX) external;
    function getSpotPrice() external view returns (uint256);

    function tokenX() external view returns (address);
    function tokenY() external view returns (address);

    function reserveX() external view returns (uint256);
    function reserveY() external view returns (uint256);

    function totalLiquidity() external view returns (uint256);

    function balanceOf(address account) external view returns (uint256);

    function weightX() external view returns (uint256);
    function weightY() external view returns (uint256);
}
