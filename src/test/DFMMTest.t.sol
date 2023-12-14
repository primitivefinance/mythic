// SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.13;

// import "../v3/BisectionLib.sol";
// import "../DFMM.sol";
// import "forge-std/Test.sol";
// import "solmate/test/utils/mocks/MockERC20.sol";
// import "../AtomicV2.sol";
// import "../Lex.sol";

// contract DFMMTest is Test {
//     using stdStorage for StdStorage;

//     LogNormal source;
//     DFMM dfmm;
//     address tokenX;
//     address tokenY;
//     Lex lex;

//     uint256 public constant TEST_SWAP_FEE = 0.003e18;

//     function setUp() public {
//         tokenX = address(new MockERC20("tokenX", "X", 18));
//         tokenY = address(new MockERC20("tokenY", "Y", 18));
//         MockERC20(tokenX).mint(address(this), 100e18);
//         MockERC20(tokenY).mint(address(this), 100e18);

//         lex = new Lex(tokenX, tokenY, 1e18);

//         dfmm = new DFMM(tokenX, tokenY, TEST_SWAP_FEE);
//         source = LogNormal(dfmm.source());
//         MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
//         MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
//     }

//     /// @dev Initializes a basic pool in dfmm.
//     modifier basic() {
//         vm.warp(0);
//         Parameters memory params = Parameters({
//             strikePriceWad: 1 ether,
//             sigmaPercentWad: 1e18,
//             tauYearsWad: 1e18
//         });
//         uint256 init_p = 1e18;
//         uint256 init_x = 1e18;
//         uint256 init_l = LogNormal(source).lx(init_x, init_p, params);
//         uint256 init_y = LogNormal(source).yl(init_l, init_p, params);
//         int256 swapConstantInit = 0;
//         uint256 found_l = LogNormal(source).findLiquidity(
//             false, init_x, init_y, swapConstantInit, 0, params
//         );

//         bytes memory init_data =
//             LogNormal(source).encodeInitData(init_x, init_y, found_l, params);

//         dfmm.init(init_data);

//         console2.log("Initialized L:       ", found_l);
//         console2.log("Initialized X:       ", init_x);
//         console2.log("Initialized Y:       ", init_y);
//         console2.log("Initialized strike:  ", source.strikePrice());
//         console2.log("Initialized sigma:   ", source.sigma());
//         console2.log("Initialized tau:     ", source.tau());
//         _;
//     }

//     function test_dfmm_init() public {
//         Parameters memory params = Parameters({
//             strikePriceWad: 1e18,
//             sigmaPercentWad: 1e18,
//             tauYearsWad: 1e18
//         });
//         uint256 init_p = 1.0e18;
//         uint256 init_x = 0.5e18;

//         // note: adding 2 wei to the d1 value before calling cdf with these values will result in 0.5e18 y reserves...
//         // actually adding 1 wei to d2 in the y given l computation also yields 0.5e18 y reserves.
//         int256 d1 = LogNormal(source).computeD1(init_p, params);
//         console2.logInt(d1);
//         int256 cdfOfD1 = Gaussian.cdf(d1);
//         console2.logInt(cdfOfD1);

//         uint256 init_l = LogNormal(source).lx(init_x, init_p, params);
//         uint256 init_y = LogNormal(source).yl(init_l, init_p, params);
//         console2.log("init_l", init_l);
//         console2.log("init_y", init_y);
//         int256 swapConstantInit = 0;
//         uint256 found_l = LogNormal(source).findLiquidity(
//             false, init_x, init_y, swapConstantInit, 0, params
//         );
//         console2.log("found_l", found_l);

//         // This computation is slightly off (invariant won't be zero).
//         bytes memory init_data =
//             LogNormal(source).encodeInitData(init_x, init_y, found_l, params);

//         console2.log("init_x", init_x);
//         console2.log("init_y", init_y);
//         console2.log("init_l", init_l);

//         console2.log(MockERC20(tokenX).balanceOf(address(this)));
//         console2.log(MockERC20(tokenY).balanceOf(address(this)));

//         (uint256 x, uint256 y, uint256 l) = dfmm.init(init_data);
//         console2.log("x", x);
//         console2.log("y", y);
//         console2.log("l", l);

//         assertEq(x, init_x, "x != init x");
//         assertEq(y, init_y, "y != init y");
//         assertEq(l, found_l, "l != found l");
//         assertEq(dfmm.balanceOf(address(this)), found_l, "l != found l");
//         assertTrue(dfmm.inited(), "not initialized!");
//         assertEq(
//             MockERC20(tokenX).balanceOf(address(dfmm)),
//             init_x,
//             "x balance != init x"
//         );
//         assertEq(
//             MockERC20(tokenY).balanceOf(address(dfmm)),
//             init_y,
//             "y balance != init y"
//         );
//     }

