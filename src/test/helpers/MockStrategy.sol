// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../interfaces/IDFMM.sol";
import "../../interfaces/IStrategy.sol";

contract MockStrategy is IStrategy {
    address public immutable dfmm;

    constructor(address dfmm_) {
        dfmm = dfmm_;
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

    function update(uint256 poolId, bytes calldata data) external { }

    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256) { }

    function getPoolParams(uint256 poolId)
        external
        view
        returns (bytes memory params)
    { }
}
