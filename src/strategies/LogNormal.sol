// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "../lib/lognormal/LogNormalLib.sol";

import "../interfaces/ICore.sol";
import "../interfaces/IStrategy.sol";

/// @notice Log Normal has three variable parameters:
/// K - strike price
/// sigma - volatility
/// tau - time to expiry
///
/// Swaps are validated by the trading function:
/// Gaussian.ppf(x / L) + Gaussian.ppf(y / KL) = -sigma * sqrt(tau)
contract LogNormal is IStrategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    ICore public core;
    uint256 public swapFee;
    LogNormParameters public __slot__;

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

    constructor(uint256 _swapFee) {
        require(_swapFee < ONE, "swap fee percentage must be less than 100%");
        swapFee = _swapFee;
        core = ICore(msg.sender);
    }

    modifier onlyCore() {
        // require(msg.sender == address(core), "only core");
        _;
    }

    /// @dev Returns the original parameters that were used to initialize the pool.
    function staticSlot() public view returns (LogNormParameters memory) {
        return __slot__;
    }

    /// @dev Slot holds out parameters, these return the dyanmic parameters.
    function dynamicSlot() public view returns (bytes memory params) {
        return abi.encode(strikePrice(), sigma(), tau());
    }

    function dynamicSlotInternal()
        public
        view
        returns (LogNormParameters memory params)
    {
        (params.strike, params.sigma, params.tau) =
            (strikePrice(), sigma(), tau());
    }

    function getReservesAndLiquidity()
        public
        view
        returns (uint256, uint256, uint256)
    {
        return core.getReservesAndLiquidity();
    }

    function _syncDynamicSlot() internal {
        LogNormParameters memory params = staticSlot();

        targetSigma = params.sigma;
        lastSigma = params.sigma;
        sigmaUpdateEnd = block.timestamp;
        lastSigmaSync = block.timestamp;

        targetStrike = params.strike;
        lastStrike = params.strike;
        strikeUpdateEnd = block.timestamp;
        lastStrikeSync = block.timestamp;

        targetTau = params.tau;
        lastTau = params.tau;
        tauUpdateEnd = block.timestamp;
        lastTauSync = block.timestamp;
    }

    /// @dev Computes the result of the tradingFunction().
    function computeSwapConstant(bytes memory data)
        public
        view
        returns (int256)
    {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        return tradingFunction({
            rx: rx,
            ry: ry,
            L: L,
            params: dynamicSlotInternal()
        });
    }

    /// @dev Decodes and validates pool initialization parameters.
    /// Sets the `slot` state variable.
    function init(bytes calldata data)
        public
        onlyCore
        returns (
            bool valid,
            int256 invariant,
            uint256 rx,
            uint256 ry,
            uint256 L
        )
    {
        (rx, ry, L, __slot__) =
            abi.decode(data, (uint256, uint256, uint256, LogNormParameters));

        _syncDynamicSlot();

        invariant = tradingFunction({
            rx: rx,
            ry: ry,
            L: L,
            params: dynamicSlotInternal()
        });

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function validateAllocationOrDeallocation(bytes calldata data)
        public
        view
        onlyCore
        returns (
            bool valid,
            int256 invariant,
            uint256 rx,
            uint256 ry,
            uint256 L
        )
    {
        (rx, ry, L) = abi.decode(data, (uint256, uint256, uint256));

        invariant = tradingFunction({
            rx: rx,
            ry: ry,
            L: L,
            params: dynamicSlotInternal()
        });

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @dev Reverts if the caller is not a contract with the Core interface.
    function validate(bytes memory data)
        public
        view
        onlyCore
        returns (
            bool valid,
            int256 invariant,
            int256 liquidityDelta,
            uint256 nextRx,
            uint256 nextRy,
            uint256 nextL
        )
    {
        (uint256 startRx, uint256 startRy, uint256 startL) =
            getReservesAndLiquidity();

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        // Rounding up is advantageous towards the protocol, to make sure swap fees
        // are fully paid for.
        uint256 minLiquidityDelta;
        uint256 amountIn;
        uint256 fees;
        if (nextRx > startRx) {
            amountIn = nextRx - startRx;
            fees = amountIn.mulWadUp(swapFee);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRx);
        } else if (nextRy > startRy) {
            // δl = δx * L / X, where δx = delta x * fee
            amountIn = nextRy - startRy;
            fees = amountIn.mulWadUp(swapFee);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRy);
        } else {
            // should never get here!
            revert("invalid swap: inputs x and y have the same sign!");
        }

        liquidityDelta = int256(nextL) - int256(startL);

        invariant = tradingFunction({
            rx: nextRx,
            ry: nextRy,
            L: nextL,
            params: dynamicSlotInternal()
        });

        bool validSwapConstant = -(EPSILON) < invariant && invariant < EPSILON;
        valid = validSwapConstant && liquidityDelta >= int256(minLiquidityDelta);
    }

    // ===== LogNormParameters ===== //

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

    function getParams() public view returns (uint256, uint256, uint256) {
        return (strikePrice(), sigma(), tau());
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

    event SetSigma(
        uint256 targetSigma,
        uint256 lastSigma,
        uint256 sigmaUpdateEnd,
        uint256 delta
    );

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
        emit SetSigma(
            targetSigma,
            lastSigma,
            sigmaUpdateEnd,
            targetSigma > lastSigma
                ? targetSigma - lastSigma
                : lastSigma - targetSigma
        );
    }

    event SetStrikePrice(
        uint256 targetStrike,
        uint256 lastStrike,
        uint256 strikeUpdateEnd,
        uint256 delta
    );

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
        emit SetStrikePrice(
            targetStrike,
            lastStrike,
            strikeUpdateEnd,
            targetStrike > lastStrike
                ? targetStrike - lastStrike
                : lastStrike - targetStrike
        );
    }

    event SetTau(
        uint256 targetTau, uint256 lastTau, uint256 tauUpdateEnd, uint256 delta
    );

    function setTau(uint256 newTargetTau, uint256 newTauUpdateEnd) external {
        require(newTauUpdateEnd > block.timestamp, "Update end passed");

        _syncTau();

        uint256 tauDelta = lastTau > newTargetTau
            ? lastTau - newTargetTau
            : newTargetTau - lastTau;

        tauUpdatePerSecond = tauDelta / (newTauUpdateEnd - block.timestamp);
        targetTau = newTargetTau;
        tauUpdateEnd = newTauUpdateEnd;
        emit SetTau(
            targetTau,
            lastTau,
            tauUpdateEnd,
            targetTau > lastTau ? targetTau - lastTau : lastTau - targetTau
        );
    }
}
