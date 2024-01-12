// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../../DFMM.sol";
import "../../strategies/LogNormal/LogNormal.sol";
import "../../strategies/G3M/G3M.sol";
import "../helpers/Lex.sol";
import "../LogNormal/LogNormalSolver.sol";
import "../G3M/G3MSolver.sol";

contract MultiDFMMTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    LogNormal logNormal;
    LogNormalSolver logNormSolver;
    G3M g3m;
    G3MSolver g3mSolver;
    address tokenX;
    address tokenY;
    Lex lex;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    uint256 public LN_POOL_ID;
    uint256 public G3M_POOL_ID;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100e18);
        MockERC20(tokenY).mint(address(this), 100000e18);

        lex = new Lex(tokenX, tokenY, ONE);

        dfmm = new DFMM();

        logNormal = new LogNormal(address(dfmm));
        logNormSolver = new LogNormalSolver(address(logNormal));
        g3m = new G3M(address(dfmm));
        g3mSolver = new G3MSolver(address(g3m));

        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    modifier realisticEth() {
        vm.warp(0);
        LogNormParameters memory params = LogNormParameters({
            strike: ONE * 2300,
            sigma: ONE,
            tau: ONE,
            swapFee: TEST_SWAP_FEE
        });
        uint256 init_p = ONE * 2345;
        uint256 init_x = ONE * 10;
        bytes memory initData =
            logNormSolver.getInitialPoolData(init_x, init_p, params);

        IMultiCore.InitParams memory initParams;
        initParams.strategy = address(logNormal);
        initParams.tokenX = tokenX;
        initParams.tokenY = tokenY;
        initParams.data = initData;
        (LN_POOL_ID,,,) = dfmm.init(initParams);

        _;
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);
        G3MParameters memory g3mParams = G3MParameters({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: TEST_SWAP_FEE
        });
        uint256 init_p = ONE;
        uint256 init_x = ONE;

        LogNormParameters memory logNormParams = LogNormParameters({
            strike: ONE,
            sigma: ONE,
            tau: ONE,
            swapFee: TEST_SWAP_FEE
        });
        bytes memory logNormInitData =
            logNormSolver.getInitialPoolData(init_x, init_p, logNormParams);

        IMultiCore.InitParams memory logNormInitParams;
        logNormInitParams.strategy = address(logNormal);
        logNormInitParams.tokenX = tokenX;
        logNormInitParams.tokenY = tokenY;
        logNormInitParams.data = logNormInitData;

        (LN_POOL_ID,,,) = dfmm.init(logNormInitParams);

        bytes memory g3mInitData =
            g3mSolver.getInitialPoolData(init_x, init_p, g3mParams);
        IMultiCore.InitParams memory g3mInitParams;
        g3mInitParams.strategy = address(g3m);
        g3mInitParams.tokenX = tokenX;
        g3mInitParams.tokenY = tokenY;
        g3mInitParams.data = g3mInitData;

        (G3M_POOL_ID,,,) = dfmm.init(g3mInitParams);

        _;
    }

    function test_multi_internal_price() public basic {
        uint256 internalPrice = logNormSolver.internalPrice(LN_POOL_ID);

        console2.log(internalPrice);
    }

    function test_multi_internal_price_post_y_in() public basic {
        uint256 internalPrice = logNormSolver.internalPrice(LN_POOL_ID);
        uint256 amountIn = 0.1 ether;
        bool swapXIn = false;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            logNormSolver.simulateSwap(LN_POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(LN_POOL_ID, payload);

        uint256 postSwapInternalPrice = logNormSolver.internalPrice(LN_POOL_ID);

        assertGt(postSwapInternalPrice, internalPrice);
    }

    function test_multi_internal_price_post_x_in() public basic {
        uint256 internalPrice = logNormSolver.internalPrice(LN_POOL_ID);
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            logNormSolver.simulateSwap(LN_POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(LN_POOL_ID, payload);

        uint256 postSwapInternalPrice = logNormSolver.internalPrice(LN_POOL_ID);

        assertLt(postSwapInternalPrice, internalPrice);
    }

    function test_multi_swap_eth_backtest() public realisticEth {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            logNormSolver.simulateSwap(LN_POOL_ID, swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(LN_POOL_ID, payload);
    }

    function test_allocate_multiple_times() public basic {
        uint256 amountX = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) =
            logNormSolver.allocateGivenX(LN_POOL_ID, amountX);

        uint256 preBalance = dfmm.balanceOf(address(this), LN_POOL_ID);
        IMultiCore.Pool memory pool = dfmm.getPool(LN_POOL_ID);
        uint256 deltaLiquidity = L - pool.totalLiquidity;
        bytes memory data = abi.encode(rx, ry, L);
        dfmm.allocate(LN_POOL_ID, data);
        assertEq(
            preBalance + deltaLiquidity,
            dfmm.balanceOf(address(this), LN_POOL_ID)
        );

        (rx, ry, L) = logNormSolver.allocateGivenX(LN_POOL_ID, amountX * 2);
        IMultiCore.Pool memory postPool = dfmm.getPool(LN_POOL_ID);
        deltaLiquidity = L - postPool.totalLiquidity;
        data = abi.encode(rx, ry, L);

        MockERC20(tokenX).mint(address(0xbeef), rx);
        MockERC20(tokenY).mint(address(0xbeef), ry);

        vm.startPrank(address(0xbeef));
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
        dfmm.allocate(LN_POOL_ID, data);
        assertEq(deltaLiquidity, dfmm.balanceOf(address(0xbeef), LN_POOL_ID));
        vm.stopPrank();
    }
}
