// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "src/interfaces/IDFMM.sol";
import "src/interfaces/IStrategy.sol";
import "src/lib/DynamicParamLib.sol";
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
        address controller;
    }

    /// @dev Parameterization of the G3M curve.
    struct G3MParams {
        uint256 wX;
        uint256 wY;
        uint256 swapFee;
        address controller;
    }

    /// @inheritdoc IStrategy
    address public immutable dfmm;

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    // TODO: Move these errors into an interface
    error InvalidWeightX();

    /// @dev Restricts the caller to the DFMM contract.
    modifier onlyDFMM() {
        if (msg.sender != address(dfmm)) revert NotDFMM();
        _;
    }

    /// @inheritdoc IStrategy
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
        (valid, invariant, reserveX, reserveY, totalLiquidity,,,) =
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
            uint256 swapFee,
            address controller
        )
    {
        (reserveX, reserveY, totalLiquidity, wX, swapFee, controller) = abi
            .decode(data, (uint256, uint256, uint256, uint256, uint256, address));

        if (wX >= ONE) {
            revert InvalidWeightX();
        }

        internalParams[poolId].wX.lastComputedValue = wX;
        internalParams[poolId].swapFee = swapFee;
        internalParams[poolId].controller = controller;

        invariant = tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (G3MParams))
        );

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @inheritdoc IStrategy
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

    /// @inheritdoc IStrategy
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

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        invariant = tradingFunction(nextRx, nextRy, nextL, params);
        console2.log(invariant);
        // todo: remove this
        liquidityDelta = int256(nextL);
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @inheritdoc IStrategy
    function update(
        address sender,
        uint256 poolId,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        G3MUpdateCode updateCode = abi.decode(data, (G3MUpdateCode));

        if (updateCode == G3MUpdateCode.SwapFee) {
            internalParams[poolId].swapFee = decodeFeeUpdate(data);
        } else if (updateCode == G3MUpdateCode.WeightX) {
            (uint256 targetWeightX, uint256 targetTimestamp) =
                decodeWeightXUpdate(data);
            internalParams[poolId].wX.set(targetWeightX, targetTimestamp);
        } else if (updateCode == G3MUpdateCode.Controller) {
            internalParams[poolId].controller = decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    /// @inheritdoc IStrategy
    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        G3MParams memory params;

        params.wX = internalParams[poolId].wX.actualized();
        params.wY = ONE - params.wX;
        params.swapFee = internalParams[poolId].swapFee;
        params.controller = internalParams[poolId].controller;

        return abi.encode(params);
    }

    /// @inheritdoc IStrategy
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
