// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./IG3M.sol";

interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract G3M is IG3M {
    address public admin;
    address public tokenX;
    address public tokenY;
    uint256 public primaryWeight;
    uint256 public reserveX;
    uint256 public reserveY;

    uint256 public constant WAD = 1 ether;

    mapping(address => uint256) public balanceOf;

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    constructor(address tokenX_, address tokenY_, uint256 primaryWeight_) {
        require(tokenX != tokenY, "Invalid tokens");
        require(primaryWeight_ <= WAD, "Invalid weight");
        tokenX = tokenX_;
        tokenY = tokenY_;
        primaryWeight = primaryWeight_;
    }

    function updateWeight(uint256 newPrimaryWeight) external onlyAdmin {
        emit UpdateWeight(primaryWeight, newPrimaryWeight);
        primaryWeight = newPrimaryWeight;
    }

    function addLiquidity(uint256 liquidity) external returns (uint256 amountX, uint256 amountY) {
        (amountX, amountY) = calculateAmounts(liquidity);

        IERC20(tokenX).transferFrom(msg.sender, address(this), amountX);
        IERC20(tokenY).transferFrom(msg.sender, address(this), amountY);

        reserveX += amountX * WAD;
        reserveY += amountY * WAD;

        balanceOf[msg.sender] += liquidity;
        emit AddLiquidity(msg.sender, liquidity, amountX, amountY);
    }

    function removeLiquidity(uint256 liquidity) external returns (uint256 amountX, uint256 amountY) {
        require(balanceOf[msg.sender] >= liquidity, "Insufficient liquidity");
        balanceOf[msg.sender] -= liquidity;

        (amountX, amountY) = calculateAmounts(liquidity);

        reserveX -= amountX * WAD;
        reserveY -= amountY * WAD;

        IERC20(tokenX).transfer(msg.sender, amountX);
        IERC20(tokenY).transfer(msg.sender, amountY);

        emit RemoveLiquidity(msg.sender, liquidity, amountX, amountY);
    }

    function swap(bool swapDirection, uint256 input, uint256 output) external {
        IERC20(swapDirection ? tokenX : tokenY).transferFrom(msg.sender, address(this), input);
        IERC20(swapDirection ? tokenX : tokenY).transfer(msg.sender, output);

        reserveX += (swapDirection ? input : output) * WAD;
        reserveY += (swapDirection ? output : input) * WAD;

        require((reserveX * WAD) / reserveY == primaryWeight, "Invalid swap");

        emit Swap(msg.sender, swapDirection, input, output);
    }

    function updateWeights(uint256 newPrimaryWeight) external onlyAdmin {
        emit UpdateWeight(primaryWeight, newPrimaryWeight);
        primaryWeight = newPrimaryWeight;
    }

    function getPrimaryWeight() external view returns (uint256) {
        return primaryWeight;
    }

    function getSecondaryWeight() external view returns (uint256) {
        return WAD - primaryWeight;
    }

    function getSpotPriceX() external view returns (uint256) {
        return reserveY / reserveX;
    }

    function getSpotPriceY() external view returns (uint256) {
        return reserveX / reserveY;
    }

    function calculateAmounts(uint256 liquidity) public view returns (uint256 amountX, uint256 amountY) {
        amountX = (liquidity * primaryWeight) / WAD;
        amountY = liquidity - amountX;
    }
}
