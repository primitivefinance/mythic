// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

interface IParams {
    struct InitParams {
        uint256 poolId;
        address strategy;
        address tokenX;
        address tokenY;
        uint256 swapFeePercentageWad;
        bytes data;
    }
}
