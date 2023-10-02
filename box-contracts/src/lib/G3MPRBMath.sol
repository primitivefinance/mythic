// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { UD60x18, ud, UNIT } from "@prb/math/UD60x18.sol";

/**
 * @dev Amount of liquidity burnt when a pool is initialized for the
 * first time. Main reason is mainly to avoid the case where the pool
 * gets totally drained and someone calls `initPool` again.
 * @custom:todo Check if the amount is correct?
 */
UD60x18 constant BURNT_LIQUIDITY = UD60x18.wrap(1_000);

/// @dev Current swap fee (expressed in 10,000).
uint256 constant SWAP_FEE = 30; // 0.3%

/// @dev Minimum weight of a token in the pool.
UD60x18 constant MIN_WEIGHT = UD60x18.wrap(0.01e18);

/// @dev Maximum weight of a token in the pool.
UD60x18 constant MAX_WEIGHT = UD60x18.wrap(990000000000000000);
