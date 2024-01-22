// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/utils/FixedPointMathLib.sol";

struct DynamicParam {
    uint256 lastComputedValue;
    uint256 updateEnd;
    int256 updatePerSecond;
    uint256 lastUpdateAt;
}

library DynamicParamLib {
    using FixedPointMathLib for uint256;

    error InvalidUpdateEnd();

    function actualized(DynamicParam memory param)
        internal
        view
        returns (uint256)
    {
        if (param.lastUpdateAt == param.updateEnd) {
            return param.lastComputedValue;
        }

        uint256 updateTo = block.timestamp > param.updateEnd
            ? param.updateEnd
            : block.timestamp;
        uint256 deltaTime = updateTo - param.lastUpdateAt;

        if (param.updatePerSecond > 0) {
            return param.lastComputedValue
                + deltaTime * uint256(param.updatePerSecond);
        } else {
            return param.lastComputedValue
                - deltaTime * uint256(-param.updatePerSecond);
        }
    }

    function sync(DynamicParam storage param) internal {
        param.lastComputedValue = actualized(param);
        param.lastUpdateAt = block.timestamp;
    }

    function set(
        DynamicParam storage param,
        uint256 target,
        uint256 updateEnd
    ) internal {
        if (updateEnd <= block.timestamp) revert InvalidUpdateEnd();
        sync(param);
        uint256 timeDelta = updateEnd - block.timestamp;
        int256 delta = int256(target) - int256(param.lastComputedValue);
        int256 deltaPerSecond = delta / int256(timeDelta);
        param.updateEnd = updateEnd;
        param.updatePerSecond = deltaPerSecond;
    }
}
