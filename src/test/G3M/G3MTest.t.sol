// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console2.sol";
import "solmate/test/utils/mocks/MockERC20.sol";

import "../../strategies/G3M/G3M.sol";
import "../../MultiDFMM.sol";
import "../helpers/Lex.sol";
import "./G3MSolver.sol";

contract G3MTest is Test {
    using stdStorage for StdStorage;

    MultiDFMM dfmm;
    G3M g3m;
    G3MSolver solver;
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
        dfmm = new MultiDFMM();
        g3m = new G3M(address(dfmm));
        solver = new G3MSolver(address(dfmm));

        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);
        G3mParameters memory params = G3mParameters({
            wx: 0.5 ether,
            wy: 0.5 ether,
            swapFee: TEST_SWAP_FEE
        });
        uint256 init_p = ONE;
        uint256 init_x = ONE;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IMultiCore.InitParams memory initParams = IMultiCore.InitParams({
            strategy: address(g3m),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    function test_G3M_init() public {
        G3mParameters memory params = G3mParameters({
            wx: 0.5 ether,
            wy: 0.5 ether,
            swapFee: TEST_SWAP_FEE
        });
        uint256 init_p = ONE;
        uint256 init_x = ONE;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IMultiCore.InitParams memory initParams = IMultiCore.InitParams({
            strategy: address(g3m),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);
    }

    // function test_g3m_swap_x_in() public basic {
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = true;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);
    // }

    // function test_g3m_swap_y_in() public basic {
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = false;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);
    // }

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

    // function test_allocate_liquidity_given_x() public basic {
    //     uint256 amountX = 0.1 ether;
    //     (uint256 rx, uint256 ry, uint256 L) = solver.allocateGivenX(amountX);

    //     uint256 preBalance = dfmm.balanceOf(address(this));
    //     uint256 preTotalLiquidity = dfmm.totalLiquidity();

    //     bytes memory data = abi.encode(rx, ry, L);
    //     dfmm.allocate(data);

    //     uint256 deltaTotalLiquidity = dfmm.totalLiquidity() - preTotalLiquidity;
    //     assertEq(
    //         preBalance + deltaTotalLiquidity, dfmm.balanceOf(address(this))
    //     );
    // }

    // function test_deallocate_liquidity_given_x() public basic {
    //     uint256 amountX = 0.1 ether;
    //     (uint256 rx, uint256 ry, uint256 L) = solver.deallocateGivenX(amountX);

    //     uint256 preBalance = dfmm.balanceOf(address(this));
    //     uint256 preTotalLiquidity = dfmm.totalLiquidity();

    //     bytes memory data = abi.encode(rx, ry, L);
    //     dfmm.deallocate(data);

    //     uint256 deltaTotalLiquidity = preTotalLiquidity - dfmm.totalLiquidity();
    //     assertEq(
    //         preBalance - deltaTotalLiquidity, dfmm.balanceOf(address(this))
    //     );
    // }

    // function test_allocate_liquidity_given_y() public basic {
    //     uint256 amountY = 0.1 ether;
    //     (uint256 rx, uint256 ry, uint256 L) = solver.allocateGivenY(amountY);

    //     bytes memory data = abi.encode(rx, ry, L);
    //     dfmm.allocate(data);
    // }

    // function test_deallocate_liquidity_given_y() public basic {
    //     uint256 amountY = 0.1 ether;
    //     (uint256 rx, uint256 ry, uint256 L) = solver.deallocateGivenY(amountY);

    //     bytes memory data = abi.encode(rx, ry, L);
    //     dfmm.deallocate(data);
    // }
}
