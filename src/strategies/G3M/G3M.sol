// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../interfaces/IDFMM.sol";
import "../../interfaces/IStrategy.sol";
import "../../lib/DynamicParamLib.sol";
import "./G3MLib.sol";
import "./G3MHelper.sol";

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

    /// @dev Parameterization of the G3M curve.
    struct G3MParams {
        uint256 wX;
        uint256 wY;
        uint256 swapFee;
    }

    address public immutable dfmm;

    mapping(uint256 => InternalParams) public internalParams;

    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    // TODO: Move these errors into an interface
    error InvalidWeightX();

    modifier onlyDFMM() {
        if (msg.sender != address(dfmm)) revert NotDFMM();
        _;
    }

    /// @dev Decodes and validates pool initialization parameters.
    /// Sets the `slot` state variable.
    function init(
        address,
        uint256 poolId,
        bytes calldata data
    )
        external
        onlyDFMM
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
        internalParams[poolId].swapFee = swapFee;

        invariant = tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (G3MParams))
        );

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function validateAllocateOrDeallocate(
        address,
        uint256 poolId,
        bytes calldata data
    )
        external
        view
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
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (G3MParams))
        );

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @dev Reverts if the caller is not a contract with the Core interface.
    function validateSwap(
        address,
        uint256 poolId,
        bytes memory data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            int256 liquidityDelta,
            uint256 nextRx,
            uint256 nextRy,
            uint256 nextL
        )
    {
        G3MParams memory params = abi.decode(getPoolParams(poolId), (G3MParams));

        (uint256 startRx, uint256 startRy, uint256 startL) =
            IDFMM(dfmm).getReservesAndLiquidity(poolId);

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

        uint256 poolId = poolId;
        liquidityDelta = int256(nextL)
            - int256(
                computeNextLiquidity(
                    startRx, startRy, abi.decode(getPoolParams(poolId), (G3MParams))
                )
            );
        invariant = tradingFunction(nextRx, nextRy, nextL, params);
        bool validSwapConstant = -(EPSILON) < invariant && invariant < EPSILON;
        valid = validSwapConstant && liquidityDelta >= int256(minLiquidityDelta);
    }

    function update(uint256 poolId, bytes calldata data) external onlyDFMM {
        G3MUpdateCode updateCode = abi.decode(data, (G3MUpdateCode));

        if (updateCode == G3MUpdateCode.SwapFee) {
            internalParams[poolId].swapFee = decodeFeeUpdate(data);
        } else if (updateCode == G3MUpdateCode.WeightX) {
            (uint256 targetWeightX, uint256 targetTimestamp) =
                decodeWeightXUpdate(data);
            internalParams[poolId].wX.set(targetWeightX, targetTimestamp);
        } else {
            revert InvalidUpdateCode();
        }
    }

    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        G3MParams memory params;

        params.wX = internalParams[poolId].wX.actualized();
        params.wY = ONE - params.wX;
        params.swapFee = internalParams[poolId].swapFee;

        return abi.encode(params);
    }

    /// @dev Computes the result of the tradingFunction().
    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        return tradingFunction(
            rx, ry, L, abi.decode(getPoolParams(poolId), (G3MParams))
        );
    }
}
