/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";

contract LPToken is ERC20 {
    address public immutable dfmm;

    modifier OnlyDFMM() {
        require(msg.sender == dfmm, "OnlyDFMM");
        _;
    }

    constructor(
        string memory name_,
        string memory symbol_,
        uint256 amount
    ) ERC20(name_, symbol_, 18) {
        dfmm = msg.sender;
        _mint(msg.sender, amount);
    }

    function mint(address to, uint256 amount) external OnlyDFMM {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external OnlyDFMM {
        _burn(from, amount);
    }
}
