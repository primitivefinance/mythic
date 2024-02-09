// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MInitTest is G3MSetUp {
    using DynamicParamLib for DynamicParam;

    function test_G3M_init_SetInternalParams() public init {
        (DynamicParam memory wX, uint256 swapFee, address controller) =
            g3m.internalParams(POOL_ID);

        assertEq(wX.actualized(), defaultParams.wX);
        assertEq(swapFee, defaultParams.swapFee);
        assertEq(controller, defaultParams.controller);
    }

    function test_G3M_init_RevertsWhenInvalidWeightX() public {
        G3M.G3MParams memory params = G3M.G3MParams({
            wX: 1.1 ether,
            wY: 0.5 ether,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        bytes memory defaultInitialPoolData =
            computeInitialPoolData(defaultReserveX, defaultStrikePrice, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(g3m),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: defaultInitialPoolData
        });

        vm.expectRevert(G3M.InvalidWeightX.selector);
        dfmm.init(initParams);
    }

    function test_G3M_init_RevertsWhenSenderNotDFMM() public {
        vm.expectRevert(IStrategy.NotDFMM.selector);
        bytes memory empty;
        g3m.init(address(this), 0, empty);
    }
}
