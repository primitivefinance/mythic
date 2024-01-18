// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./LogNormal.sol";

enum UpdateCode {
    SwapFee,
    Strike,
    Sigma,
    Tau
}

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(UpdateCode.SwapFee, uint256(swapFee));
}

function decodeFeeUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 swapFee) = abi.decode(data, (UpdateCode, uint256));
    return swapFee;
}

function encodeStrikeUpdate(uint256 strike) pure returns (bytes memory) {
    return abi.encode(UpdateCode.Strike, uint256(strike));
}

function decodeStrikeUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 strike) = abi.decode(data, (UpdateCode, uint256));
    return strike;
}

function encodeSigmaUpdate(uint256 sigma) pure returns (bytes memory) {
    return abi.encode(UpdateCode.Sigma, uint256(sigma));
}

function decodeSigmaUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 sigma) = abi.decode(data, (UpdateCode, uint256));
    return sigma;
}

function encodeTauUpdate(uint256 tau) pure returns (bytes memory) {
    return abi.encode(UpdateCode.Tau, uint256(tau));
}

function decodeTauUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 tau) = abi.decode(data, (UpdateCode, uint256));
    return tau;
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
