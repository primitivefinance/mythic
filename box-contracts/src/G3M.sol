// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IG3M {
    event AddLiquidity(address indexed sender, uint256 liquidity);
    event RemoveLiquidity(address indexed sender, uint256 liquidity);
    event Swap(address indexed sender, bool swapDirection, uint256 amount);
    event UpdateWeight(address indexed sender, uint256 oldWeight, uint256 newWeight);

    function addLiquidity(uint256 liquidity) external;
    function removeLiquidity(uint256 liquidity) external;
    function swap(bool swapDirection, uint256 amount) external;
    function updateWeight(uint256 newWeight) external;
    function getSpotPrice() external view returns (uint256);
    function getPrimaryWeight() external view returns (uint256);
    function getSecondaryWeight() external view returns (uint256);
}

contract G3M {}
