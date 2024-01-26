// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./LogNormal.sol";

enum LogNormalUpdateCode {
    Invalid,
    SwapFee,
    Strike,
    Sigma,
    Tau,
    Controller
}

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.SwapFee, uint256(swapFee));
}

function decodeFeeUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 swapFee) = abi.decode(data, (LogNormalUpdateCode, uint256));
    return swapFee;
}

function encodeStrikeUpdate(
    uint256 targetStrike,
    uint256 targetTimestamp
) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.Strike, targetStrike, targetTimestamp);
}

function decodeStrikeUpdate(bytes memory data)
    pure
    returns (uint256 targetStrike, uint256 targetTimestamp)
{
    (, targetStrike, targetTimestamp) =
        abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
}

function encodeSigmaUpdate(
    uint256 targetSigma,
    uint256 targetTimestamp
) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.Sigma, targetSigma, targetTimestamp);
}

function decodeSigmaUpdate(bytes memory data)
    pure
    returns (uint256 targetSigma, uint256 targetTimestamp)
{
    (, targetSigma, targetTimestamp) =
        abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
}

function encodeTauUpdate(
    uint256 targetTau,
    uint256 targetTimestamp
) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.Tau, targetTau, targetTimestamp);
}

function decodeTauUpdate(bytes memory data)
    pure
    returns (uint256 targetTau, uint256 targetTimestamp)
{
    (, targetTau, targetTimestamp) =
        abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
}

function encodeControllerUpdate(uint256 controller)
    pure
    returns (bytes memory data)
{
    return abi.encode(LogNormalUpdateCode.Controller, controller);
}

function decodeControllerUpdate(bytes memory data)
    pure
    returns (uint256 controller)
{
    (, controller) = abi.decode(data, (LogNormalUpdateCode, uint256));
}

contract LogNormalHelper {
    LogNormal public immutable logNormal;

    constructor(address logNormal_) {
        logNormal = LogNormal(logNormal_);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (LogNormal.LogNormalParams memory params)
    {
        bytes memory data = logNormal.getPoolParams(poolId);
        params = abi.decode(data, (LogNormal.LogNormalParams));
    }

    function prepareFeeUpdate(uint256 swapFee)
        external
        pure
        returns (bytes memory)
    {
        return encodeFeeUpdate(swapFee);
    }

    function prepareStrikeUpdate(
        uint256 targetStrike,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return encodeStrikeUpdate(targetStrike, targetTimestamp);
    }

    function prepareSigmaUpdate(
        uint256 targetSigma,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return encodeSigmaUpdate(targetSigma, targetTimestamp);
    }

    function prepareTauUpdate(
        uint256 targetTau,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return encodeTauUpdate(targetTau, targetTimestamp);
    }

    function prepareControllerUpdate(uint256 controller)
        external
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(controller);
    }
}