//     function test_dfmm_swap_x_in() public basic {
//         uint256 feePercentageWad = source.swapFeePercentageWad();

//         // Get all the current data: reserves, liquidity, invariant, params.
//         int256 invariant = dfmm.getSwapConstant();
//         console2.logInt(invariant);

//         (uint256 reserveXWad, uint256 reserveYWad, uint256 liquidity) =
//             dfmm.getReservesAndLiquidity();

//         console2.log("reserveXWad", reserveXWad);
//         console2.log("reserveYWad", reserveYWad);
//         console2.log("liquidity", liquidity);

//         Parameters memory params = source.staticSlot();

//         // todo: should take this out.
//         uint256 price = 1.0e18;

//         // Compute the adjusted x reserve and adjusted liquidity.
//         uint256 amountXIn = 0.1 ether;

//         uint256 adjustedReserveX = reserveXWad + amountXIn;
//         console2.log("Submitted new X reserve", adjustedReserveX);

//         uint256 expectedLiquidityGrowth =
//             source.lx(amountXIn * feePercentageWad / 1 ether, price, params);

//         // Try commenting this line of code out, the test should fail!
//         expectedLiquidityGrowth += 1;

//         console2.log("expectedLiquidityGrowth", expectedLiquidityGrowth);

//         // So we compute the adjusted liquidity amount to find the y, but pass in the origina liquidity.
//         // So we need the contract to validate the expected growth.
//         uint256 adjustedLiquidity = liquidity + expectedLiquidityGrowth;
//         console2.log("Submitted new liquidity", adjustedLiquidity);

//         uint256 adjustedReserveY = LogNormal(source).findY(
//             adjustedReserveX, adjustedLiquidity, invariant, params
//         );
//         console2.log("Submitted new Y reserve", adjustedReserveY);

//         // Increase the adjusted y reserve by a "rounding espilon". This reduces the amount out,
//         // and ensures we pass the invariant check.
//         adjustedReserveY += 1;

//         uint256 amountYOut = reserveYWad - adjustedReserveY;

//         console2.log("Submitted trade x in", amountXIn);
//         console2.log("Submitted trade y out", amountYOut);
//         console2.log("Implied price", amountYOut * 1 ether / amountXIn);

//         // Try doing simulate swap to see if we get a similar result.
//         (bool valid, uint256 estimatedOut,, bytes memory payload) =
//             dfmm.simulateSwap(true, amountXIn);
//         console2.log("valid", valid);
//         console2.log("estimatedOut", estimatedOut);
//         console2.logBytes(payload);

//         // Try doing the swap with the adjusted reserves.
//         bytes memory swapData =
//             abi.encode(adjustedReserveX, adjustedReserveY, adjustedLiquidity);
//         console2.logBytes(swapData);

//         dfmm.swap(swapData);
//     }

//     function test_swap_validation_failure() public basic {
//         uint256 amountInY = 39997973265321579;
//         // Get all the current data: reserves, liquidity, invariant, params.
//         int256 invariant = dfmm.getSwapConstant();
//         console2.log("invariant", invariant);

//         stdstore.target(address(dfmm)).sig(dfmm.reserveXWad.selector)
//             .checked_write(989960448015974454);
//         stdstore.target(address(dfmm)).sig(dfmm.reserveYWad.selector)
//             .checked_write(1010941622138261028);
//         stdstore.target(address(dfmm)).sig(dfmm.totalLiquidity.selector)
//             .checked_write(3242480666972586605);

//         (uint256 reserveXWad, uint256 reserveYWad, uint256 liquidity) =
//             dfmm.getReservesAndLiquidity();

//         console2.log("reserveXWad", reserveXWad);
//         console2.log("reserveYWad", reserveYWad);
//         console2.log("liquidity", liquidity);

//         // Compute the adjusted x reserve and adjusted liquidity.

//         (bool valid, uint256 estimatedOut,, bytes memory payload) =
//             dfmm.simulateSwap(false, amountInY);
//         console2.log("valid", valid);
//         console2.log("estimatedOut", estimatedOut);
//         console2.logBytes(payload);

//         // Try doing the swap with the adjusted reserves.

//         dfmm.swap(payload);
//     }

