// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./lib/RMMMath.sol";

contract RMM {
    address public tokenX;
    address public tokenY;
    uint256 public reserveX;
    uint256 public reserveY;

    constructor(address tokenX_, address tokenY_) {
        tokenX = tokenX_;
        tokenY = tokenY_;
    }
}
