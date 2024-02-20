// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../../DFMM.sol";
import "../../strategies/LogNormal/LogNormal.sol";
import "../../solvers/LogNormal/LogNormalSolver.sol";
import "../helpers/Lex.sol";

contract LogNormalTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    LogNormal logNormal;
    LogNormalSolver solver;
    address tokenX;
    address tokenY;
    Lex lex;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100_000_000 ether);
        MockERC20(tokenY).mint(address(this), 100_000_000 ether);

        lex = new Lex(tokenX, tokenY, ONE);
        dfmm = new DFMM(address(0));
        logNormal = new LogNormal(address(dfmm));
        solver = new LogNormalSolver(address(logNormal));
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    modifier realisticEth() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: ONE * 2300,
            sigma: ONE,
            tau: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = ONE * 2345;
        uint256 init_x = ONE * 10;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: TWO,
            sigma: 0.25 ether,
            tau: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = TWO;
        uint256 init_x = ONE;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }

    modifier revert_scenario() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: 0.67323818941934077 ether,
            sigma: ONE,
            tau: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = 1_329_956_352_651_532_999;
        uint256 init_x = 70.658087306013359413 ether;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }

    function test_ln_swap_x_in() public basic {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);

        dfmm.swap(poolId, swapData);
    }

    function test_ln_swap_y_in() public basic {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);

        dfmm.swap(poolId, swapData);
    }

    // todo: write assertApproxEq
    function test_price_formulas() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        (uint256 rx, uint256 ry, uint256 L) =
            solver.getReservesAndLiquidity(poolId);
        uint256 priceGivenY = solver.getPriceGivenYL(poolId, ry, L);
        uint256 priceGivenX = solver.getPriceGivenXL(poolId, rx, L);
        assertApproxEqAbs(priceGivenY, priceGivenX, 100);
    }

    function test_ln_diff_lower() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diffLowered = solver.calculateDiffLower(
            poolId, 1.9 ether, 160_249_195_342_896_967
        );

        console2.log(diffLowered);
    }

    function test_ln_diff_raise() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diffLowered = solver.calculateDiffRaise(
            poolId, 2.1 ether, 302_756_023_375_108_995
        );

        console2.log(diffLowered);
    }

    function test_ln_optimal_lower() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 optimalLower = solver.computeOptimalArbLowerPrice(
            poolId, 1.9 ether, 0.181424 ether
        );

        console2.log(optimalLower);
    }

    function test_ln_optimal_raise() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 optimalRaise = solver.computeOptimalArbRaisePrice(
            poolId, 2.1 ether, 0.345156 ether
        );

        console2.log(optimalRaise);
    }

    // function test_internal_price() public basic {
    //     uint256 internalPrice = solver.internalPrice();

    //     console2.log(internalPrice);
    // }

    // function test_internal_price_post_y_in() public basic {
    //     uint256 internalPrice = solver.internalPrice();
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = false;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);

    //     uint256 postSwapInternalPrice = solver.internalPrice();

    //     assertGt(postSwapInternalPrice, internalPrice);
    // }

    // function test_internal_price_post_x_in() public basic {
    //     uint256 internalPrice = solver.internalPrice();
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = true;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);

    //     uint256 postSwapInternalPrice = solver.internalPrice();

    //     assertLt(postSwapInternalPrice, internalPrice);
    // }

    // function test_swap_eth_backtest() public realisticEth {
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = true;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);
    // }

    // function test_allocate_multiple_times() public basic {
    //     uint256 amountX = 0.1 ether;
    //     (uint256 rx, uint256 ry, uint256 L) = solver.allocateGivenX(amountX);

    //     uint256 preBalance = dfmm.balanceOf(address(this));
    //     uint256 deltaLiquidity = L - dfmm.totalLiquidity();
    //     bytes memory data = abi.encode(rx, ry, L);
    //     dfmm.allocate(data);
    //     assertEq(preBalance + deltaLiquidity, dfmm.balanceOf(address(this)));

    //     (rx, ry, L) = solver.allocateGivenX(amountX * 2);
    //     deltaLiquidity = L - dfmm.totalLiquidity();
    //     data = abi.encode(rx, ry, L);

    //     MockERC20(tokenX).mint(address(0xbeef), rx);
    //     MockERC20(tokenY).mint(address(0xbeef), ry);

    //     vm.startPrank(address(0xbeef));
    //     MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
    //     MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    //     dfmm.allocate(data);
    //     assertEq(deltaLiquidity, dfmm.balanceOf(address(0xbeef)));
    //     vm.stopPrank();
    // }
}
