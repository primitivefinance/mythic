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
        address tokenX_,
        address tokenY_,
        uint256 sigma_,
        uint256 strikePrice_,
        uint256 tau_,
        uint256 swapFee_
    ) {
        tokenX = ERC20(tokenX_);
        tokenY = ERC20(tokenY_);
        sigma = sigma_;
        strikePrice = strikePrice_;
        tau = tau_;

        require(swapFee_ < ONE, "Swap fee too high");
        swapFee = swapFee_;
    }

    function instantiate(
        uint256 amount,
        uint256 price
    ) external returns (uint256 amountX, uint256 amountY, uint256 liquidity) {
        return initPool(true, amount, price);
    }

    function initPool(
        bool exactX,
        uint256 amount,
        uint256 price
    ) public returns (uint256 amountX, uint256 amountY, uint256 liquidity) {
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
    event dlEvent(uint256 deltaL);
    event dyEvent(uint256 deltaY);
    event dxEvent(uint256 dx);
    event feeEvent(uint256 fees);
    event amountOutIntEvent(int256 amountOut);
    event amountOutEvent(uint256 amountOut);
    event amountOutAbsEvent(int256 amountOut);
    event amountOutEvent3(uint256 amountOut);
    event spotPriceEvent(uint256 spotPrice);
    event amountInEvent(uint256 amountIn);
    function _swap(bool swapDirection, uint256 amountIn) internal returns (uint256 amountOut) {
        emit amountInEvent(amountIn);
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau);
        emit spotPriceEvent(price);

        if (swapDirection) {
            uint256 fees = amountIn * (ONE - swapFee) / ONE;
            emit feeEvent(fees);
            uint256 deltaX = amountIn - fees;
            emit dxEvent(deltaX);
            uint256 deltaL = computeLGivenX(deltaX, price, strikePrice, sigma);
            emit dlEvent(deltaL);
            uint256 deltaY = computeYGivenL(deltaL, price, strikePrice, sigma);
            emit dyEvent(deltaY);

            int256 computedAmountOut = computeOutputYGivenX(
                reserveX,
                amountIn,
                reserveY,
                deltaY,
                totalLiquidity,
                deltaL,
                strikePrice,
                sigma
            );
            emit amountOutIntEvent(computedAmountOut);
            emit amountOutAbsEvent(-computedAmountOut);

            amountOut = uint256(
                ~(
                    computeOutputYGivenX(
                        reserveX,
                        amountIn,
                        reserveY,
                        deltaY,
                        totalLiquidity,
                        deltaL,
                        strikePrice,
                        sigma
                    ) - 1
                )
            );

            emit amountOutEvent(amountOut);

            totalLiquidity += deltaL;
            reserveX += amountIn;
            reserveY -= amountOut;

            tokenX.transferFrom(msg.sender, address(this), amountIn);
            tokenY.transfer(msg.sender, amountOut);
        } else {
            uint256 fees = amountIn * (ONE - swapFee) / ONE;
            uint256 deltaY = amountIn - fees;
            uint256 deltaL = computeLGivenY(deltaY, price, strikePrice, sigma);
            uint256 deltaX = computeXGivenL(deltaL, price, strikePrice, sigma);

            amountOut = uint256(
                ~(
                    computeOutputXGivenY(
                        reserveX,
                        amountIn,
                        reserveY,
                        deltaX,
                        totalLiquidity,
                        deltaL,
                        strikePrice,
                        sigma
                    ) - 1
                )
            );

            totalLiquidity += deltaL;
            reserveY += amountIn;
            reserveX -= amountOut;

            tokenX.transferFrom(msg.sender, address(this), amountIn);
            tokenY.transfer(msg.sender, amountOut);
        }

        emit Swap(
            msg.sender,
            swapDirection,
            amountIn,
            amountOut,
            computeSpotPrice(reserveX, totalLiquidity, strikePrice, sigma, tau)
        );
    }

    function swapAmountIn(
        bool swapDirection,
        uint256 amountIn
    ) external returns (uint256) {
        return _swap(swapDirection, amountIn);
    }

    function setSwapFee(uint256 newSwapFee) external {
        require(newSwapFee < ONE, "New swap fee too high");
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

    function getLiquidity() external view returns (uint256) {
        return totalLiquidity;
    }

    function getInvariant() external view returns (int256) {
        return computeInvariant(reserveX, totalLiquidity, reserveY, strikePrice);
    }

    function getStrategyData() external view returns (bytes memory data) { 
        return abi.encode(sigma, strikePrice, tau);
    }

    event logKL(uint256 K, uint256 L, uint256 KL);
    event logCDF(int256 negativeSigma, int256 ppf, int256 cdf);
    event logFinal(int256 intMulWadDown, int256 intY, int256 intDeltaY, int256 intAmountOut);
    function computeOutputYGivenX(
        uint256 x,
        uint256 deltaX,
        uint256 y,
        uint256 deltaY,
        uint256 L,
        uint256 deltaL,
        uint256 K,
        uint256 sigmaParam
    ) public returns (int256) {
        uint256 KL = FixedPointMathLib.mulWadDown(K, L + deltaL);
        emit logKL(K, L, KL);

        int256 negativeSigma = -int256(sigmaParam);
        int256 ppf = Gaussian.ppf(
            int256(FixedPointMathLib.divWadDown(x + deltaX, L + deltaL))
        );
        int256 cdf = Gaussian.cdf(negativeSigma - ppf);
        emit logCDF(-negativeSigma, -ppf, cdf);

        // int256 cdf = Gaussian.cdf(
        //     negativeSigma
        //         - Gaussian.ppf(
        //             int256(FixedPointMathLib.divWadDown(x + deltaX, L + deltaL))
        //         )
        // );
        int256 intMulWadDown = int256(FixedPointMathLib.mulWadDown(KL, uint256(cdf)));
        int256 intY = int256(y);
        int256 intDeltaY = int256(deltaY);
        int256 intAmountOut = intMulWadDown - intY - intDeltaY;
        emit logFinal(intMulWadDown, intY, intDeltaY, -intAmountOut);


        // return int256(FixedPointMathLib.mulWadDown(KL, uint256(cdf))) - int256(y)
        //     - int256(deltaY);
        return intAmountOut;
    }
}
