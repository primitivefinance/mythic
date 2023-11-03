// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./IStrategy.sol";
import "./lib/RMMMath.sol";

contract RMM is IStrategy {
    ERC20 public tokenX;
    ERC20 public tokenY;

    uint256 public immutable sigma;
    uint256 public immutable strikePrice;
    uint256 public immutable tau;

    uint256 public swapFee;

    uint256 public reserveX;
    uint256 public reserveY;
    uint256 public totalLiquidity;

    mapping(address => uint256) public balanceOf;

    constructor(
        ERC20 tokenX_,
        ERC20 tokenY_,
        uint256 sigma_,
        uint256 strikePrice_,
        uint256 tau_,
        uint256 swapFee_
    ) {
        tokenX = tokenX_;
        tokenY = tokenY_;
        sigma = sigma_;
        strikePrice = strikePrice_;
        tau = tau_;
        swapFee = swapFee_;
    }

    function initPool(
        bool exactX,
        uint256 amount,
        uint256 price
    ) external returns (uint256 amountX, uint256 amountY, uint256 liquidity) {
        require(totalLiquidity == 0, "Pool already initialized");

        if (exactX) {
            amountX = amount;
            liquidity = computeLGivenX(amountX, price, strikePrice, sigma);
            amountY = computeYGivenL(liquidity, price, strikePrice, sigma);
        } else {
            amountY = amount;
            liquidity = computeLGivenY(amountY, price, strikePrice, sigma);
            amountX = computeXGivenL(liquidity, price, strikePrice, sigma);
        }

        totalLiquidity = liquidity;
        reserveX = amountX;
        reserveY = amountY;
        balanceOf[msg.sender] = liquidity;

        tokenX.transferFrom(msg.sender, address(this), amountX);
        tokenY.transferFrom(msg.sender, address(this), amountY);
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

    function addLiquidity(
        bool exactX,
        uint256 amount
    ) external returns (uint256 amountX, uint256 amountY, uint256 liquidity) {
        require(totalLiquidity > 0, "Pool not initialized");

        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);

        if (exactX) {
            amountX = amount;

            uint256 newLiquidity =
                computeLGivenX(reserveX + amountX, price, strikePrice, sigma);
            uint256 newReserveY =
                computeYGivenL(newLiquidity, price, strikePrice, sigma);

            amountY = newReserveY - reserveY;
            liquidity = newLiquidity - totalLiquidity;
        } else {
            amountY = amount;

            uint256 newLiquidity =
                computeLGivenY(reserveY + amountY, price, strikePrice, sigma);
            uint256 newReserveX =
                computeXGivenL(newLiquidity, price, strikePrice, sigma);

            amountX = newReserveX - reserveX;
            liquidity = newLiquidity - totalLiquidity;
        }

        totalLiquidity += liquidity;
        reserveX += amountX;
        reserveY += amountY;
        balanceOf[msg.sender] += liquidity;

        tokenX.transferFrom(msg.sender, address(this), amountX);
        tokenY.transferFrom(msg.sender, address(this), amountY);

        emit AddLiquidity(msg.sender, liquidity, amountX, amountY);
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

        emit AddLiquidity(msg.sender, newLiquidity, amountX, amountY);

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

        emit AddLiquidity(msg.sender, newLiquidity, amountX, amountY);

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

        emit RemoveLiquidity(msg.sender, newLiquidity, amountX, amountY);

        return (liquidityDelta, amountY);
    }

    function removeLiquidityExactY(uint256 amountY)
        external
        returns (uint256, uint256)
    {
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);

        uint256 newLiquidity =
            computeLGivenY(reserveX - amountY, price, strikePrice, sigma);
        uint256 newReserveX =
            computeXGivenL(newLiquidity, price, strikePrice, sigma);

        uint256 amountX = reserveX - newReserveX;

        uint256 liquidityDelta = totalLiquidity - newLiquidity;
        totalLiquidity = newLiquidity;
        reserveX -= amountX;
        reserveY -= amountY;

        tokenX.transfer(msg.sender, amountX);
        tokenY.transfer(msg.sender, amountY);

        emit RemoveLiquidity(msg.sender, newLiquidity, amountX, amountY);

        return (liquidityDelta, amountX);
    }

    function swap(uint256 amountX) external returns (uint256 amountY) {
        uint256 deltaX = amountX - swapFee;

        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);

        uint256 deltaL = computeLGivenX(deltaX, price, strikePrice, sigma);
        uint256 deltaY = computeYGivenL(deltaL, price, strikePrice, sigma);

        amountY = uint256(
            ~(
                computeOutputYGivenX(
                    reserveX,
                    amountX,
                    reserveY,
                    deltaY,
                    totalLiquidity,
                    deltaL,
                    strikePrice,
                    sigma
                ) - 1
            )
        );

        reserveX += amountX;
        reserveY -= amountY;

        tokenX.transferFrom(msg.sender, address(this), amountX);
        tokenY.transfer(msg.sender, amountY);

        emit Swap(
            msg.sender,
            true,
            amountX,
            amountY,
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau)
        );
    }

    function setSwapFee(uint256 newSwapFee) external {
        swapFee = newSwapFee;
    }

    function getSpotPrice() external view returns (uint256) {
        return
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);
    }

    function getSwapFee() external view returns (uint256) {
        return swapFee;
    }

    function getReserveX() external view returns (uint256) {
        return reserveX;
    }

    function getReserveY() external view returns (uint256) {
        return reserveY;
    }

    function getInvariant() external view returns (int256) {
        return computeInvariant(reserveX, totalLiquidity, reserveY, strikePrice);
    }

    function getStrategyData() external view returns (bytes memory data) { }
}
