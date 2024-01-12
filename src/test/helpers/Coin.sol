/// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";

contract Coin is ERC20 {
    constructor(uint256 initialSupply) ERC20("Coin", "COIN", 18) {
        _mint(msg.sender, initialSupply);
    }
}
