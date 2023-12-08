// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "./IStrategy.sol";
import "./lib/RMMMath.sol";
import "./lib/BisectionLib.sol";
import "forge-std/console2.sol";

contract RMM is IStrategy {
    struct PoolParams {
        uint256 strike;
        uint256 sigma;
        uint256 tau;
    }

    event LogParameters(
        uint256 sigma, uint256 strikePrice, uint256 tau, uint256 blockTimestamp
    );

    error NextLiquidityConstantOutOfRange(
        bool swapDirection,
        uint256 amountIn,
        uint256 amountOut,
        int256 swapConstant
    );

    error XReserveOutOfRange(uint256 reserveX, int256 swapConstant);

    error YReserveOutOfRange(uint256 reserveY, int256 swapConstant);

    error LiquidityOutOfRange(uint256 liquidity, int256 swapConstant);

    error AmountOutConstantOutOfRange(
        bool swapDirection,
        uint256 amountIn,
        uint256 amountOut,
        int256 swapConstant
    );

    ERC20 public tokenX;
    ERC20 public tokenY;

    uint256 public swapFee;

    uint256 public reserveX;
    uint256 public reserveY;
    uint256 public totalLiquidity;

    mapping(address => uint256) public balanceOf;

    uint256 private lastSigma;
    uint256 public targetSigma;
    uint256 private lastSigmaSync;
    uint256 private sigmaUpdatePerSecond;
    uint256 private sigmaUpdateEnd;

    uint256 private lastStrike;
    uint256 public targetStrike;
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

        targetSigma = sigma_;
        lastSigma = sigma_;
        sigmaUpdateEnd = block.timestamp;
        lastSigmaSync = block.timestamp;

        targetStrike = strikePrice_;
        lastStrike = strikePrice_;
        strikeUpdateEnd = block.timestamp;
        lastStrikeSync = block.timestamp;

        targetTau = tau_;
        lastTau = tau_;
        tauUpdateEnd = block.timestamp;
        lastTauSync = block.timestamp;

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
        return (strikePrice(), sigma(), tau());
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

    function computeInitialPoolState(
        uint256 amountX,
        uint256 initialPrice
    )
        public
        view
        returns (uint256 initialX, uint256 initialY, uint256 initialLiquidity)
    {
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        uint256 baseLiquidity =
            computeLGivenX(amountX, initialPrice, _strike, _sigma, _tau);
        console2.log("base liquidity", baseLiquidity);
        initialY =
            computeYGivenL(baseLiquidity, initialPrice, _strike, _sigma, _tau);

        console2.log("initialY", initialY);
        initialLiquidity = computeNextLiquidity(
            amountX, initialY, baseLiquidity, _strike, _sigma, _tau
        );

        console2.log("initialLiquidity", initialLiquidity);
        initialX = amountX;
    }

    function initExactTokensAndLiquidity(
        uint256 amountX,
        uint256 amountY,
        uint256 liquidity
    ) external {
        require(totalLiquidity == 0, "Pool already initialized");

        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        int256 validLiquidity = checkSwapConstantNextLiquidity(
            amountX, amountY, liquidity, _strike, _sigma, _tau, liquidity
        );
        int256 validAmountX = checkSwapConstantNextReserveX(
            amountX, amountY, liquidity, _strike, _sigma, _tau, amountX
        );
        int256 validAmountY = checkSwapConstantNextReserveY(
            amountX, amountY, liquidity, _strike, _sigma, _tau, amountY
        );
        if (-25 > validLiquidity || validLiquidity > 25) {
            revert LiquidityOutOfRange(liquidity, validLiquidity);
        }
        if (-25 > validAmountX || validAmountX > 25) {
            revert XReserveOutOfRange(amountX, validAmountX);
        }
        if (-25 > validAmountY || validAmountY > 25) {
            revert YReserveOutOfRange(amountY, validAmountY);
        }

        totalLiquidity = liquidity;
        reserveX += amountX;
        reserveY += amountY;
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

    // when swapping compute the new L, then we want to get the correct amount out
    // we go back and bisect with the same fn except this time the free variable is the output token reserves
    // swapping in Y -> find the correct X
    // swapping in X -> find the correct Y
    // use the new L and validate that deltaL is positive
    function _swap(
        bool swapDirection,
        uint256 nextLiquidity,
        uint256 amountIn,
        uint256 amountOut
    ) internal returns (uint256) {
        PoolParams memory params;
        (params.strike, params.sigma, params.tau) = getParams();

        // validate nextLiquidity
        int256 swapConstant = checkSwapConstantNextLiquidity(
            reserveX,
            reserveY,
            totalLiquidity,
            params.strike,
            params.sigma,
            params.tau,
            nextLiquidity
        );

        if (-25 > swapConstant || swapConstant > 25) {
            revert NextLiquidityConstantOutOfRange(
                swapDirection, amountIn, amountOut, swapConstant
            );
        }

        uint256 price = computeSpotPrice(
            reserveX, nextLiquidity, params.strike, params.sigma, params.tau
        );

        if (swapDirection) {
            uint256 fees = amountIn * (ONE - swapFee) / ONE;
            console2.log("fees", fees);
            uint256 deltaL = computeLGivenX(
                fees, price, params.strike, params.sigma, params.tau
            );
            console2.log("deltaL", deltaL);

            int256 amountOutConstant = checkSwapConstantNextReserveY(
                reserveX + amountIn,
                reserveY,
                nextLiquidity + deltaL,
                params.strike,
                params.sigma,
                params.tau,
                reserveY - amountOut
            );

            if (-50 > amountOutConstant || amountOutConstant > 50) {
                revert AmountOutConstantOutOfRange(
                    swapDirection, amountIn, amountOut, amountOutConstant
                );
            }

            totalLiquidity = nextLiquidity + deltaL;
            reserveX += amountIn;
            reserveY -= amountOut;

            tokenX.transferFrom(msg.sender, address(this), amountIn);
            tokenY.transfer(msg.sender, amountOut);
        } else {
            uint256 fees = amountIn * (ONE - swapFee) / ONE;
            uint256 deltaL = computeLGivenY(
                fees, price, params.strike, params.sigma, params.tau
            );

            int256 amountOutConstant = checkSwapConstantNextReserveX(
                reserveX,
                reserveY + amountIn,
                nextLiquidity + deltaL,
                params.strike,
                params.sigma,
                params.tau,
                reserveX - amountOut
            );

            if (-25 > amountOutConstant || amountOutConstant > 25) {
                revert AmountOutConstantOutOfRange(
                    swapDirection, amountIn, amountOut, amountOutConstant
                );
            }

            totalLiquidity = nextLiquidity + deltaL;
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
            computeSpotPrice(
                reserveX,
                totalLiquidity,
                params.strike,
                params.sigma,
                params.tau
            )
        );

        return amountOut;
    }

    function getSwapConstantGivenLiquidity(uint256 newLiquidity)
        external
        view
        returns (int256)
    {
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        return checkSwapConstantNextLiquidity(
            reserveX,
            reserveY,
            totalLiquidity,
            _strike,
            _sigma,
            _tau,
            newLiquidity
        );
    }

    function getNextLiquidity() external view returns (uint256) {
        (uint256 _strike, uint256 _sigma, uint256 _tau) = getParams();
        return computeNextLiquidity(
            reserveX, reserveY, totalLiquidity, _strike, _sigma, _tau
        );
    }

    function getDeltaL(
        bool swapDirection,
        uint256 nextLiquidity,
        uint256 amountIn
    ) external view returns (uint256) {
        PoolParams memory params;
        (params.strike, params.sigma, params.tau) = getParams();
        uint256 price = computeSpotPrice(
            reserveX, nextLiquidity, params.strike, params.sigma, params.tau
        );
        uint256 fees = amountIn * (ONE - swapFee) / ONE;

        if (swapDirection) {
            return computeLGivenX(
                fees, price, params.strike, params.sigma, params.tau
            );
        } else {
            return computeLGivenY(
                fees, price, params.strike, params.sigma, params.tau
            );
        }
    }

    function getAmountOut(
        bool swapDirection,
        uint256 nextLiquidity,
        uint256 amountIn
    ) external view returns (uint256) {
        PoolParams memory params;
        (params.strike, params.sigma, params.tau) = getParams();
        uint256 price = computeSpotPrice(
            reserveX, nextLiquidity, params.strike, params.sigma, params.tau
        );
        uint256 fees = amountIn * (ONE - swapFee) / ONE;

        if (swapDirection) {
            uint256 deltaL = computeLGivenX(
                fees, price, params.strike, params.sigma, params.tau
            );
            uint256 nextReserve = computeNextReserveY(
                reserveX + amountIn,
                reserveY,
                nextLiquidity + deltaL,
                params.strike,
                params.sigma,
                params.tau
            );
            return reserveY - nextReserve;
        } else {
            uint256 deltaL = computeLGivenY(
                fees, price, params.strike, params.sigma, params.tau
            );
            uint256 nextReserve = computeNextReserveX(
                reserveX,
                reserveY + amountIn,
                nextLiquidity + deltaL,
                params.strike,
                params.sigma,
                params.tau
            );
            return reserveX - nextReserve;
        }
    }

    function sigma() public view returns (uint256) {
        if (block.timestamp >= sigmaUpdateEnd) {
            return targetSigma;
        }

        return lastSigma > targetSigma
            ? lastSigma - (block.timestamp - lastSigmaSync) * sigmaUpdatePerSecond
            : lastSigma + (block.timestamp - lastSigmaSync) * sigmaUpdatePerSecond;
    }

    function strikePrice() public view returns (uint256) {
        if (block.timestamp >= strikeUpdateEnd) {
            return targetStrike;
        }

        return lastStrike > targetStrike
            ? lastStrike
                - (block.timestamp - lastStrikeSync) * strikeUpdatePerSecond
            : lastStrike
                + (block.timestamp - lastStrikeSync) * strikeUpdatePerSecond;
    }

    function tau() public view returns (uint256) {
        if (block.timestamp >= tauUpdateEnd) {
            return targetTau;
        }

        return lastTau > targetTau
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

    function setSigma(
        uint256 newTargetSigma,
        uint256 newSigmaUpdateEnd
    ) external {
        require(newSigmaUpdateEnd > block.timestamp, "Update end passed");

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

    function setStrikePrice(
        uint256 newTargetStrike,
        uint256 newStrikeUpdateEnd
    ) external {
        require(newStrikeUpdateEnd > block.timestamp, "Update end passed");

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
        require(newTauUpdateEnd > block.timestamp, "Update end passed");

        _syncTau();

        uint256 tauDelta = lastTau > newTargetTau
            ? lastTau - newTargetTau
            : newTargetTau - lastTau;

        tauUpdatePerSecond = tauDelta / (newTauUpdateEnd - block.timestamp);
        targetTau = newTargetTau;
        tauUpdateEnd = newTauUpdateEnd;
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    function swapAmountIn(
        bool swapDirection,
        uint256 nextLiquidity,
        uint256 amountIn,
        uint256 amountOut
    ) external returns (uint256) {
        return _swap(swapDirection, nextLiquidity, amountIn, amountOut);
    }

    function swap(
        bool swapDirection,
        uint256 nextLiquidity,
        uint256 amountIn,
        uint256 amountOut
    ) external returns (uint256) {
        return _swap(swapDirection, nextLiquidity, amountIn, amountOut);
    }

    function setSwapFee(uint256 newSwapFee) external {
        require(newSwapFee < ONE, "New swap fee too high");
        swapFee = newSwapFee;
    }

    function getSpotPrice() public view returns (uint256) {
        return computeSpotPrice(
            reserveX, totalLiquidity, strikePrice(), sigma(), tau()
        );
    }

    function getSpotPriceFromY() public view returns (uint256) {
        return computeSpotPriceGivenY(
            reserveY, totalLiquidity, strikePrice(), sigma()
        );
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
        return
            computeInvariant(reserveX, totalLiquidity, reserveY, strikePrice());
    }

    function logData() external {
        emit LogParameters(sigma(), strikePrice(), tau(), block.timestamp);
    }

    function getStrategyData() external view returns (bytes memory data) {
        return abi.encode(sigma(), strikePrice(), tau());
    }
}