//     function test_failing_swap() public basic {
//         uint256 amountInX = 51341421141636942;
//         // Get all the current data: reserves, liquidity, invariant, params.
//         int256 invariant = dfmm.getSwapConstant();
//         console2.log("invariant", invariant);

//         stdstore.target(address(dfmm)).sig(dfmm.reserveXWad.selector)
//             .checked_write(998598855675280966);
//         stdstore.target(address(dfmm)).sig(dfmm.reserveYWad.selector)
//             .checked_write(1002982509937604659);
//         stdstore.target(address(dfmm)).sig(dfmm.totalLiquidity.selector)
//             .checked_write(3243656204559085150);

//         (uint256 reserveXWad, uint256 reserveYWad, uint256 liquidity) =
//             dfmm.getReservesAndLiquidity();

//         console2.log("reserveXWad", reserveXWad);
//         console2.log("reserveYWad", reserveYWad);
//         console2.log("liquidity", liquidity);

//         // Compute the adjusted x reserve and adjusted liquidity.

//         (bool valid, uint256 estimatedOut,, bytes memory payload) =
//             dfmm.simulateSwap(true, amountInX);
//         console2.log("valid", valid);
//         console2.log("estimatedOut", estimatedOut);
//         console2.logBytes(payload);

//         // Try doing the swap with the adjusted reserves.

//         dfmm.swap(payload);
//     }

//     function test_dfmm_simulate_swap_x_in() public basic {
//         uint256 price_initial = dfmm.internalPrice();

//         uint256 amountIn = 0.1 ether;
//         bool swapXIn = true;

//         (bool valid, uint256 estimatedOut,, bytes memory payload) =
//             dfmm.simulateSwap(swapXIn, amountIn);

//         console2.log("valid", valid);
//         console2.log("estimatedOut", estimatedOut);
//         console2.logBytes(payload);

//         // Get out current Y balance.
//         uint256 yBalance = MockERC20(tokenY).balanceOf(address(this));
//         dfmm.swap(payload);
//         uint256 newYBalance = MockERC20(tokenY).balanceOf(address(this));
//         uint256 price_after = dfmm.internalPrice();

//         console2.log("yBalance", yBalance);
//         console2.log("newYBalance", newYBalance);
//         console2.log("newYBalance - yBalance", newYBalance - yBalance);
//         console2.log("price_initial", price_initial);
//         console2.log("price_after", price_after);
//         assertTrue(
//             price_after < price_initial,
//             "price did not decrease after selling x tokens"
//         );
//         assertEq(newYBalance - yBalance, estimatedOut);
//     }

//     function test_dfmm_simulate_swap_y_in() public basic {
//         uint256 amountIn = 0.1 ether;
//         bool swapXIn = false;

//         (bool valid, uint256 estimatedOut,, bytes memory payload) =
//             dfmm.simulateSwap(swapXIn, amountIn);

//         console2.log("valid", valid);
//         console2.log("estimatedOut", estimatedOut);
//         console2.logBytes(payload);

//         // Get out current X balance.
//         uint256 xBalance = MockERC20(tokenX).balanceOf(address(this));
//         dfmm.swap(payload);
//         uint256 newXBalance = MockERC20(tokenX).balanceOf(address(this));

//         console2.log("xBalance", xBalance);
//         console2.log("newXBalance", newXBalance);
//         console2.log("newXBalance - xBalance", newXBalance - xBalance);

//         // assertEq(newXBalance - xBalance, estimatedOut);
//     }

//     function test_find_profit() public basic {
//         uint256 initialGuess = 9156098526889172;
//         // uint256 initialGuess = 156098526889172;
//         bool xForY = true;

//         AtomicV2 atomic =
//             new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

//         lex.setPrice(992047873705309300);
//         (bool valid, uint256 estimatedOut, uint256 estimatedPrice,) =
//             dfmm.simulateSwap(xForY, initialGuess);

//         // (uint256 bestAmountIn, int256 bestProfit, uint256 bestGuessEndPrice) =
//         //     atomic.searchMaxArbProfit(initialGuess, xForY);

//         (int256 initialGuessProfit, uint256 initialGuessEndPrice) =
//             atomic.calculateProfit(xForY, initialGuess);
//         uint256 lexPrice = lex.price();
//         uint256 averagePrice = estimatedOut * 1 ether / initialGuess;
//         console2.log("averagePrice", averagePrice);

