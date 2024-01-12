// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "../strategies/LogNormal/LogNormalLib.sol";
import "../strategies/G3M/G3MLib.sol";

interface LogNormalStrategyLike {
    function getPoolParams(uint256 poolId)
        external
        view
        returns (LogNormParameters memory);
}

interface G3MStrategyLike {
    function getPoolParams(uint256 poolId)
        external
        view
        returns (G3MParameters memory);
}
