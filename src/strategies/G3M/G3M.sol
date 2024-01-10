// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../interfaces/IMultiCore.sol";
import "../../interfaces/IMultiStrategy.sol";
import "./G3MLib.sol";

/**
 * @notice Geometric Mean Market Maker.
 */
struct WeightX {
    uint256 target;
    uint256 last;
    uint256 updateEnd;
    uint256 updatePerSecond;
    uint256 lastSync;
}

contract G3M is IMultiStrategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    IMultiCore public immutable core;

    mapping(uint256 => G3MParameters) public slots;
    mapping(uint256 => WeightX) public weights;

    constructor(address _core) {
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
        returns (G3MParameters memory)
    {
        return slots[poolId];
    }

    /// @dev Slot holds out parameters, these return the dyanmic parameters.
    function dynamicSlot(uint256 poolId) public view returns (bytes memory) {
        return abi.encode(weightX(poolId), weightY(poolId));
    }

    function dynamicSlotInternal(uint256 poolId)
        public
        view
        returns (G3MParameters memory params)
    {
        params.wx = weightX(poolId);
        params.wy = weightY(poolId);
        params.swapFee = slots[poolId].swapFee;
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        return core.getReservesAndLiquidity(poolId);
    }

    function _syncDynamicSlot(uint256 poolId) internal {
        G3MParameters memory params = slots[poolId];

        WeightX memory weight;

        weight.target = params.wx;
        weight.last = params.wx;
        weight.updateEnd = block.timestamp;
        weight.lastSync = block.timestamp;

        weights[poolId] = weight;
    }

    /// @dev Computes the result of the tradingFunction().
    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) public view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        return tradingFunction(rx, ry, L, dynamicSlotInternal(poolId));
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
            abi.decode(data, (uint256, uint256, uint256, G3MParameters));

        require(slots[poolId].wx + slots[poolId].wy == ONE, "Invalid weights");

        _syncDynamicSlot(poolId);

        invariant = tradingFunction(rx, ry, L, dynamicSlotInternal(poolId));

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
        G3MParameters memory params = slots[poolId];

        (uint256 startRx, uint256 startRy, uint256 startL) =
            getReservesAndLiquidity(poolId);

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        uint256 minLiquidityDelta;
        uint256 amountIn;
        uint256 fees;
        if (nextRx > startRx) {
            amountIn = nextRx - startRx;
            fees = amountIn.mulWadUp(params.swapFee);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRx);
        } else if (nextRy > startRy) {
            amountIn = nextRy - startRy;
            fees = amountIn.mulWadUp(params.swapFee);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRy);
        } else {
            revert("invalid swap: inputs x and y have the same sign!");
        }

        liquidityDelta = int256(nextL) - int256(startL);

        invariant = tradingFunction(nextRx, nextRy, nextL, params);

        bool validSwapConstant = -(EPSILON) < invariant && invariant < EPSILON;

        valid = validSwapConstant && liquidityDelta >= int256(minLiquidityDelta);
    }

    function weightX(uint256 poolId) public view returns (uint256) {
        WeightX memory weight = weights[poolId];
        if (block.timestamp >= weight.updateEnd) {
            return weight.target;
        }

        uint256 timeElapsed = block.timestamp - weight.lastSync;
        uint256 weightXDelta = timeElapsed * weight.updatePerSecond;

        if (weight.last > weight.target) {
            return weight.last - weightXDelta;
        } else {
            return weight.last + weightXDelta;
        }
    }

    function weightY(uint256 poolId) public view returns (uint256) {
        return 1 ether - weightX(poolId);
    }
}
