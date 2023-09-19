// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IG3M {
    event AddLiquidity(address indexed sender, uint256 liquidity, uint256 amountX, uint256 amountY);
    event RemoveLiquidity(address indexed sender, uint256 liquidity, uint256 amountX, uint256 amountY);
    event Swap(address indexed sender, bool swapDirection, uint256 input, uint256 output);
    event UpdatePrimaryWeight(uint256 oldPrimaryWeight, uint256 newPrimaryWeight);

    function addLiquidity(uint256 liquidity) external returns (uint256 amountX, uint256 amountY);
    function removeLiquidity(uint256 liquidity) external returns (uint256 amountX, uint256 amountY);
    function swap(bool swapDirection, uint256 input, uint256 output) external;
    function updatePrimaryWeight(uint256 newPrimaryWeight) external;
    function getSpotPriceX() external view returns (uint256);
    function getSpotPriceY() external view returns (uint256);
    function getPrimaryWeight() external view returns (uint256);
    function getSecondaryWeight() external view returns (uint256);
}
