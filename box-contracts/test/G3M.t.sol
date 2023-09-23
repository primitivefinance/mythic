// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../src/G3M.sol";

contract G3MTest is Test {
    G3M public g3m;
    MockERC20 public tokenX;
    MockERC20 public tokenY;

    function setUp() public {
        tokenX = new MockERC20("TokenX", "X", 18);
        tokenY = new MockERC20("TokenY", "Y", 18);

        tokenX.mint(address(this), 2000 ether);
        tokenY.mint(address(this), 2000 ether);

        g3m = new G3M(address(tokenX), address(tokenY), 0.5 ether);

        tokenX.approve(address(g3m), 2000 ether);
        tokenY.approve(address(g3m), 2000 ether);
    }

    function test_computeInvariant() public {
        uint256 invariant =
            G3MMath.computeInvariant(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(invariant);
    }

    function test_computeSpotPrice() public {
        uint256 spotPrice =
            G3MMath.computeSpotPrice(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(spotPrice);
    }

    function test_initPool() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 invariant =
            G3MMath.computeInvariant(amountX, 0.5 ether, amountY, 0.5 ether);
        uint256 liquidity = g3m.initPool(amountX, amountY);

        assertEq(tokenX.balanceOf(address(g3m)), amountX);
        assertEq(tokenY.balanceOf(address(g3m)), amountY);
        assertEq(g3m.reserveX(), amountX * 10 ** 18);
        assertEq(g3m.reserveY(), amountY * 10 ** 18);
        assertEq(g3m.totalLiquidity(), liquidity);
        assertApproxEqRel(g3m.totalLiquidity(), invariant * 2, 1_000);
        assertEq(g3m.getSpotPrice(), 3 * FixedPoint.ONE);
    }

    function test_computeAmountInGivenExactLiquidity() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 liquidity = g3m.initPool(amountX, amountY);

        uint256 amountIn = G3MMath.computeAmountInGivenExactLiquidity(
            g3m.totalLiquidity(), liquidity, g3m.reserveX()
        );
        console.log(amountIn);
    }

    function test_addLiquidity() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;

        uint256 liquidity = g3m.initPool(initAmountX, initAmountY);

        (uint256 amountX, uint256 amountY) = g3m.addLiquidity(liquidity);
        assertEq(g3m.reserveX(), (initAmountX + amountX) * 10 ** 18);
        assertEq(g3m.reserveY(), (initAmountY + amountY) * 10 ** 18);
        assertEq(g3m.totalLiquidity(), liquidity * 2);
        assertEq(amountX, 750 ether);
        assertEq(amountY, 250 ether);
    }

    function test_computeAmountOutGivenExactLiquidity() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 liquidity = g3m.initPool(amountX, amountY);

        uint256 amountOut = G3MMath.computeAmountOutGivenExactLiquidity(
            g3m.totalLiquidity(), liquidity / 2, g3m.reserveX()
        );
        console.log(amountOut);
    }

    function test_removeLiquidity() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;

        uint256 liquidity = g3m.initPool(initAmountX, initAmountY);
        (uint256 amountX, uint256 amountY) = g3m.removeLiquidity(liquidity / 2);

        assertEq(g3m.reserveX(), (initAmountX * 10 ** 18) / 2);
        assertEq(g3m.reserveY(), (initAmountY * 10 ** 18) / 2);
        assertApproxEqRel(g3m.totalLiquidity(), liquidity / 2, 1_000);

        assertEq(amountX, initAmountX / 2);
        assertEq(amountY, initAmountY / 2);

        assertEq(tokenX.balanceOf(address(g3m)), initAmountX / 2);
        assertEq(tokenY.balanceOf(address(g3m)), initAmountY / 2);
    }

    function test_computeOutGivenIn() public {
        g3m.initPool(750 ether, 250 ether);
        uint256 amountOut = G3MMath.computeOutGivenIn(
            50 ether * FixedPoint.ONE,
            g3m.reserveX(),
            g3m.reserveY(),
            g3m.getPrimaryWeight(),
            g3m.getSecondaryWeight()
        );

        console.log(amountOut);
    }

    function test_swapAmountIn() public {
        g3m.initPool(750 ether, 250 ether);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether);
        console.log(
            G3MMath.computeInvariant(
                g3m.reserveX(),
                g3m.getPrimaryWeight(),
                g3m.reserveY(),
                g3m.getSecondaryWeight()
            )
        );
        uint256 amountOut = g3m.swapAmountIn(true, 50 ether);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether + 50 ether);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether - amountOut);
        console.log(
            G3MMath.computeInvariant(
                g3m.reserveX(),
                g3m.getPrimaryWeight(),
                g3m.reserveY(),
                g3m.getSecondaryWeight()
            )
        );
    }

    function test_swapAmountOut() public {
        g3m.initPool(750 ether, 250 ether);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether);
        uint256 amountIn = g3m.swapAmountOut(true, 50 ether);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether + amountIn);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether - 50 ether);
    }
}
