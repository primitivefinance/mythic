// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

interface IG3M {
    event AddLiquidity(address indexed sender, uint256 liquidity, uint256 amount1, uint256 amount2);
    event RemoveLiquidity(address indexed sender, uint256 liquidity, uint256 amount1, uint256 amount2);
    event Swap(address indexed sender, bool swapDirection, uint256 amount);
    event UpdateWeight(uint256 oldPrimaryWeight, uint256 newPrimaryWeight);

    function addLiquidity(uint256 liquidity) external returns (uint256 amount1, uint256 amount2);
    function removeLiquidity(uint256 liquidity) external returns (uint256 amount1, uint256 amount2);
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

    mapping(address => uint256) public balanceOf;

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    constructor(address token1_, address token2_, uint256 primaryWeight_) {
        token1 = token1_;
        token2 = token2_;
        primaryWeight = primaryWeight_;
    }

    function addLiquidity(uint256 liquidity) external returns (uint256 amount1, uint256 amount2) {
        (amount1, amount2) = calculateAmounts(liquidity);

        IERC20(token1).transferFrom(msg.sender, address(this), amount1);
        IERC20(token2).transferFrom(msg.sender, address(this), amount2);

        balanceOf[msg.sender] += liquidity;
        emit AddLiquidity(msg.sender, liquidity, amount1, amount2);
    }

    function updateWeights(uint256 newPrimaryWeight) external onlyAdmin {
        emit UpdateWeight(primaryWeight, newPrimaryWeight);
        primaryWeight = newPrimaryWeight;
    }

    function getPrimaryWeight() external view returns (uint256) {
        return primaryWeight;
    }

    function getSecondaryWeight() external view returns (uint256) {
        return 10_000 - primaryWeight;
    }

    function calculateAmounts(uint256 liquidity) public returns (uint256 amount1, uint256 amount2) {
        amount1 = liquidity * primaryWeight / 10_000;
        amount2 = liquidity - amount1;
    }
}
