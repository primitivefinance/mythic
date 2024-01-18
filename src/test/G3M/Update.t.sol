/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MUpdateTest is G3MSetUp {
    function test_G3M_update_UpdatesSwapFee() public init {
        G3M.PublicParams memory params = helper.getPoolParams(POOL_ID);
        assertEq(params.swapFee, TEST_SWAP_FEE);

        uint256 newSwapFee = 0.004 ether;
        bytes memory data = helper.prepareFeeUpdate(newSwapFee);
        dfmm.update(POOL_ID, data);

        params = helper.getPoolParams(POOL_ID);
        assertEq(params.swapFee, newSwapFee);
    }
}
