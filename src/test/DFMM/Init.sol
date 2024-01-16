// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../DFMM.sol";
import "../../interfaces/IDFMM.sol";
import "./SetUp.sol";

contract DFMMInit is DFMMSetUp {
    function test_DFMM_init_StoresStrategyInitialReserves() public {
        bytes memory data = abi.encode(uint256(1));

        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(0xbeef),
            tokenY: address(0xdead),
            data: data
        });

        (uint256 poolId,,,) = dfmm.init(params);
        (uint256 reserveX, uint256 reserveY, uint256 totalLiquidity) =
            dfmm.getReservesAndLiquidity(poolId);

        assertEq(reserveX, 2 ether);
        assertEq(reserveY, 3 ether);
        assertEq(totalLiquidity, 4 ether);
    }

    event Init(
        address indexed account,
        address indexed strategy,
        uint256 poolId,
        uint256 reserveX,
        uint256 reserveY,
        uint256 totalLiquidity
    );

    function test_DFMM_init_EmitsInitEvent() public {
        bytes memory data = abi.encode(uint256(1));

        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(0xbeef),
            tokenY: address(0xdead),
            data: data
        });

        vm.expectEmit(true, true, true, true, address(dfmm));
        emit Init(
            address(this), address(strategy), 0, 2 ether, 3 ether, 4 ether
        );

        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenSameTokens() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(0xbeef),
            tokenY: address(0xbeef),
            data: ""
        });

        vm.expectRevert(IDFMM.InvalidTokens.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenNotValid() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(0xbeef),
            tokenY: address(0xdead),
            data: abi.encode(uint256(0))
        });

        vm.expectRevert(
            abi.encodeWithSelector(IDFMM.Invalid.selector, false, 0)
        );
        dfmm.init(params);
    }
}
