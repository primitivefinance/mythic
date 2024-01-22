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
    {
        uint256 status = abi.decode(data, (uint256));

        if (status == 1) {
            valid = true;
            swapConstantGrowth = 1 ether;
            reserveX = 2 ether;
            reserveY = 3 ether;
            totalLiquidity = 4 ether;
        }

        if (status == 2) {
            valid = true;
            swapConstantGrowth = 1 ether;
            reserveX = 100 ether;
            reserveY = 100 ether;
            totalLiquidity = 10 ether;
        }
    }

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
    {
        uint256 status = abi.decode(data, (uint256));

        if (status == 1) {
            valid = true;
            invariant = 1 ether;
            reserveX = 50 ether;
            reserveY = 50 ether;
            totalLiquidity = 5 ether;
        } else if (status == 9) {
            valid = true;
            invariant = 1 ether;
            reserveX = 100 ether;
            reserveY = 120 ether;
            totalLiquidity = 10 ether;
        } else if (status == 8) {
            valid = true;
            invariant = 1 ether;
            reserveX = 120 ether;
            reserveY = 100 ether;
            totalLiquidity = 10 ether;
        }
    }

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
