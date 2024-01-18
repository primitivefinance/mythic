// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./G3M.sol";

enum UpdateCode {
    Invalid,
    SwapFee,
    WeightX
}

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(UpdateCode.SwapFee, uint256(swapFee));
}

function decodeFeeUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 swapFee) = abi.decode(data, (UpdateCode, uint256));
    return swapFee;
}

function encodeWeightXUpdate(
    uint256 targetWeightX,
    uint256 targetTimestamp
) pure returns (bytes memory data) {
    return abi.encode(UpdateCode.WeightX, targetWeightX, targetTimestamp);
}

function decodeWeightXUpdate(bytes memory data)
    pure
    returns (uint256 targetWeightX, uint256 targetTimestamp)
{
    (, targetWeightX, targetTimestamp) =
        abi.decode(data, (UpdateCode, uint256, uint256));
}

contract G3MHelper {
    G3M public immutable g3m;

    constructor(address g3m_) {
        g3m = G3M(g3m_);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (G3M.PublicParams memory params)
    {
        bytes memory data = g3m.getPoolParams(poolId);
        params = abi.decode(data, (G3M.PublicParams));
    }

    function prepareFeeUpdate(uint256 swapFee)
        public
        pure
        returns (bytes memory data)
    {
        return encodeFeeUpdate(swapFee);
    }

    function prepareWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public pure returns (bytes memory data) {
        return abi.encode(uint8(1), targetWeightX, targetTimestamp);
    }
}
