// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../DFMM.sol";
import "../solvers/LogNormalSolver.sol";
import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../Lex.sol";

contract LogNormalTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    LogNormalSolver solver;
    address tokenX;
    address tokenY;
    Lex lex;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100e18);
        MockERC20(tokenY).mint(address(this), 100000e18);

        lex = new Lex(tokenX, tokenY, ONE);

        dfmm = new DFMM(true, tokenX, tokenY, TEST_SWAP_FEE);
        solver = new LogNormalSolver(address(dfmm.strategy()));
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

        dfmm.init(initData);

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

        dfmm.init(initData);

        _;
    }

    function test_dfmm_swap_x_in() public basic {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(payload);
    }

    function test_dfmm_swap_y_in() public basic {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = false;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(payload);
    }

    function test_internal_price() public basic {
        uint256 internalPrice = solver.internalPrice();

        console2.log(internalPrice);
    }

    function test_internal_price_post_y_in() public basic {
        uint256 internalPrice = solver.internalPrice();
        uint256 amountIn = 0.1 ether;
        bool swapXIn = false;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(payload);

        uint256 postSwapInternalPrice = solver.internalPrice();

        assertGt(postSwapInternalPrice, internalPrice);
    }

    function test_internal_price_post_x_in() public basic {
        uint256 internalPrice = solver.internalPrice();
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(payload);

        uint256 postSwapInternalPrice = solver.internalPrice();

        assertLt(postSwapInternalPrice, internalPrice);
    }

    function test_swap_eth_backtest() public realisticEth {
        uint256 amountIn = 0.1 ether;
        bool swapXIn = true;

        // Try doing simulate swap to see if we get a similar result.
        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(swapXIn, amountIn);

        assertEq(valid, true);

        dfmm.swap(payload);
    }

    function test_allocate_liquidity_given_x() public basic {
        uint256 amountX = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) = solver.allocateGivenX(amountX);

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.allocate(data);
    }

    function test_deallocate_liquidity_given_x() public basic {
        uint256 amountX = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) = solver.deallocateGivenX(amountX);

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.deallocate(data);
    }

    function test_allocate_liquidity_given_y() public basic {
        uint256 amountY = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) = solver.allocateGivenY(amountY);

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.allocate(data);
    }

    function test_deallocate_liquidity_given_y() public basic {
        uint256 amountY = 0.1 ether;
        (uint256 rx, uint256 ry, uint256 L) = solver.deallocateGivenY(amountY);

        bytes memory data = abi.encode(rx, ry, L);
        dfmm.deallocate(data);
    }
}
