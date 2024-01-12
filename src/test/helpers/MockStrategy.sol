// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../interfaces/IMultiCore.sol";
import "../../interfaces/IMultiStrategy.sol";

abstract contract MockStrategy is IMultiStrategy {
    IMultiCore public immutable core;

    constructor(address _core) {
        core = IMultiCore(_core);
    }

    function init(
        uint256 poolId,
        bytes calldata data
    )
        external
        returns (
            bool valid,
            int256 swapConstantGrowth,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    { }

    function validateAllocateOrDeallocate(
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

    function validateSwap(
        uint256 poolId,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    { }

    function dynamicSlot(uint256 poolId)
        external
        view
        returns (bytes memory data)
    { }
}
