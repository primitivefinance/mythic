/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract LogNormalUpdateTest is LogNormalSetUp {
    function test_LogNormal_update_UpdatesSwapFee() public init {
        LogNormal.PublicParams memory params = helper.getPoolParams(POOL_ID);
        assertEq(params.swapFee, TEST_SWAP_FEE);

        uint256 newSwapFee = 0.004 ether;
        bytes memory data = helper.prepareFeeUpdate(newSwapFee);
        dfmm.update(POOL_ID, data);

        params = helper.getPoolParams(POOL_ID);
        assertEq(params.swapFee, newSwapFee);
    }
}
