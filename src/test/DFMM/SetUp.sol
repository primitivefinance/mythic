// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../helpers/SetUp.sol";
import "../helpers/MockStrategy.sol";

contract DFMMSetUp is SetUp {
    MockStrategy strategy;
    uint256 public POOL_ID;

    function setUp() public override {
        SetUp.setUp();
        strategy = new MockStrategy(address(dfmm));
    }

    modifier init() {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: abi.encode(uint256(2))
        });

        (POOL_ID,,,) = dfmm.init(params);
        _;
    }
}
