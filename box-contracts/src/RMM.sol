// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./lib/RMMMath.sol";

contract RMM {
    ERC20 public tokenX;
    ERC20 public tokenY;

    uint256 public immutable sigma;
    uint256 public immutable strikePrice;
    uint256 public immutable tau;

    uint256 public reserveX;
    uint256 public reserveY;
    uint256 public totalLiquidity;

    constructor(
        ERC20 tokenX_,
        ERC20 tokenY_,
        uint256 sigma_,
        uint256 strikePrice_,
        uint256 tau_
    ) {
        tokenX = tokenX_;
        tokenY = tokenY_;
        sigma = sigma_;
        strikePrice = strikePrice_;
        tau = tau_;
    }

    function initExactX(
        uint256 amountX,
        uint256 price
    ) external returns (uint256, uint256) {
        uint256 l = computeLGivenX(amountX, price, strikePrice, sigma);
        uint256 amountY = computeYGivenL(l, price, strikePrice, sigma);

        totalLiquidity = l;
        reserveX = amountX;
        reserveY = amountY;

        tokenX.transferFrom(msg.sender, address(this), amountX);
        tokenY.transferFrom(msg.sender, address(this), amountY);

        return (l, amountY);
    }

    function initExactY(
        uint256 amountY,
        uint256 price
    ) external returns (uint256, uint256) {
        uint256 l = computeLGivenY(amountY, price, strikePrice, sigma);
        uint256 amountX = computeXGivenL(l, price, strikePrice, sigma);

        totalLiquidity = l;
        reserveX = amountX;
        reserveY = amountY;

        tokenX.transferFrom(msg.sender, address(this), amountX);
        tokenY.transferFrom(msg.sender, address(this), amountY);

        return (l, amountX);
    }

    function addLiquidityExactX(uint256 amountX)
        external
        returns (uint256, uint256)
    {
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);

        uint256 newLiquidity =
            computeLGivenX(reserveX + amountX, price, strikePrice, sigma);
        uint256 newReserveY =
            computeYGivenL(newLiquidity, price, strikePrice, sigma);

        uint256 amountY = newReserveY - reserveY;

        uint256 liquidityDelta = newLiquidity - totalLiquidity;
        totalLiquidity = newLiquidity;
        reserveX += amountX;
        reserveY += amountY;

        tokenX.transferFrom(msg.sender, address(this), amountX);
        tokenY.transferFrom(msg.sender, address(this), amountY);

        return (liquidityDelta, amountY);
    }

    function addLiquidityExactY(uint256 amountY)
        external
        returns (uint256, uint256)
    {
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);

        uint256 newLiquidity =
            computeLGivenY(reserveY + amountY, price, strikePrice, sigma);
        uint256 newReserveX =
            computeXGivenL(newLiquidity, price, strikePrice, sigma);

        uint256 amountX = newReserveX - reserveX;

        uint256 liquidityDelta = newLiquidity - totalLiquidity;
        totalLiquidity = newLiquidity;
        reserveX += amountX;
        reserveY += amountY;

        tokenX.transferFrom(msg.sender, address(this), amountX);
        tokenY.transferFrom(msg.sender, address(this), amountY);

        return (liquidityDelta, amountX);
    }

    function removeLiquidityExactX(uint256 amountX)
        external
        returns (uint256, uint256)
    {
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);

        uint256 newLiquidity =
            computeLGivenX(reserveX - amountX, price, strikePrice, sigma);
        uint256 newReserveY =
            computeYGivenL(newLiquidity, price, strikePrice, sigma);

        uint256 amountY = reserveY - newReserveY;

        uint256 liquidityDelta = totalLiquidity - newLiquidity;
        totalLiquidity = newLiquidity;
        reserveX -= amountX;
        reserveY -= amountY;

        tokenX.transfer(msg.sender, amountX);
        tokenY.transfer(msg.sender, amountY);

        return (liquidityDelta, amountY);
    }
}
