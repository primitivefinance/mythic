// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "../lib/lognormal/LogNormalLib.sol";
import "forge-std/console2.sol";

import "../interfaces/IMultiCore.sol";
import "../interfaces/IMultiStrategy.sol";

struct Sigma {
    uint256 target;
    uint256 last;
    uint256 updateEnd;
    uint256 updatePerSecond;
    uint256 lastSync;
}

struct Strike {
    uint256 target;
    uint256 last;
    uint256 updateEnd;
    uint256 updatePerSecond;
    uint256 lastSync;
}

struct Tau {
    uint256 target;
    uint256 last;
    uint256 updateEnd;
    uint256 updatePerSecond;
    uint256 lastSync;
}

/// @notice Log Normal has three variable parameters:
/// K - strike price
/// sigma - volatility
/// tau - time to expiry
///
/// Swaps are validated by the trading function:
/// Gaussian.ppf(x / L) + Gaussian.ppf(y / KL) = -sigma * sqrt(tau)
contract LogNormal is IMultiStrategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    IMultiCore public core;
    uint256 public swapFee;

    mapping(uint256 => LogNormParameters) public slots;

    mapping(uint256 => Sigma) public sigmas;
    mapping(uint256 => Strike) public strikes;
    mapping(uint256 => Tau) public taus;

    constructor(address _core, uint256 _swapFee) {
        require(_swapFee < ONE, "swap fee percentage must be less than 100%");
        swapFee = _swapFee;
        core = IMultiCore(_core);
    }

    modifier onlyCore() {
        // require(msg.sender == address(core), "only core");
        _;
    }

    /// @dev Returns the original parameters that were used to initialize the pool.
    function staticSlot(uint256 poolId)
        public
        view
        returns (LogNormParameters memory)
    {
        return slots[poolId];
    }

    /// @dev Slot holds out parameters, these return the dyanmic parameters.
    function dynamicSlot(uint256 poolId)
        public
        view
        returns (bytes memory params)
    {
        return abi.encode(strikePrice(poolId), sigma(poolId), tau(poolId));
    }

    function dynamicSlotInternal(uint256 poolId)
        public
        view
        returns (LogNormParameters memory params)
    {
        (params.strike, params.sigma, params.tau) =
            (strikePrice(poolId), sigma(poolId), tau(poolId));
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        return core.getReservesAndLiquidity(poolId);
    }

    function _syncDynamicSlot(uint256 poolId) internal {
        LogNormParameters memory params = slots[poolId];

        Sigma memory newSigma;
        Strike memory newStrike;
        Tau memory newTau;

        newSigma.target = params.sigma;
        newSigma.last = params.sigma;
        newSigma.updateEnd = block.timestamp;
        newSigma.lastSync = block.timestamp;

        newStrike.target = params.strike;
        newStrike.last = params.strike;
        newStrike.updateEnd = block.timestamp;
        newStrike.lastSync = block.timestamp;

        newTau.target = params.tau;
        newTau.last = params.tau;
        newTau.updateEnd = block.timestamp;
        newTau.lastSync = block.timestamp;

        sigmas[poolId] = newSigma;
        strikes[poolId] = newStrike;
        taus[poolId] = newTau;
    }

    /// @dev Computes the result of the tradingFunction().
    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) public view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        return tradingFunction({
            rx: rx,
            ry: ry,
            L: L,
            params: dynamicSlotInternal(poolId)
        });
    }

    /// @dev Decodes and validates pool initialization parameters.
    /// Sets the `slot` state variable.
    function init(
        uint256 poolId,
        bytes calldata data
    )
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
        (rx, ry, L, slots[poolId]) =
            abi.decode(data, (uint256, uint256, uint256, LogNormParameters));

        _syncDynamicSlot(poolId);

        invariant = tradingFunction({
            rx: rx,
            ry: ry,
            L: L,
            params: dynamicSlotInternal(poolId)
        });

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function validateAllocateOrDeallocate(
        uint256 poolId,
        bytes calldata data
    )
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
            params: dynamicSlotInternal(poolId)
        });

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @dev Reverts if the caller is not a contract with the Core interface.
    function validateSwap(
        uint256 poolId,
        bytes memory data
    )
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
            getReservesAndLiquidity(poolId);

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

        LogNormParameters memory params = dynamicSlotInternal(poolId);

        invariant = tradingFunction({
            rx: nextRx,
            ry: nextRy,
            L: nextL,
            params: params
        });

        bool validSwapConstant = -(EPSILON) < invariant && invariant < EPSILON;
        valid = validSwapConstant && liquidityDelta >= int256(minLiquidityDelta);
    }

    // ===== LogNormParameters ===== //

    function sigma(uint256 poolId) public view returns (uint256) {
        Sigma memory _sigma = sigmas[poolId];
        if (block.timestamp >= _sigma.updateEnd) {
            return _sigma.target;
        }

        return _sigma.last > _sigma.target
            ? _sigma.last
                - (block.timestamp - _sigma.lastSync) * _sigma.updatePerSecond
            : _sigma.last
                + (block.timestamp - _sigma.lastSync) * _sigma.updatePerSecond;
    }

    function strikePrice(uint256 poolId) public view returns (uint256) {
        Strike memory strike = strikes[poolId];
        if (block.timestamp >= strike.updateEnd) {
            return strike.target;
        }

        return strike.last > strike.target
            ? strike.last
                - (block.timestamp - strike.lastSync) * strike.updatePerSecond
            : strike.last
                + (block.timestamp - strike.lastSync) * strike.updatePerSecond;
    }

    function tau(uint256 poolId) public view returns (uint256) {
        Tau memory _tau = taus[poolId];
        if (block.timestamp >= _tau.updateEnd) {
            return _tau.target;
        }

        return _tau.last > _tau.target
            ? _tau.last - (block.timestamp - _tau.lastSync) * _tau.updatePerSecond
            : _tau.last + (block.timestamp - _tau.lastSync) * _tau.updatePerSecond;
    }

    function getParams(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        return (strikePrice(poolId), sigma(poolId), tau(poolId));
    }

    function _syncSigma(uint256 poolId) private {
        Sigma memory newSigma = sigmas[poolId];
        newSigma.last = sigma(poolId);
        newSigma.lastSync = block.timestamp;
        sigmas[poolId] = newSigma;
    }

    function _syncStrike(uint256 poolId) private {
        Strike memory newStrike = strikes[poolId];
        newStrike.last = strikePrice(poolId);
        newStrike.lastSync = block.timestamp;
        strikes[poolId] = newStrike;
    }

    function _syncTau(uint256 poolId) private {
        Tau memory newTau = taus[poolId];
        newTau.last = tau(poolId);
        newTau.lastSync = block.timestamp;
        taus[poolId] = newTau;
    }

    event SetSigma(
        uint256 targetSigma,
        uint256 lastSigma,
        uint256 sigmaUpdateEnd,
        uint256 delta
    );

    function setSigma(
        uint256 poolId,
        uint256 newTargetSigma,
        uint256 newSigmaUpdateEnd
    ) external {
        require(newSigmaUpdateEnd > block.timestamp, "Update end passed");

        _syncSigma(poolId);

        Sigma memory _sigma = sigmas[poolId];

        uint256 sigmaDelta = _sigma.last > newTargetSigma
            ? _sigma.last - newTargetSigma
            : newTargetSigma - _sigma.last;

        _sigma.updatePerSecond =
            sigmaDelta / (newSigmaUpdateEnd - block.timestamp);
        _sigma.target = newTargetSigma;
        _sigma.updateEnd = newSigmaUpdateEnd;
        sigmas[poolId] = _sigma;
        emit SetSigma(
            _sigma.target,
            _sigma.last,
            _sigma.updateEnd,
            _sigma.target > _sigma.last
                ? _sigma.target - _sigma.last
                : _sigma.last - _sigma.target
        );
    }

    event SetStrikePrice(
        uint256 targetStrike,
        uint256 lastStrike,
        uint256 strikeUpdateEnd,
        uint256 delta
    );

    function setStrikePrice(
        uint256 poolId,
        uint256 newTargetStrike,
        uint256 newStrikeUpdateEnd
    ) external {
        require(newStrikeUpdateEnd > block.timestamp, "Update end passed");

        _syncStrike(poolId);

        Strike memory strike = strikes[poolId];

        uint256 strikeDelta = strike.last > newTargetStrike
            ? strike.last - newTargetStrike
            : newTargetStrike - strike.last;

        strike.updatePerSecond =
            strikeDelta / (newStrikeUpdateEnd - block.timestamp);
        strike.target = newTargetStrike;
        strike.updateEnd = newStrikeUpdateEnd;
        strikes[poolId] = strike;
        emit SetStrikePrice(
            strike.target,
            strike.last,
            strike.updateEnd,
            strike.target > strike.last
                ? strike.target - strike.last
                : strike.last - strike.target
        );
    }

    event SetTau(
        uint256 targetTau, uint256 lastTau, uint256 tauUpdateEnd, uint256 delta
    );

    function setTau(
        uint256 poolId,
        uint256 newTargetTau,
        uint256 newTauUpdateEnd
    ) external {
        require(newTauUpdateEnd > block.timestamp, "Update end passed");

        _syncTau(poolId);

        Tau memory _tau = taus[poolId];

        uint256 tauDelta = _tau.last > newTargetTau
            ? _tau.last - newTargetTau
            : newTargetTau - _tau.last;

        _tau.updatePerSecond = tauDelta / (newTauUpdateEnd - block.timestamp);
        _tau.target = newTargetTau;
        _tau.updateEnd = newTauUpdateEnd;
        taus[poolId] = _tau;
        emit SetTau(
            _tau.target,
            _tau.last,
            _tau.updateEnd,
            _tau.target > _tau.last
                ? _tau.target - _tau.last
                : _tau.last - _tau.target
        );
    }
}
