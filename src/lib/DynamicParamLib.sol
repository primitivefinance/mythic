// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/utils/FixedPointMathLib.sol";

library DynamicParamLib {
    using FixedPointMathLib for uint256;

    struct DynamicParam {
        uint256 lastComputedValue;
        uint256 updateEnd;
        uint256 lastUpdateAt;
        int256 updatePerSecond;
    }

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
}
