// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./IStrategy.sol";
import "./lib/RMMMath.sol";

contract RMM is IStrategy {
    event LogParameters(
        uint256 sigma, uint256 strikePrice, uint256 tau, uint256 blockTimestamp
    );

    ERC20 public tokenX;
    ERC20 public tokenY;

    uint256 public swapFee;

    uint256 public reserveX;
    uint256 public reserveY;
    uint256 public totalLiquidity;

    mapping(address => uint256) public balanceOf;

    uint256 private lastSigma;
    uint256 private targetSigma;
    uint256 private lastSigmaSync;
    uint256 private sigmaUpdatePerSecond;
    uint256 private sigmaUpdateEnd;

    uint256 private lastStrike;
    uint256 private targetStrike;
    uint256 private lastStrikeSync;
    uint256 private strikeUpdatePerSecond;
    uint256 private strikeUpdateEnd;

    uint256 private lastTau;
    uint256 private targetTau;
    uint256 private lastTauSync;
    uint256 private tauUpdatePerSecond;
    uint256 private tauUpdateEnd;


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
        lastSigma = sigma_;
        lastStrike = strikePrice_;
        lastTau = tau_;

        require(swapFee_ < ONE, "Swap fee too high");
        swapFee = swapFee_;
    }

    function instantiate(
        uint256 amount,
        uint256 price
    ) external returns (uint256 amountX, uint256 amountY, uint256 liquidity) {
        return initPool(true, amount, price);
    }

    function getPortfolioValue() public view returns (uint256) {
        return reserveX * getSpotPrice() / 1e18 + reserveY;
    }

    function getParams() public view returns (uint256, uint256, uint256) {
        return (sigma(), strikePrice(), tau());
    }

    function initPool(
        bool exactX,
        uint256 amount,
        uint256 price
    ) public returns (uint256 amountX, uint256 amountY, uint256 liquidity) {
        require(totalLiquidity == 0, "Pool already initialized");

        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();

        if (exactX) {
            amountX = amount;
            liquidity = computeLGivenX(amountX, price, _strike, _sigma, _tau);
            amountY = computeYGivenL(liquidity, price, _strike, _sigma, _tau);
        } else {
            amountY = amount;
            liquidity = computeLGivenY(amountY, price, _strike, _sigma, _tau);
            amountX = computeXGivenL(liquidity, price, _strike, _sigma, _tau);
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
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 l = computeLGivenX(amountX, price, _strike, _sigma, _tau);
        uint256 amountY = computeYGivenL(l, price, _strike, _sigma, _tau);

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
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 l = computeLGivenY(amountY, price, _strike, _sigma, _tau);
        uint256 amountX = computeXGivenL(l, price, _strike, _sigma, _tau);

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

        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();

        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, _strike, _sigma, _tau);

        if (exactX) {
            amountX = amount;

            uint256 newLiquidity =
                computeLGivenX(reserveX + amountX, price, _strike, _sigma, _tau);
            uint256 newReserveY =
                computeYGivenL(newLiquidity, price, _strike, _sigma, _tau);

            amountY = newReserveY - reserveY;
            liquidity = newLiquidity - totalLiquidity;
        } else {
            amountY = amount;

            uint256 newLiquidity =
                computeLGivenY(reserveY + amountY, price, _strike, _sigma, _tau);
            uint256 newReserveX =
                computeXGivenL(newLiquidity, price, _strike, _sigma, _tau);

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
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, _strike, _sigma, _tau);

        uint256 newLiquidity =
            computeLGivenX(reserveX + amountX, price, _strike, _sigma, _tau);
        uint256 newReserveY =
            computeYGivenL(newLiquidity, price, _strike, _sigma, _tau);

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
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, _strike, _sigma, _tau);

        uint256 newLiquidity =
            computeLGivenY(reserveY + amountY, price, _strike, _sigma, _tau);
        uint256 newReserveX =
            computeXGivenL(newLiquidity, price, _strike, _sigma, _tau);

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
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, _strike, _sigma, _tau);

        uint256 newLiquidity =
            computeLGivenX(reserveX - amountX, price, _strike, _sigma, _tau);
        uint256 newReserveY =
            computeYGivenL(newLiquidity, price, _strike, _sigma, _tau);

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
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, _strike, _sigma, _tau);

        uint256 newLiquidity =
            computeLGivenY(reserveX - amountY, price, _strike, _sigma, _tau);
        uint256 newReserveX =
            computeXGivenL(newLiquidity, price, _strike, _sigma, _tau);

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

    function _swap(bool swapDirection, uint256 amountIn) internal returns (uint256 amountOut) {
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 price =
            computeSpotPrice(reserveX, totalLiquidity, _strike, _sigma, _tau);

        if (swapDirection) {
            uint256 fees = amountIn * (ONE - swapFee) / ONE;
            uint256 deltaL = computeLGivenX(fees, price, _strike, _sigma, _tau);

            amountOut = uint256(
                ~(
                    computeOutputYGivenX(
                        reserveX,
                        reserveY,
                        amountIn,
                        totalLiquidity,
                        deltaL,
                        _strike,
                        _sigma,
                        _tau
                    ) - 1
                )
            );

            totalLiquidity += deltaL;
            reserveX += amountIn;
            reserveY -= amountOut;

            tokenX.transferFrom(msg.sender, address(this), amountIn);
            tokenY.transfer(msg.sender, amountOut);
        } else {
            uint256 fees = amountIn * (ONE - swapFee) / ONE;
            uint256 deltaL = computeLGivenY(fees, price, _strike, _sigma, _tau);

            amountOut = uint256(
                ~(
                    computeOutputXGivenY(
                        reserveX,
                        reserveY,
                        amountIn,
                        totalLiquidity,
                        deltaL,
                        _strike,
                        _sigma,
                        _tau
                    ) - 1
                )
            );

            totalLiquidity += deltaL;
            reserveY += amountIn;
            reserveX -= amountOut;

            tokenY.transferFrom(msg.sender, address(this), amountIn);
            tokenX.transfer(msg.sender, amountOut);
        }

        emit Swap(
            msg.sender,
            swapDirection,
            amountIn,
            amountOut,
            computeSpotPrice(reserveX, totalLiquidity, _strike, _sigma, _tau)
        );
    }

    function sigma() public view returns (uint256) {
        if (block.timestamp >= sigmaUpdateEnd) {
            return targetSigma;
        }

        return
            lastSigma > targetSigma 
                ? lastSigma - (block.timestamp - lastSigmaSync) * sigmaUpdatePerSecond
                : lastSigma + (block.timestamp - lastSigmaSync) * sigmaUpdatePerSecond;
    }

    function strikePrice() public view returns (uint256) {
        if (block.timestamp >= strikeUpdateEnd) {
            return targetStrike;
        }

        return 
            lastStrike > targetStrike
                ? lastStrike - (block.timestamp - lastStrikeSync) * strikeUpdatePerSecond
                : lastStrike + (block.timestamp - lastStrikeSync) * strikeUpdatePerSecond;
    }

    function tau() public view returns (uint256) {
        if (block.timestamp >= tauUpdateEnd) {
            return targetTau;
        }

        return 
            lastTau > targetTau 
                ? lastTau - (block.timestamp - lastTauSync) * tauUpdatePerSecond
                : lastTau + (block.timestamp - lastTauSync) * tauUpdatePerSecond;
    }

    function _syncSigma() private {
        lastSigma = sigma();
        lastSigmaSync = block.timestamp;
    }

    function _syncStrike() private {
        lastStrike = strikePrice();
        lastStrikeSync = block.timestamp;
    }

    function _syncTau() private {
        lastTau = tau();
        lastTauSync = block.timestamp;
    }

    function setSigma(uint256 newTargetSigma, uint256 newSigmaUpdateEnd) external {
        require(newSigmaUpdateEnd > block.timestamp, "Update end pasted");

        _syncSigma();

        uint256 sigmaDelta = lastSigma > newTargetSigma
            ? lastSigma - newTargetSigma
            : newTargetSigma - lastSigma;

        sigmaUpdatePerSecond =
            sigmaDelta / (newSigmaUpdateEnd - block.timestamp);
        targetSigma = newTargetSigma;
        sigmaUpdateEnd = newSigmaUpdateEnd;
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    function setStrikePrice(uint256 newTargetStrike, uint256 newStrikeUpdateEnd) external {
        require(newStrikeUpdateEnd > block.timestamp, "Update end pasted");

        _syncStrike();

        uint256 strikeDelta = lastStrike > newTargetStrike
            ? lastStrike - newTargetStrike
            : newTargetStrike - lastStrike;

        strikeUpdatePerSecond =
            strikeDelta / (newStrikeUpdateEnd - block.timestamp);
        targetStrike = newTargetStrike;
        strikeUpdateEnd = newStrikeUpdateEnd;
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    function setTau(uint256 newTargetTau, uint256 newTauUpdateEnd) external {
        require(newTauUpdateEnd > block.timestamp, "Update end pasted");

        _syncTau();

        uint256 tauDelta = lastTau > newTargetTau
            ? lastTau - newTargetTau
            : newTargetTau - lastTau;

        tauUpdatePerSecond =
            tauDelta / (newTauUpdateEnd - block.timestamp);
        targetTau = newTargetTau;
        tauUpdateEnd = newTauUpdateEnd;
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
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

    function getSpotPrice() public view returns (uint256) {
        return
            computeSpotPrice(reserveX, totalLiquidity, strikePrice(), sigma(), tau());
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
        return computeInvariant(reserveX, totalLiquidity, reserveY, strikePrice());
    }

    function logData() external {
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    function getStrategyData() external view returns (bytes memory data) {
        return abi.encode(sigma(), strikePrice(), tau());
    }
}
