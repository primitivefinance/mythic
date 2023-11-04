// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    event Update(uint value);

    function setNumber(uint256 newNumber) public {
        number = newNumber;
        emit Update(newNumber);
    }

    function increment() public {
        number++;
        emit Update(number);
    }
}
