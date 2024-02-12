// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solmate/utils/FixedPointMathLib.sol";

using FixedPointMathLib for uint256;

function computeScalingFactor(address token) view returns (uint256) {
    uint256 decimals = ERC20(token).decimals();
    uint256 difference = 18 - decimals;
    return FixedPointMathLib.WAD * 10 ** difference;
}

function upscale(
    uint256 amount,
    uint256 scalingFactor
) pure returns (uint256) {
    return FixedPointMathLib.mulWadDown(amount, scalingFactor);
}

function downscaleDown(
    uint256 amount,
    uint256 scalingFactor
) pure returns (uint256) {
    return FixedPointMathLib.divWadDown(amount, scalingFactor);
}

function downscaleUp(
    uint256 amount,
    uint256 scalingFactor
) pure returns (uint256) {
    return FixedPointMathLib.divWadUp(amount, scalingFactor);
}