//         // uint256 initialGuessDifference = initialGuessEndPrice > lexPrice
//         //     ? initialGuessEndPrice - lexPrice
//         //     : lexPrice - initialGuessEndPrice;
//         // uint256 bestGuessDifference = bestGuessEndPrice > lexPrice
//         //     ? bestGuessEndPrice - lexPrice
//         //     : lexPrice - bestGuessEndPrice;

//         // console2.log("lex price", lex.price());
//         // console2.log("initialGuessProfit", initialGuessProfit);
//         // console2.log("initialGuessEndPrice", initialGuessEndPrice);
//         // console2.log("initialGuessDifference", initialGuessDifference);
//         // console2.log("bestAmountIn", bestAmountIn);
//         // console2.log("bestProfit", bestProfit);
//         // console2.log("bestGuessEndPrice", bestGuessEndPrice);
//         // console2.log("bestGuessDifference", bestGuessDifference);
//     }

//     // function test_dfmm_simulate_swap_y_in_small() public basic {
//     //     uint256 amountIn = 0.000001 ether;
//     //     bool swapXIn = false;

//     //     (bool valid, uint256 estimatedOut,, bytes memory payload) =
//     //         dfmm.simulateSwap(swapXIn, amountIn);

//     //     console2.log("valid", valid);
//     //     console2.log("estimatedOut", estimatedOut);
//     //     console2.logBytes(payload);

//     //     // Get out current X balance.
//     //     uint256 xBalance = MockERC20(tokenX).balanceOf(address(this));
//     //     dfmm.swap(payload);
//     //     uint256 newXBalance = MockERC20(tokenX).balanceOf(address(this));

//     //     console2.log("xBalance", xBalance);
//     //     console2.log("newXBalance", newXBalance);
//     //     console2.log("newXBalance - xBalance", newXBalance - xBalance);

//     //     assertEq(newXBalance - xBalance, estimatedOut);
//     // }

//     // function test_dfmm_find_trade_raise_price() public basic {
//     //     uint256 price_initial = dfmm.internalPrice();

//     //     // Need to raise price, so target price + 10%.
//     //     uint256 target_price = price_initial * 11 / 10;

//     //     (uint256 x, uint256 y, uint256 l) = dfmm.getReservesAndLiquidity();
//     //     console2.log("x", x);
//     //     console2.log("y", y);
//     //     console2.log("l", l);

//     //     AtomicV2 atomic =
//     //         new AtomicV2(address(dfmm), address(0), tokenX, tokenY);

//     //     uint256 amountIn =
//     //         atomic.try_arbitrage_until_target_price(target_price, 0);
//     // }

//     // function test_dfmm_find_trade_lower_price() public basic {
//     //     uint256 price_initial = dfmm.internalPrice();

//     //     // Need to raise price, so target price + 10%.
//     //     uint256 target_price = price_initial * 9 / 10; //333424706894090465;

//     //     (uint256 x, uint256 y, uint256 l) = dfmm.getReservesAndLiquidity();
//     //     console2.log("x", x);
//     //     console2.log("y", y);
//     //     console2.log("l", l);

//     //     AtomicV2 atomic =
//     //         new AtomicV2(address(dfmm), address(0), tokenX, tokenY);

//     //     atomic.try_arbitrage_until_target_price(target_price - 10, 0);

//     //     /* uint256 i;
//     //     while (i != 10) {
//     //         try atomic.try_arbitrage_until_target_price(target_price, 0) {
//     //             target_price = 333424706894090465 - i * 0.001 ether;
//     //         } catch { }

//     //         i++;
//     //     } */
//     // }

//     function test_price_monotonicity() public basic {
//         (uint256 x, uint256 y, uint256 l) = dfmm.getReservesAndLiquidity();

//         uint256 max_d = 1 ether - x * 1 ether / l;

//         uint256 i;
//         uint256 d = 1;
//         while (i != 25) {
//             uint256 price = source.internalPrice(x + d, l);
//             console2.log("x / l", x * 1e18 / l);
//             console2.log("x + d", x + d);
//             console2.log("price", price);
//             console2.log("cur d", d);

//             d += d * 5;
//             i++;
//         }

//         console2.log("max d", max_d);
//     }

//     function test_dfmm_atomic_arb_bug() public {
//         Parameters memory params = Parameters({
//             strikePriceWad: 1e18,
//             sigmaPercentWad: 1e18,
//             tauYearsWad: 1e18
//         });

//         bytes memory init_data = LogNormal(source).encodeInitData(
//             1000000000000000000, 999999999999999997, 3241096933647192684, params
//         );

//         dfmm.init(init_data);

//         uint256 target_price = 1 ether / 2; //333424706894090465;

//         AtomicV2 atomic =
//             new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

