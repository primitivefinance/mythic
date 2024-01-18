// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./LogNormal.sol";

enum UpdateCode {
    SwapFee
}

contract LogNormalHelper {
    LogNormal public immutable logNormal;

    constructor(address logNormal_) {
        logNormal = LogNormal(logNormal_);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (LogNormal.PublicParams memory params)
    {
        bytes memory data = logNormal.getPoolParams(poolId);
        params = abi.decode(data, (LogNormal.PublicParams));
    }

    function prepareFeeUpdate(uint256 swapFee)
        public
        pure
        returns (bytes memory data)
    {
        return abi.encode(uint8(0), uint256(swapFee));
    }
}
