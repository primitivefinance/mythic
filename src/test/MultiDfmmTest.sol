// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../MultiDFMM.sol";
import "../strategies/LogNormal.sol";
import "../solvers/LogNormalSolver.sol";
import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../interfaces/IParams.sol";
import "../Lex.sol";

contract MultiDFMMTest is Test, IParams {
    using stdStorage for StdStorage;

    MultiDFMM dfmm;
    LogNormalSolver solver;
    LogNormal logNormal;
    address tokenX;
    address tokenY;
    Lex lex;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    uint256 public constant POOL_ID = 0;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100e18);
        MockERC20(tokenY).mint(address(this), 100000e18);

        lex = new Lex(tokenX, tokenY, ONE);

        dfmm = new MultiDFMM();
        logNormal = new LogNormal(address(dfmm), TEST_SWAP_FEE);
        solver = new LogNormalSolver(address(logNormal));
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    modifier realisticEth() {
        vm.warp(0);
        LogNormParameters memory params =
            LogNormParameters({ strike: ONE * 2300, sigma: ONE, tau: ONE });
        uint256 init_p = ONE * 2345;
        uint256 init_x = ONE * 10;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        InitParams memory initParams;
        initParams.poolId = dfmm.nonce();
        initParams.strategy = address(logNormal);
        initParams.tokenX = tokenX;
        initParams.tokenY = tokenY;
        initParams.swapFeePercentageWad = TEST_SWAP_FEE;
        initParams.data = initData;
        dfmm.init(initParams);

        _;
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);
        LogNormParameters memory params =
            LogNormParameters({ strike: ONE, sigma: ONE, tau: ONE });
        uint256 init_p = ONE;
        uint256 init_x = ONE;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        InitParams memory initParams;
        initParams.poolId = dfmm.nonce();
        initParams.strategy = address(logNormal);
        initParams.tokenX = tokenX;
        initParams.tokenY = tokenY;
        initParams.swapFeePercentageWad = TEST_SWAP_FEE;
        initParams.data = initData;

        dfmm.init(initParams);

        _;
    }

    function test_multi_basic() public basic { }

    function test_multi_dfmm_swap_x_in() public basic {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(POOL_ID, payload);
    }

    function test_multi_dfmm_swap_y_in() public basic {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = false;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(POOL_ID, payload);
    }

    function test_multi_internal_price() public basic {
        uint256 internalPrice = solver.internalPrice(POOL_ID);

        console2.log(internalPrice);
    }

    function test_multi_internal_price_post_y_in() public basic {
        uint256 internalPrice = solver.internalPrice(POOL_ID);
        uint256 amountIn = 0.1 ether;
        bool swapXIn = false;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(POOL_ID, payload);

        uint256 postSwapInternalPrice = solver.internalPrice(POOL_ID);

        assertGt(postSwapInternalPrice, internalPrice);
    }

    function test_multi_internal_price_post_x_in() public basic {
        uint256 internalPrice = solver.internalPrice(POOL_ID);
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(POOL_ID, payload);

        uint256 postSwapInternalPrice = solver.internalPrice(POOL_ID);

        assertLt(postSwapInternalPrice, internalPrice);
    }

    function test_multi_swap_eth_backtest() public realisticEth {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(POOL_ID, payload);
    }

    function test_multi_allocate_liquidity_given_x() public basic {
        uint256 amountX = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) =
            solver.allocateGivenX(POOL_ID, amountX);

        uint256 preBalance = dfmm.balanceOf(address(this), POOL_ID);
        Pool memory pool = dfmm.getPool(POOL_ID);
        uint256 preTotalLiquidity = pool.totalLiquidity;

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.allocate(POOL_ID, data);

        Pool memory postPool = dfmm.getPool(POOL_ID);

        uint256 deltaTotalLiquidity =
            postPool.totalLiquidity - preTotalLiquidity;
        assertEq(
            preBalance + deltaTotalLiquidity,
            dfmm.balanceOf(address(this), POOL_ID)
        );
    }

    function test_allocate_multiple_times() public basic {
        uint256 amountX = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) =
            solver.allocateGivenX(POOL_ID, amountX);

        uint256 preBalance = dfmm.balanceOf(address(this), POOL_ID);
        Pool memory pool = dfmm.getPool(POOL_ID);
        uint256 deltaLiquidity = L - pool.totalLiquidity;
        bytes memory data = abi.encode(rx, ry, L);
        dfmm.allocate(POOL_ID, data);
        assertEq(
            preBalance + deltaLiquidity, dfmm.balanceOf(address(this), POOL_ID)
        );

        (rx, ry, L) = solver.allocateGivenX(POOL_ID, amountX * 2);
        Pool memory postPool = dfmm.getPool(POOL_ID);
        deltaLiquidity = L - postPool.totalLiquidity;
        data = abi.encode(rx, ry, L);

        MockERC20(tokenX).mint(address(0xbeef), rx);
        MockERC20(tokenY).mint(address(0xbeef), ry);

        vm.startPrank(address(0xbeef));
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
        dfmm.allocate(POOL_ID, data);
        assertEq(deltaLiquidity, dfmm.balanceOf(address(0xbeef), POOL_ID));
        vm.stopPrank();
    }

    function test_deallocate_liquidity_given_x() public basic {
        uint256 amountX = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) =
            solver.deallocateGivenX(POOL_ID, amountX);

        uint256 preBalance = dfmm.balanceOf(address(this), POOL_ID);
        Pool memory pool = dfmm.getPool(POOL_ID);
        uint256 preTotalLiquidity = pool.totalLiquidity;

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.deallocate(POOL_ID, data);

        Pool memory postPool = dfmm.getPool(POOL_ID);

        uint256 deltaTotalLiquidity =
            preTotalLiquidity - postPool.totalLiquidity;
        assertEq(
            preBalance - deltaTotalLiquidity,
            dfmm.balanceOf(address(this), POOL_ID)
        );
    }

    function test_allocate_liquidity_given_y() public basic {
        uint256 amountY = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) =
            solver.allocateGivenY(POOL_ID, amountY);

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.allocate(POOL_ID, data);
    }

    function test_deallocate_liquidity_given_y() public basic {
        uint256 amountY = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) =
            solver.deallocateGivenY(POOL_ID, amountY);

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.deallocate(POOL_ID, data);
    }
}
