// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "src/interfaces/IDFMM.sol";
import "src/interfaces/IStrategy.sol";
import "src/lib/DynamicParamLib.sol";
import "./ConstantSumLib.sol";
import "forge-std/Test.sol";

contract ConstantSum is IStrategy {
    using FixedPointMathLib for uint256;
    using DynamicParamLib for DynamicParam;

    struct InternalParams {
        uint256 price;
        uint256 swapFee;
        address controller;
    }

    struct ConstantSumParams {
        uint256 price;
        uint256 swapFee;
        address controller;
    }

    address public dfmm;

    mapping(uint256 => InternalParams) public internalParams;

    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    modifier onlyDFMM() {
        if (msg.sender != dfmm) revert NotDFMM();
        _;
    }

    function init(
        address,
        uint256 poolId,
        bytes calldata data
    )
        public
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        ConstantSumParams memory params;
        (reserveX, reserveY, totalLiquidity, params) =
            abi.decode(data, (uint256, uint256, uint256, ConstantSumParams));

        internalParams[poolId].price = params.price;
        internalParams[poolId].swapFee = params.swapFee;

        // Get the trading function and check this is valid
        invariant = ConstantSumLib.tradingFunction(
            reserveX, reserveY, totalLiquidity, params.price
        );

        valid = -EPSILON < invariant && invariant < EPSILON;

        return (valid, invariant, reserveX, reserveY, totalLiquidity);
    }

    function validateSwap(
        address,
        uint256 poolId,
        bytes calldata data
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
        ConstantSumParams memory params =
            abi.decode(getPoolParams(poolId), (ConstantSumParams));

        (uint256 startRx, uint256 startRy, uint256 startL) =
            IDFMM(dfmm).getReservesAndLiquidity(poolId);

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        uint256 minLiquidityDelta;
        uint256 amountIn;
        uint256 fees;
        if (nextRx > startRy) {
            amountIn = nextRx - startRx;
            console2.log("amountIn in validate: ", amountIn);
            fees = amountIn.mulWadUp(params.swapFee);
            console2.log("fees in validate: ", fees);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRx);
        } else if (nextRy > startRy) {
            amountIn = nextRy - startRy;
            console2.log("amountIn in validate: ", amountIn);
            fees = amountIn.mulWadUp(params.swapFee);
            console2.log("fees in validate: ", fees);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRy);
        } else {
            revert("invalid swap: inputs x and y have the same sign!");
        }

        liquidityDelta = int256(nextL) - int256(startL);
        assert(liquidityDelta >= int256(minLiquidityDelta));

        console2.log("liquidityDelta in validate: ", liquidityDelta);
        console2.log("price: ", params.price);
        invariant =
            ConstantSumLib.tradingFunction(nextRx, nextRy, nextL, params.price);

        console2.log("invariant in validate: ", invariant);
        valid = -EPSILON < invariant && invariant < EPSILON;
    }

    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256) {
        (uint256 reserveX, uint256 reserveY, uint256 totalLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));
        return ConstantSumLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (ConstantSumParams)).price
        );
    }

    // This should literally always work lol
    function validateAllocateOrDeallocate(
        address sender,
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
    { }

    function update(
        address sender,
        uint256 poolId,
        bytes calldata data
    ) external { }

    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        ConstantSumParams memory params;

        params.price = internalParams[poolId].price;
        params.swapFee = internalParams[poolId].swapFee;

        return abi.encode(params);
    }
}
