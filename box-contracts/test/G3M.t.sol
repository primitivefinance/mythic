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

        tokenX.mint(address(this), 20000 ether);
        tokenY.mint(address(this), 20000 ether);

        g3m = new G3M(address(tokenX), address(tokenY), 0.5 ether);

        tokenX.approve(address(g3m), 20000 ether);
        tokenY.approve(address(g3m), 20000 ether);
    }

    function test_constructor_max_weight() public {
        g3m = new G3M(address(tokenX), address(tokenY), MAX_WEIGHT);
        assertEq(g3m.weightX(), MAX_WEIGHT);
        assertEq(g3m.weightY(), MIN_WEIGHT);
    }

    function test_constructor_min_weight() public {
        g3m = new G3M(address(tokenX), address(tokenY), MIN_WEIGHT);
        assertEq(g3m.weightX(), MIN_WEIGHT);
        assertEq(g3m.weightY(), MAX_WEIGHT);
    }

    function test_constructor_weights(uint256 weightX) public {
        vm.assume(weightX >= MIN_WEIGHT && weightX <= MAX_WEIGHT);
        assertEq(g3m.weightX() + g3m.weightY(), FixedPoint.ONE);
        assertEq(MIN_WEIGHT + MAX_WEIGHT, FixedPoint.ONE);
    }

    function test_constructor_reverts_invalid_tokens() public {
        vm.expectRevert("Invalid tokens");
        g3m = new G3M(address(tokenX), address(tokenX), 0.5 ether);
    }

    function test_constructor_reverts_invalid_max_weight() public {
        vm.expectRevert("Weight X too high");
        g3m = new G3M(address(tokenX), address(tokenY), MAX_WEIGHT + 1);
    }

    function test_constructor_reverts_invalid_min_weight() public {
        vm.expectRevert("Weight X too low");
        g3m = new G3M(address(tokenX), address(tokenY), MIN_WEIGHT - 1);
    }

    function test_updateWeightX(uint256 newWeightX) public {
        vm.assume(newWeightX >= MIN_WEIGHT && newWeightX <= MAX_WEIGHT);
        g3m.updateWeightX(newWeightX);
        assertEq(g3m.weightX(), newWeightX);
        uint256 newWeightY = FixedPoint.ONE - newWeightX;
        assertEq(g3m.weightY(), newWeightY);
    }

    function test_updateWeightX_reverts_not_admin() public {
        vm.expectRevert("Not admin");
        vm.prank(address(0xbeef));
        g3m.updateWeightX(0.5 ether);
    }

    function test_computeInvariant() public view {
        uint256 invariant =
            computeInvariantUp(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(invariant);
    }

    function test_computeSpotPrice() public view {
        uint256 spotPrice =
            computeSpotPrice(750 ether, 0.5 ether, 250 ether, 0.5 ether);
        console.log(spotPrice);
    }

    function test_initPool() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 invariant =
            computeInvariantDown(amountX, 0.5 ether, amountY, 0.5 ether);
        uint256 liquidity = g3m.initPool(amountX, amountY);

        assertEq(tokenX.balanceOf(address(g3m)), amountX);
        assertEq(tokenY.balanceOf(address(g3m)), amountY);
        assertEq(g3m.reserveX(), amountX * 10 ** 18);
        assertEq(g3m.reserveY(), amountY * 10 ** 18);
        assertEq(g3m.totalLiquidity(), liquidity + BURNT_LIQUIDITY);
        assertEq(g3m.getSpotPrice(), toWad(3));
        assertEq(g3m.balanceOf(address(this)), liquidity);
        assertEq(g3m.balanceOf(address(0)), 1_000);

        // TODO: Not a huge fan of using approx, let's see if we can use
        // something better here.
        assertApproxEqRel(g3m.totalLiquidity(), invariant * 2, 1_000);
        assertApproxEqRel(
            g3m.balanceOf(address(this)), invariant * 2 - BURNT_LIQUIDITY, 1_000
        );
    }

    function test_initPool_reverts_if_already_called() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;
        g3m.initPool(amountX, amountY);
        vm.expectRevert("Pool already initialized");
        g3m.initPool(amountX, amountY);
    }

    function test_computeAmountInGivenExactLiquidity() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 liquidity = g3m.initPool(amountX, amountY);

        uint256 amountIn = computeAmountInGivenExactLiquidity(
            g3m.totalLiquidity(), liquidity, g3m.reserveX()
        );
        console.log(amountIn);
    }

    function test_addLiquidity_reverts_if_pool_not_initialized() public {
        vm.expectRevert("Pool not initialized");
        g3m.addLiquidity(100 ether);
    }

    function test_addLiquidity() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;

        uint256 liquidity = g3m.initPool(initAmountX, initAmountY);

        (uint256 amountX, uint256 amountY) =
            g3m.addLiquidity(liquidity + BURNT_LIQUIDITY);
        assertEq(g3m.reserveX(), (initAmountX + amountX) * 10 ** 18);
        assertEq(g3m.reserveY(), (initAmountY + amountY) * 10 ** 18);
        assertEq(g3m.totalLiquidity(), ((liquidity + BURNT_LIQUIDITY) * 2));
        assertEq(amountX, 750 ether);
        assertEq(amountY, 250 ether);
    }

    function test_addLiquidity_maintains_spot_price() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        uint256 liquidity = g3m.initPool(initAmountX, initAmountY);
        uint256 oldSpotPrice = g3m.getSpotPrice();
        g3m.addLiquidity(liquidity);
        assertEq(g3m.getSpotPrice(), oldSpotPrice);
    }

    function test_computeAmountOutGivenExactLiquidity() public {
        uint256 amountX = 750 ether;
        uint256 amountY = 250 ether;

        uint256 liquidity = g3m.initPool(amountX, amountY);

        uint256 amountOut = computeAmountOutGivenExactLiquidity(
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

    function test_removeLiquidity_maintain_spot_price() public {
        uint256 initAmountX = 750 ether;
        uint256 initAmountY = 250 ether;
        uint256 liquidity = g3m.initPool(initAmountX, initAmountY);
        uint256 oldSpotPrice = g3m.getSpotPrice();
        g3m.removeLiquidity(liquidity / 2);
        assertEq(g3m.getSpotPrice(), oldSpotPrice);
    }

    function test_computeOutGivenIn() public {
        g3m.initPool(750 ether, 250 ether);
        uint256 amountOut = computeOutGivenIn(
            toWad(50 ether),
            g3m.reserveX(),
            g3m.reserveY(),
            g3m.weightX(),
            g3m.weightY()
        );

        console.log(amountOut);
    }

    function test_swapAmountIn() public {
        g3m.initPool(750 ether, 250 ether);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether);

        uint256 amountIn = 50 ether;
        uint256 amountOut = g3m.swapAmountIn(true, amountIn);
        assertEq(tokenX.balanceOf(address(g3m)), 750 ether + amountIn);
        assertEq(tokenY.balanceOf(address(g3m)), 250 ether - amountOut);
    }

    function test_swapAmountOut() public {
        uint256 liquidityX = 750 ether;
        uint256 liquidityY = 250 ether;
        g3m.initPool(liquidityX, liquidityY);

        uint256 initialBalanceX = tokenX.balanceOf(address(g3m));
        uint256 initialBalanceY = tokenY.balanceOf(address(g3m));
        assertEq(initialBalanceX, liquidityX);
        assertEq(initialBalanceY, liquidityY);

        uint256 amountOut = 15 ether;
        uint256 amountIn = g3m.swapAmountOut(true, amountOut);
        assertEq(tokenX.balanceOf(address(g3m)), initialBalanceX + amountIn);
        assertEq(tokenY.balanceOf(address(g3m)), initialBalanceY - amountOut);
    }
}
