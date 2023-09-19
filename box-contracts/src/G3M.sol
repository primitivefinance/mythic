// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IG3M {
    event AddLiquidity(address indexed sender, uint256 liquidity);
    event RemoveLiquidity(address indexed sender, uint256 liquidity);
    event Swap(address indexed sender, bool swapDirection, uint256 amount);
    event UpdateWeight(uint256 oldPrimaryWeight, uint256 newPrimaryWeight);

    function addLiquidity(uint256 liquidity) external;
    function removeLiquidity(uint256 liquidity) external;
    function swap(bool swapDirection, uint256 amount) external;
    function updateWeight(uint256 newPrimaryWeight) external;
    function getSpotPrice() external view returns (uint256);
    function getPrimaryWeight() external view returns (uint256);
    function getSecondaryWeight() external view returns (uint256);
}

contract G3M is IG3M {
    address public admin;
    address public token1;
    address public token2;
    uint256 public primaryWeight;
    uint256 public secondaryWeight;

    mapping(address => uint256) public balanceOf;

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    constructor(address token1_, address token2_, uint256 primaryWeight_) {
        token1 = token1_;
        token2 = token2_;
        primaryWeight = primaryWeight_;
        secondaryWeight = 10_000 - primaryWeight;
    }

    function updateWeights(uint256 newPrimaryWeight) external onlyAdmin {
        emit UpdateWeight(primaryWeight, newPrimaryWeight);
        primaryWeight = newPrimaryWeight;
        secondaryWeight = 10_000 - newPrimaryWeight;
    }
}
