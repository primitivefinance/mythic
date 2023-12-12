// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../v3/BisectionLib.sol";
import "../DFMM.sol";
import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../AtomicV2.sol";

contract LEX {
    using FixedPointMathLib for int256;
    using FixedPointMathLib for uint256;

    address public admin;
    address public arbiterTokenX;
    address public arbiterTokenY;
    uint256 public price;
    uint256 constant WAD = 10 ** 18;

    // Each LiquidExchange contract will be deployed with a pair of token addresses and an initial price
    constructor(
        address arbiterTokenX_,
        address arbiterTokenY_,
        uint256 price_
    ) {
        admin = msg.sender; // Set the contract deployer as the initial admin
        arbiterTokenX = arbiterTokenX_;
        arbiterTokenY = arbiterTokenY_;
        price = price_;
    }

    // Our admin lock
    modifier onlyAdmin() {
        require(msg.sender == admin, "Only admin can call this function");
        _;
    }

    event PriceChange(uint256 price);
    event Swap(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        address to
    );

    // Admin only function to set the price of x in terms of y
    function setPrice(uint256 _price) public onlyAdmin {
        price = _price;
        emit PriceChange(price);
    }

    function swap(address tokenIn, uint256 amountIn) public {
        uint256 amountOut;
        address tokenOut;
        if (tokenIn == arbiterTokenX) {
            tokenOut = arbiterTokenY;
            amountOut = FixedPointMathLib.mulWadDown(amountIn, price);
        } else if (tokenIn == arbiterTokenY) {
            tokenOut = arbiterTokenX;
            amountOut = FixedPointMathLib.divWadDown(amountIn, price);
        } else {
            revert("Invalid token");
        }
        require(
            ERC20(tokenIn).transferFrom(msg.sender, address(this), amountIn),
            "Transfer failed"
        );
        require(
            ERC20(tokenOut).transfer(msg.sender, amountOut), "Transfer failed"
        );
        emit Swap(tokenIn, tokenOut, amountIn, amountOut, msg.sender);
    }
}

