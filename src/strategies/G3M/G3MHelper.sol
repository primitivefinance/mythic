// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./G3M.sol";

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
        return abi.encode(uint8(0), uint256(swapFee));
    }

    function prepareWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public pure returns (bytes memory data) {
        return abi.encode(uint8(1), targetWeightX, targetTimestamp);
    }
}
