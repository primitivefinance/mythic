// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "src/interfaces/IStrategy.sol";
import "src/interfaces/IDFMM.sol";
import "src/lib/StrategyLib.sol";
import "src/strategies/ConstantSum/ConstantSum.sol";

contract ConstantSumSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    address public strategy;

    constructor(address strategy_) {
        strategy = strategy_;
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 ry,
        ConstantSum.ConstantSumParams memory params
    ) public pure returns (bytes memory) {
        // The pool can be initialized with any non-negative amount of rx, and ry.
        // so we have to allow a user to pass an amount of both even if one is zero.
        uint256 L = rx + ry.divWadUp(params.price);
        abi.encode(rx, ry, L, params.price, params.swapFee, params.controller);
    }
}