//         MockERC20(tokenX).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).mint(address(this), 1000 ether);
//         MockERC20(tokenX).mint(address(lex), 1000 ether);
//         MockERC20(tokenY).mint(address(lex), 1000 ether);

//         // Recreated from the stack trace from the sim!
//         lex.setPrice(0.9018324562457659 ether);
//         atomic.lower_exchange_price(0.121192175267026861 ether);
//         lex.setPrice(0.7139372064852015 ether);
//         atomic.lower_exchange_price(0.277888107205955025 ether);
//         lex.setPrice(0.7142109156818223 ether);
//     }

//     function test_dfmm_swap_over_time_dyanmic_tau() public basic {
//         console2.log("Current time: ", block.timestamp);
//         // Warp some time.
//         vm.warp(10 days);
//         console2.log("Updated time: ", block.timestamp);

//         // Try doing a swap.
//         uint256 amountIn = 0.1 ether;
//         bool swapXIn = true;

//         (bool valid, uint256 estimatedOut,, bytes memory payload) =
//             dfmm.simulateSwap(swapXIn, amountIn);

//         dfmm.swap(payload);
//     }

//     function test_dfmm_swap_dynamic_strike() public basic {
//         uint256 original_price = dfmm.internalPrice();

//         // Change the strike.
//         stdstore.target(address(source)).sig(source.targetStrike.selector)
//             .checked_write(1.5 ether);

//         // Log the strike price.
//         console2.log("Current strike: ", source.strikePrice());
//         // Log the target strike.
//         console2.log("Target strike: ", source.targetStrike());

//         // Try doing a swap.
//         uint256 amountIn = 10;
//         bool swapXIn = true;

//         (bool valid, uint256 estimatedOut,, bytes memory payload) =
//             dfmm.simulateSwap(swapXIn, amountIn);

//         (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
//             dfmm.getReservesAndLiquidity();
//         console2.log("Balance[X]", MockERC20(tokenX).balanceOf(address(dfmm)));
//         console2.log("Balance[Y]", MockERC20(tokenY).balanceOf(address(dfmm)));
//         console2.log("Reserve[X]", reserveX);
//         console2.log("Reserve[Y]", reserveY);
//         console2.log("Liquidity", liquidity);

//         console2.log("Amount[IN]", amountIn);
//         console2.log("Amount[OUT]", estimatedOut);
//         console2.log("Price[APRX]", estimatedOut * 1 ether / amountIn);
//         uint256 oldPrice = dfmm.internalPrice();
//         dfmm.swap(payload);
//         console2.log("Price[ORGN]", original_price);
//         console2.log("Price[OLD]", oldPrice);
//         console2.log("Price[NEW]", dfmm.internalPrice());

//         (reserveX, reserveY, liquidity) = dfmm.getReservesAndLiquidity();
//         console2.log("Balance[X]", MockERC20(tokenX).balanceOf(address(dfmm)));
//         console2.log("Balance[Y]", MockERC20(tokenY).balanceOf(address(dfmm)));
//         console2.log("Reserve[X]", reserveX);
//         console2.log("Reserve[Y]", reserveY);
//         console2.log("Liquidity", liquidity);
//     }

//     function test_profit_finder_raise_price() public basic {
//         AtomicV2 atomic =
//             new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

//         MockERC20(tokenX).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).mint(address(this), 1000 ether);
//         MockERC20(tokenX).mint(address(lex), 1000 ether);
//         MockERC20(tokenY).mint(address(lex), 1000 ether);

//         lex.setPrice(1.5 ether);
//         uint256 input;
//     }

//     function test_profit_finder_lower_price() public basic {
//         AtomicV2 atomic =
//             new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

//         MockERC20(tokenX).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).mint(address(this), 1000 ether);
//         MockERC20(tokenX).mint(address(lex), 1000 ether);
//         MockERC20(tokenY).mint(address(lex), 1000 ether);

//         lex.setPrice(0.5 ether);
//         uint256 input;
//     }

//     function test_profit_finder_from_atomic_lower() public basic { }

//     function test_profit_finder_from_atomic_raise() public basic {
//         AtomicV2 atomic =
//             new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

//         MockERC20(tokenX).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).approve(address(atomic), type(uint256).max);
//         MockERC20(tokenY).mint(address(this), 1000 ether);
//         MockERC20(tokenX).mint(address(lex), 1000 ether);
//         MockERC20(tokenY).mint(address(lex), 1000 ether);

//         lex.setPrice(1.5 ether);
//     }
// }
