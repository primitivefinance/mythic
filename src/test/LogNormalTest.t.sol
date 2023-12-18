// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../DFMM.sol";
import "../Solver.sol";
import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../Lex.sol";

contract LogNormalTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    Solver solver;
    address tokenX;
    address tokenY;
    Lex lex;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100e18);
        MockERC20(tokenY).mint(address(this), 100e18);

        lex = new Lex(tokenX, tokenY, ONE);

        dfmm = new DFMM(tokenX, tokenY, TEST_SWAP_FEE);
        solver = new Solver(address(dfmm.source()));
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);
        Parameters memory params = Parameters({
            strikePriceWad: ONE,
            sigmaPercentWad: ONE,
            tauYearsWad: ONE
        });
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
}
