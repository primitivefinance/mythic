// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./lib/RMMMath.sol";

contract RMM {
    uint256 public sigma;
    address public tokenX;
    address public tokenY;
    uint256 public reserveX;
    uint256 public reserveY;
    uint256 public liquidity;

    constructor(address tokenX_, address tokenY_, uint256 sigma_) {
        tokenX = tokenX_;
        tokenY = tokenY_;
        sigma = sigma_;
    }
}
