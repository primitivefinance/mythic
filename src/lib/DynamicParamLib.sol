// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/utils/FixedPointMathLib.sol";

struct DynamicParam {
    uint256 target;
    uint256 last;
    uint256 updateEnd;
    uint256 updatePerSecond;
    uint256 lastSync;
}

library DynamicParamLib {
    using FixedPointMathLib for uint256;

    function actualized(DynamicParam memory param)
        internal
        view
        returns (uint256)
    {
        if (param.lastSync == param.updateEnd) {
            return param.last;
        }

        uint256 updateTo = block.timestamp > param.updateEnd
            ? param.updateEnd
            : block.timestamp;
        uint256 deltaTime = updateTo - param.lastSync;

        if (param.updatePerSecond > 0) {
            return param.last
                + deltaTime * param.updatePerSecond;
        } else {
            return param.last
                - deltaTime * param.updatePerSecond;
        }
    }

    function set(DynamicParam memory param, uint256 target, uint256 end) internal view returns (DynamicParam memory) {
      require(end > block.timestamp, "End must be greater than current block.timestamp");
      
      DynamicParam memory syncedParam = sync(param);

      uint256 delta = syncedParam.last > target
        ? syncedParam.last - target
        : target - syncedParam.last;

      syncedParam.updatePerSecond = delta / (end - block.timestamp);
      syncedParam.target = target;
      syncedParam.updateEnd = end;

      return syncedParam;
    }

    function sync(DynamicParam memory param) internal view returns (DynamicParam memory) {
        param.last = actualized(param);
        param.lastSync = block.timestamp;
        return param;
    }
}