contract DFMMTest is Test {
    using stdStorage for StdStorage;

    LogNormal source;
    DFMM dfmm;
    address tokenX;
    address tokenY;
    LEX lex;

    uint256 public constant TEST_SWAP_FEE = 0.003e18;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 1_000_000000 ether);
        MockERC20(tokenY).mint(address(this), 1_000_000000 ether);

        lex = new LEX(tokenX, tokenY, 1_000_000000 ether);

        dfmm = new DFMM(tokenX, tokenY, TEST_SWAP_FEE);
        source = LogNormal(dfmm.source());
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);
        Parameters memory params = Parameters({
            strikePriceWad: 1e18,
            sigmaPercentWad: 1e18,
            tauYearsWad: 1e18
        });
        uint256 init_p = 1.0e18;
        uint256 init_x = 1.0e18;
        uint256 init_l = LogNormal(source).lx(init_x, init_p, params);
        uint256 init_y = LogNormal(source).yl(init_l, init_p, params);
        int256 swapConstantInit = 0;
        uint256 found_l = LogNormal(source).findLiquidity(
            init_x, init_y, swapConstantInit, params
        );

        bytes memory init_data =
            LogNormal(source).encodeInitData(init_x, init_y, found_l, params);

        dfmm.init(init_data);

        console2.log("Initialized L:       ", found_l);
        console2.log("Initialized X:       ", init_x);
        console2.log("Initialized Y:       ", init_y);
        console2.log("Initialized strike:  ", source.strikePrice());
        console2.log("Initialized sigma:   ", source.sigma());
        console2.log("Initialized tau:     ", source.tau());
        _;
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier pool_with_high_price() {
        vm.warp(0);
        Parameters memory params = Parameters({
            strikePriceWad: 2500 ether,
            sigmaPercentWad: 1e18,
            tauYearsWad: 1e18
        });
        uint256 init_p = 2500 ether;
        uint256 init_x = 0.5 ether;
        uint256 init_l = LogNormal(source).lx(init_x, init_p, params);
        uint256 init_y = LogNormal(source).yl(init_l, init_p, params);
        int256 swapConstantInit = 0;
        uint256 found_l = LogNormal(source).findLiquidity(
            init_x, init_y, swapConstantInit, params
        ) - 100;

        bytes memory init_data =
            LogNormal(source).encodeInitData(init_x, init_y, found_l, params);

        dfmm.init(init_data);

        console2.log("Initialized L:       ", found_l);
        console2.log("Initialized X:       ", init_x);
        console2.log("Initialized Y:       ", init_y);
        console2.log("Initialized strike:  ", source.strikePrice());
        console2.log("Initialized sigma:   ", source.sigma());
        console2.log("Initialized tau:     ", source.tau());
        _;
    }

    function test_dfmm_init() public {
        Parameters memory params = Parameters({
            strikePriceWad: 1e18,
            sigmaPercentWad: 1e18,
            tauYearsWad: 1e18
        });
        uint256 init_p = 1.0e18;
        uint256 init_x = 0.5e18;

        // note: adding 2 wei to the d1 value before calling cdf with these values will result in 0.5e18 y reserves...
        // actually adding 1 wei to d2 in the y given l computation also yields 0.5e18 y reserves.
        int256 d1 = LogNormal(source).computeD1(init_p, params);
        console2.logInt(d1);
        int256 cdfOfD1 = Gaussian.cdf(d1);
        console2.logInt(cdfOfD1);

        uint256 init_l = LogNormal(source).lx(init_x, init_p, params);
        uint256 init_y = LogNormal(source).yl(init_l, init_p, params);
        console2.log("init_l", init_l);
        console2.log("init_y", init_y);
        int256 swapConstantInit = 0;
        uint256 found_l = LogNormal(source).findLiquidity(
            init_x, init_y, swapConstantInit, params
        );
        console2.log("found_l", found_l);

        // This computation is slightly off (invariant won't be zero).
        bytes memory init_data =
            LogNormal(source).encodeInitData(init_x, init_y, found_l, params);

        console2.log("init_x", init_x);
        console2.log("init_y", init_y);
        console2.log("init_l", init_l);

        console2.log(MockERC20(tokenX).balanceOf(address(this)));
        console2.log(MockERC20(tokenY).balanceOf(address(this)));

        (uint256 x, uint256 y, uint256 l) = dfmm.init(init_data);
        console2.log("x", x);
        console2.log("y", y);
        console2.log("l", l);

        assertEq(x, init_x, "x != init x");
        assertEq(y, init_y, "y != init y");
        assertEq(l, found_l, "l != found l");
        assertEq(dfmm.balanceOf(address(this)), found_l, "l != found l");
        assertTrue(dfmm.inited(), "not initialized!");
        assertEq(
            MockERC20(tokenX).balanceOf(address(dfmm)),
            init_x,
            "x balance != init x"
        );
        assertEq(
            MockERC20(tokenY).balanceOf(address(dfmm)),
            init_y,
            "y balance != init y"
        );
    }

    function test_dfmm_swap_x_in() public basic {
        uint256 feePercentageWad = source.swapFeePercentageWad();

        // Get all the current data: reserves, liquidity, invariant, params.
        int256 invariant = dfmm.getSwapConstant();
        console2.logInt(invariant);

        (uint256 reserveXWad, uint256 reserveYWad, uint256 liquidity) =
            dfmm.getReservesAndLiquidity();

        console2.log("reserveXWad", reserveXWad);
        console2.log("reserveYWad", reserveYWad);
        console2.log("liquidity", liquidity);

        Parameters memory params = source.staticSlot();

        // todo: should take this out.
        uint256 price = 1.0e18;

        // Compute the adjusted x reserve and adjusted liquidity.
        uint256 amountXIn = 0.1 ether;

        uint256 adjustedReserveX = reserveXWad + amountXIn;
        console2.log("Submitted new X reserve", adjustedReserveX);

        uint256 expectedLiquidityGrowth =
            source.lx(amountXIn * feePercentageWad / 1 ether, price, params);

        // Try commenting this line of code out, the test should fail!
        expectedLiquidityGrowth += 1;

        console2.log("expectedLiquidityGrowth", expectedLiquidityGrowth);

        // So we compute the adjusted liquidity amount to find the y, but pass in the origina liquidity.
        // So we need the contract to validate the expected growth.
        uint256 adjustedLiquidity = liquidity + expectedLiquidityGrowth;
        console2.log("Submitted new liquidity", adjustedLiquidity);

        uint256 adjustedReserveY = LogNormal(source).findY(
            adjustedReserveX, adjustedLiquidity, invariant, params
        );
        console2.log("Submitted new Y reserve", adjustedReserveY);

        // Increase the adjusted y reserve by a "rounding espilon". This reduces the amount out,
        // and ensures we pass the invariant check.
        adjustedReserveY += 1;

        uint256 amountYOut = reserveYWad - adjustedReserveY;

        console2.log("Submitted trade x in", amountXIn);
        console2.log("Submitted trade y out", amountYOut);
        console2.log("Implied price", amountYOut * 1 ether / amountXIn);

        // Try doing simulate swap to see if we get a similar result.
        (bool valid, uint256 estimatedOut,, bytes memory payload) =
            dfmm.simulateSwap(true, amountXIn);
        console2.log("valid", valid);
        console2.log("estimatedOut", estimatedOut);
        console2.logBytes(payload);

        // Try doing the swap with the adjusted reserves.
        bytes memory swapData =
            abi.encode(adjustedReserveX, adjustedReserveY, adjustedLiquidity);
        console2.logBytes(swapData);

        dfmm.swap(swapData);
    }

    function test_dfmm_simulate_swap_x_in() public basic {
        uint256 price_initial = dfmm.internalPrice();

        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        (bool valid, uint256 estimatedOut,, bytes memory payload) =
            dfmm.simulateSwap(swapXIn, amountIn);

        console2.log("valid", valid);
        console2.log("estimatedOut", estimatedOut);
        console2.logBytes(payload);

        // Get out current Y balance.
        uint256 yBalance = MockERC20(tokenY).balanceOf(address(this));
        dfmm.swap(payload);
        uint256 newYBalance = MockERC20(tokenY).balanceOf(address(this));
        uint256 price_after = dfmm.internalPrice();

        console2.log("yBalance", yBalance);
        console2.log("newYBalance", newYBalance);
        console2.log("newYBalance - yBalance", newYBalance - yBalance);
        console2.log("price_initial", price_initial);
        console2.log("price_after", price_after);
        assertTrue(
            price_after < price_initial,
            "price did not decrease after selling x tokens"
        );
        assertEq(newYBalance - yBalance, estimatedOut);
    }

    function test_dfmm_simulate_swap_y_in() public basic {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = false;

        (bool valid, uint256 estimatedOut,, bytes memory payload) =
            dfmm.simulateSwap(swapXIn, amountIn);

        console2.log("valid", valid);
        console2.log("estimatedOut", estimatedOut);
        console2.logBytes(payload);

        // Get out current X balance.
        uint256 xBalance = MockERC20(tokenX).balanceOf(address(this));
        dfmm.swap(payload);
        uint256 newXBalance = MockERC20(tokenX).balanceOf(address(this));

        console2.log("xBalance", xBalance);
        console2.log("newXBalance", newXBalance);
        console2.log("newXBalance - xBalance", newXBalance - xBalance);

        assertEq(newXBalance - xBalance, estimatedOut);
    }

    function test_dfmm_simulate_swap_y_in_small() public basic {
        uint256 amountIn = 0.000001 ether;
        bool swapXIn = false;

        (bool valid, uint256 estimatedOut,, bytes memory payload) =
            dfmm.simulateSwap(swapXIn, amountIn);

        console2.log("valid", valid);
        console2.log("estimatedOut", estimatedOut);
        console2.logBytes(payload);

        // Get out current X balance.
        uint256 xBalance = MockERC20(tokenX).balanceOf(address(this));
        dfmm.swap(payload);
        uint256 newXBalance = MockERC20(tokenX).balanceOf(address(this));

        console2.log("xBalance", xBalance);
        console2.log("newXBalance", newXBalance);
        console2.log("newXBalance - xBalance", newXBalance - xBalance);

        assertEq(newXBalance - xBalance, estimatedOut);
    }

    function test_dfmm_find_trade_raise_price() public basic {
        uint256 price_initial = dfmm.internalPrice();

        // Need to raise price, so target price + 10%.
        uint256 target_price = price_initial * 11 / 10;

        (uint256 x, uint256 y, uint256 l) = dfmm.getReservesAndLiquidity();
        console2.log("x", x);
        console2.log("y", y);
        console2.log("l", l);

        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(0), tokenX, tokenY);

        uint256 amountIn =
            atomic.try_arbitrage_until_target_price(target_price, 0);
    }

    function test_dfmm_find_trade_lower_price() public basic {
        uint256 price_initial = dfmm.internalPrice();

        // Need to raise price, so target price + 10%.
        uint256 target_price = price_initial * 9 / 10; //333424706894090465;

        (uint256 x, uint256 y, uint256 l) = dfmm.getReservesAndLiquidity();
        console2.log("x", x);
        console2.log("y", y);
        console2.log("l", l);

        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(0), tokenX, tokenY);

        atomic.try_arbitrage_until_target_price(target_price - 10, 0);

        /* uint256 i;
        while (i != 10) {
            try atomic.try_arbitrage_until_target_price(target_price, 0) {
                target_price = 333424706894090465 - i * 0.001 ether;
            } catch { }

            i++;
        } */
    }

    function test_price_monotonicity() public basic {
        (uint256 x, uint256 y, uint256 l) = dfmm.getReservesAndLiquidity();

        uint256 max_d = 1 ether - x * 1 ether / l;

        uint256 i;
        uint256 d = 1;
        while (i != 25) {
            uint256 price = source.internalPrice(x + d, l);
            console2.log("x / l", x * 1e18 / l);
            console2.log("x + d", x + d);
            console2.log("price", price);
            console2.log("cur d", d);

            d += d * 5;
            i++;
        }

        console2.log("max d", max_d);
    }

    function test_dfmm_atomic_arb_bug() public {
        Parameters memory params = Parameters({
            strikePriceWad: 1e18,
            sigmaPercentWad: 1e18,
            tauYearsWad: 1e18
        });

        bytes memory init_data = LogNormal(source).encodeInitData(
            1000000000000000000, 999999999999999997, 3241096933647192684, params
        );

        dfmm.init(init_data);

        uint256 target_price = 1 ether / 2; //333424706894090465;

        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).mint(address(this), 1000 ether);
        MockERC20(tokenX).mint(address(lex), 1000 ether);
        MockERC20(tokenY).mint(address(lex), 1000 ether);

        // Recreated from the stack trace from the sim!
        lex.setPrice(0.9018324562457659 ether);
        atomic.lower_exchange_price(0.121192175267026861 ether);
        lex.setPrice(0.7139372064852015 ether);
        atomic.lower_exchange_price(0.277888107205955025 ether);
        lex.setPrice(0.7142109156818223 ether);
        atomic.try_arbitrage_until_target_price(0.7142109156818223 ether, 0);
    }

    function test_dfmm_swap_over_time_dyanmic_tau() public basic {
        console2.log("Current time: ", block.timestamp);
        // Warp some time.
        vm.warp(10 days);
        console2.log("Updated time: ", block.timestamp);

        // Try doing a swap.
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        (bool valid, uint256 estimatedOut,, bytes memory payload) =
            dfmm.simulateSwap(swapXIn, amountIn);

        dfmm.swap(payload);
    }

    function test_dfmm_swap_dynamic_strike() public basic {
        uint256 original_price = dfmm.internalPrice();

        // Change the strike.
        stdstore.target(address(source)).sig(source.targetStrike.selector)
            .checked_write(1.5 ether);

        // Log the strike price.
        console2.log("Current strike: ", source.strikePrice());
        // Log the target strike.
        console2.log("Target strike: ", source.targetStrike());

        // Try doing a swap.
        uint256 amountIn = 10;
        bool swapXIn = true;

        (bool valid, uint256 estimatedOut,, bytes memory payload) =
            dfmm.simulateSwap(swapXIn, amountIn);

        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity();
        console2.log("Balance[X]", MockERC20(tokenX).balanceOf(address(dfmm)));
        console2.log("Balance[Y]", MockERC20(tokenY).balanceOf(address(dfmm)));
        console2.log("Reserve[X]", reserveX);
        console2.log("Reserve[Y]", reserveY);
        console2.log("Liquidity", liquidity);

        console2.log("Amount[IN]", amountIn);
        console2.log("Amount[OUT]", estimatedOut);
        console2.log("Price[APRX]", estimatedOut * 1 ether / amountIn);
        uint256 oldPrice = dfmm.internalPrice();
        dfmm.swap(payload);
        console2.log("Price[ORGN]", original_price);
        console2.log("Price[OLD]", oldPrice);
        console2.log("Price[NEW]", dfmm.internalPrice());

        (reserveX, reserveY, liquidity) = dfmm.getReservesAndLiquidity();
        console2.log("Balance[X]", MockERC20(tokenX).balanceOf(address(dfmm)));
        console2.log("Balance[Y]", MockERC20(tokenY).balanceOf(address(dfmm)));
        console2.log("Reserve[X]", reserveX);
        console2.log("Reserve[Y]", reserveY);
        console2.log("Liquidity", liquidity);
    }

    function test_profit_finder_raise_price() public basic {
        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).mint(address(this), 1000 ether);
        MockERC20(tokenX).mint(address(lex), 1000 ether);
        MockERC20(tokenY).mint(address(lex), 1000 ether);

        lex.setPrice(1.5 ether);
        uint256 input;

        try atomic.profitFinder().searchRaisePrice(100, 10) { }
        catch (bytes memory err) {
            assembly {
                err := add(err, 0x04)
            }

            (input,) = abi.decode(err, (uint256, uint256));
        }
        // do the trade
        atomic.raise_exchange_price(input);
        // check the cumulative profit
        console2.log("input amount", input);
        console2.log("cumulative profit", atomic.cumulativeProfit());
    }

    function test_profit_finder_lower_price() public basic {
        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).mint(address(this), 1000 ether);
        MockERC20(tokenX).mint(address(lex), 1000 ether);
        MockERC20(tokenY).mint(address(lex), 1000 ether);

        lex.setPrice(0.5 ether);
        uint256 input;
        try atomic.profitFinder().searchLowerPrice(100, 10) { }
        catch (bytes memory err) {
            assembly {
                err := add(err, 0x04)
            }

            (input,) = abi.decode(err, (uint256, uint256));
        }
        // do the trade
        atomic.lower_exchange_price(input);
        // check the cumulative profit
        console2.log("input amount", input);
        console2.log("cumulative profit", atomic.cumulativeProfit());
    }

    function test_profit_finder_from_atomic_lower() public basic {
        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).mint(address(this), 1000 ether);
        MockERC20(tokenX).mint(address(lex), 1000 ether);
        MockERC20(tokenY).mint(address(lex), 1000 ether);

        lex.setPrice(0.8358473209862632 ether);
        (uint256 input,) = atomic.searchLowerPrice(100, 5);
        uint256 price = dfmm.internalPrice();
        atomic.lower_exchange_price(input);

        console2.log("Price[SRT]", price);
        console2.log("Price[END]", dfmm.internalPrice());
    }

    function test_profit_finder_from_atomic_raise() public basic {
        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).mint(address(this), 1000 ether);
        MockERC20(tokenX).mint(address(lex), 1000 ether);
        MockERC20(tokenY).mint(address(lex), 1000 ether);

        lex.setPrice(1.5 ether);
        atomic.searchRaisePrice(256, 10);
    }

    function test_arb_loop() public pool_with_high_price {
        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).mint(address(this), 1000000 ether);
        MockERC20(tokenX).mint(address(lex), 1000000 ether);
        MockERC20(tokenY).mint(address(lex), 1000000 ether);

        // Set LEX 2500 -> 3000
        lex.setPrice(3000 ether);

        // Check current DEX price.
        uint256 dex_price = dfmm.internalPrice();

        // Swap Y -> X -> Y'.
        uint256 y_in = 1 ether;
        uint256 y_balance = MockERC20(tokenY).balanceOf(address(this));
        atomic.raise_exchange_price(y_in);
        uint256 y_balance_after = MockERC20(tokenY).balanceOf(address(this));

        // Profit = Y' - Y
        uint256 profit = y_balance_after - y_balance;

        // Initial swap's output X tokens.
        uint256 intermediate_x = atomic.intermediateTokenXBalance();

        // Effective price = Y / X
        uint256 effective_price = y_in * 1 ether / intermediate_x;

        console2.log("dex price before", dex_price);
        console2.log("dex price after", dfmm.internalPrice());
        console2.log("intermediate x", intermediate_x);
        console2.log("effective price", effective_price);
        console2.log("y balance before", y_balance);
        console2.log("y balance after", y_balance_after);
        console2.log("profit", profit);
    }

    function test_arb_loop_simple() public basic {
        AtomicV2 atomic =
            new AtomicV2(address(dfmm), address(lex), tokenX, tokenY);

        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).mint(address(this), 1000000 ether);
        MockERC20(tokenX).mint(address(lex), 1000000 ether);
        MockERC20(tokenY).mint(address(lex), 1000000 ether);

        // Set LEX 1.0 -> 0.992047873705309300
        uint256 new_price = 0.985 ether; //0.9920478737053093 ether;
        lex.setPrice(new_price);

        // Check current DEX price.
        uint256 dex_price = dfmm.internalPrice();

        // Swap Y -> X -> Y'.
        uint256 delta_x = 0.0173635 ether; //0.0091561 ether;
        uint256 delta_y_to_get_x = delta_x * lex.price() / 1 ether;
        uint256 y_balance = MockERC20(tokenY).balanceOf(address(this));
        atomic.lower_exchange_price(delta_y_to_get_x);
        uint256 y_balance_after = MockERC20(tokenY).balanceOf(address(this));

        // Profit = Y' - Y
        uint256 profit = y_balance_after - y_balance;

        // Initial swap's output X tokens.
        uint256 intermediate_x = atomic.intermediateTokenXBalance();

        // Make sure its the same as the delta_x we sent in.
        assertEq(intermediate_x, delta_x, "invalid immediate x!");

        // Intermediate Y is the initial Y we send in.
        uint256 intermediate_y = atomic.intermediateTokenYStartBalance();

        // End Y is the final Y we get out.
        uint256 intermediate_y_end = atomic.intermediateTokenYEndBalance();

        // Effective price = Y / X
        uint256 effective_price = intermediate_y_end * 1 ether / intermediate_x;

        console2.log("dex price before", dex_price);
        console2.log("dex price after", dfmm.internalPrice());
        console2.log("intermediate y", intermediate_y);
        console2.log("intermediate x", intermediate_x);
        console2.log("effective price", effective_price);
        console2.log("y balance before", y_balance);
        console2.log("y balance after", y_balance_after);
        console2.log("profit", profit);
    }
}
