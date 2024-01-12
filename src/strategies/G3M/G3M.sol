// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../interfaces/IMultiCore.sol";
import "../../interfaces/IStrategy.sol";
import "../../lib/DynamicParamLib.sol";
import "./G3MLib.sol";

/**
 * @notice Geometric Mean Market Maker.
 */
contract G3M is IStrategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using DynamicParamLib for DynamicParam;

    struct InternalParams {
        DynamicParam wX;
        uint256 swapFee;
    }

    IMultiCore public immutable core;

    mapping(uint256 => InternalParams) public internalParams;

    constructor(address _core) {
        core = IMultiCore(_core);
    }

    // TODO: Move these errors into an interface
    error NotCore();
    error InvalidWeightX();

    modifier onlyCore() {
        // if (msg.sender != address(core)) revert NotCore();
        _;
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
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        (valid, invariant, reserveX, reserveY, totalLiquidity,,) =
            _decodeInit(poolId, data);
    }

    function _decodeInit(
        uint256 poolId,
        bytes calldata data
    )
        private
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity,
            uint256 wX,
            uint256 swapFee
        )
    {
        (reserveX, reserveY, totalLiquidity, wX, swapFee) =
            abi.decode(data, (uint256, uint256, uint256, uint256, uint256));

        if (wX >= ONE) {
            revert InvalidWeightX();
        }

        internalParams[poolId].wX.lastComputedValue = wX;
        internalParams[poolId].wX.lastUpdateAt = block.timestamp;
        internalParams[poolId].swapFee = swapFee;

        invariant = tradingFunction(
            reserveX, reserveY, totalLiquidity, getPoolParams(poolId)
        );

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
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        (reserveX, reserveY, totalLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));

        invariant = tradingFunction(
            reserveX, reserveY, totalLiquidity, getPoolParams(poolId)
        );

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
        G3MParameters memory params = getPoolParams(poolId);

        (uint256 startRx, uint256 startRy, uint256 startL) =
            core.getReservesAndLiquidity(poolId);

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        uint256 amountIn;
        uint256 fees;
        uint256 minLiquidityDelta;

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

    function getPoolParams(uint256 poolId)
        public
        view
        returns (G3MParameters memory params)
    {
        params.wX = internalParams[poolId].wX.actualized();
        params.wY = ONE - params.wX;
        params.swapFee = internalParams[poolId].swapFee;
    }

    /*
    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        G3MParameters memory params;

        params.wX = internalParams[poolId].wX.actualized();
        params.wY = ONE - params.wX;
        params.swapFee = internalParams[poolId].swapFee;

        return abi.encode(params);
    }
    */

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        return core.getReservesAndLiquidity(poolId);
    }

    /// @dev Computes the result of the tradingFunction().
    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) public view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        return tradingFunction(rx, ry, L, getPoolParams(poolId));
    }
}
