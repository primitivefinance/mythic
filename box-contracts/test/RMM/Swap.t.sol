// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.t.sol";

contract RMMSwap is RMMSetUp {
    function test_rmm_swap_UpdatesReserves() public {
        uint256 deltaX = 500 ether;

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 preReserveX = rmm.reserveX();
        uint256 preReserveY = rmm.reserveY();

        uint256 amountY = rmm.swapAmountIn(true, 500 ether);

        uint256 postReserveX = rmm.reserveX();
        uint256 postReserveY = rmm.reserveY();

        assertEq(preReserveX + deltaX, postReserveX);
        assertEq(preReserveY - amountY, postReserveY);
    }

    function test_rmm_swap_UpdatesRMMBalances() public {
        uint256 deltaX = 500 ether;

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 preBalanceX = tokenX.balanceOf(address(rmm));
        uint256 preBalanceY = tokenY.balanceOf(address(rmm));

        uint256 amountY = rmm.swapAmountIn(true, 500 ether);

        uint256 postBalanceX = tokenX.balanceOf(address(rmm));
        uint256 postBalanceY = tokenY.balanceOf(address(rmm));

        assertEq(preBalanceX + deltaX, postBalanceX);
        assertEq(preBalanceY - amountY, postBalanceY);
    }

    function test_rmm_swap_UpdatesSenderBalances() public {
        uint256 deltaX = 500 ether;

        rmm.initExactX(5_000 ether, initialPrice);

        uint256 preBalanceX = tokenX.balanceOf(address(this));
        uint256 preBalanceY = tokenY.balanceOf(address(this));

        uint256 amountY = rmm.swapAmountIn(true, 500 ether);

        uint256 postBalanceX = tokenX.balanceOf(address(this));
        uint256 postBalanceY = tokenY.balanceOf(address(this));

        assertEq(preBalanceX - deltaX, postBalanceX);
        assertEq(preBalanceY + amountY, postBalanceY);
    }

    function test_int() public {
        console2.logUint(uint256(bytes32("ffffffffffffffffffffffffffffffffffffffffffffffffff8d2693ed22ec91")));
    }
}
