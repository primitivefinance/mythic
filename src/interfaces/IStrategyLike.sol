// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "../strategies/LogNormal/LogNormalLib.sol";
import "../strategies/G3M/G3MLib.sol";

interface IStrategyLike {
    function computeSwapConstant(
        uint256 poolId,
        bytes memory
    ) external view returns (int256);
    function swapFee() external view returns (uint256);
    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256, uint256, uint256);
    function validateSwap(
        uint256 poolId,
        bytes calldata
    ) external view returns (bool, int256, int256, uint256, uint256, uint256);
}

interface LogNormalStrategyLike {
    function dynamicSlotInternal(uint256 poolId)
        external
        view
        returns (LogNormParameters memory);
}

interface G3MStrategyLike {
    function dynamicSlotInternal(uint256 poolId)
        external
        view
        returns (G3mParameters memory);
}
