// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../../DFMM.sol";
import "../../strategies/ConstantSum/ConstantSum.sol";
import "../../solvers/ConstantSum/ConstantSumSolver.sol";
import "../helpers/Lex.sol";

contract ConstantSumTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    ConstantSum constantSum;
    ConstantSumSolver solver;
    address tokenX;
    address tokenY;
    Lex lex;

    uint256 public constant TEST_ZERO_FEE = 0;
    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100_000_000 ether);
        MockERC20(tokenY).mint(address(this), 100_000_000 ether);

        lex = new Lex(tokenX, tokenY, ONE);
        dfmm = new DFMM();
        constantSum = new ConstantSum(address(dfmm));
        solver = new ConstantSumSolver(address(constantSum));
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    modifier basic_feeless() {
        vm.warp(0);

        ConstantSum.ConstantSumParams memory params = ConstantSum
            .ConstantSumParams({ price: ONE * 2, swapFee: 0, controller: address(0) });

        uint256 init_x = ONE * 1;
        uint256 init_y = ONE * 1;

        bytes memory initData =
            solver.getInitialPoolData(init_x, init_y, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(constantSum),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    modifier basic() {
        vm.warp(0);

        ConstantSum.ConstantSumParams memory params = ConstantSum
            .ConstantSumParams({
            price: ONE * 2,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });

        uint256 init_x = ONE * 1;
        uint256 init_y = ONE * 1;

        bytes memory initData =
            solver.getInitialPoolData(init_x, init_y, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(constantSum),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    function test_init() public basic_feeless {
        uint256 poolId = dfmm.nonce() - 1;
        (ConstantSum.ConstantSumParams memory params) = abi.decode(
            constantSum.getPoolParams(poolId), (ConstantSum.ConstantSumParams)
        );
        assertEq(params.price, 2 ether);
        assertEq(params.swapFee, 0.003 ether);
        assertEq(params.controller, address(0));

        (uint256 initRx, uint256 initRy, uint256 initL) =
            DFMM(dfmm).getReservesAndLiquidity(poolId);

        assertEq(initRx, 1 ether);
        assertEq(initRy, 1 ether);
        assertEq(initL, 1.5 ether);
    }

    function test_constant_sum_swap_x_in_no_fee() public basic_feeless {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);
        console2.log("Valid: ", valid);
        console2.log("AmountOut: ", amountOut);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 0.2 ether);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 1.1 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 0.8 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.5 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_constant_sum_swap_y_in_no_fee() public basic_feeless {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);
        console2.log("Valid: ", valid);
        console2.log("AmountOut: ", amountOut);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 0.05 ether);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 0.95 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 1.1 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.5 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_constant_sum_swap_x_in_invalid() public basic_feeless {
        bool xIn = true;
        uint256 amountIn = 1.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.simulateSwap(poolId, xIn, amountIn);
    }

    function test_constant_sum_swap_y_in_invalid() public basic_feeless {
        bool xIn = false;
        uint256 amountIn = 2.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.simulateSwap(poolId, xIn, amountIn);
    }

    function test_constant_sum_swap_x_in_with_fee() public basic {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);
        console2.log("Valid: ", valid);
        console2.log("AmountOut: ", amountOut);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 0.1994 ether);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 1.1 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 0.8006 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.5003 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_constant_sum_swap_y_in_with_fee() public basic {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);
        console2.log("Valid: ", valid);
        console2.log("AmountOut: ", amountOut);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 0.04985 ether);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 0.95015 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 1.1 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.50015 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_constant_sum_allocate() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 amountX = 0.1 ether;
        uint256 amountY = 0.1 ether;
        (bool valid, bytes memory swapData) =
            solver.simulateAllocateOrDeallocate(poolId, true, amountX, amountY);
        console2.log("Valid: ", valid);
        assert(valid);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 1.1 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 1.1 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.65 ether);

        dfmm.allocate(poolId, swapData);
    }

    function test_constant_sum_deallocate() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 amountX = 0.1 ether;
        uint256 amountY = 0.1 ether;
        (bool valid, bytes memory swapData) =
            solver.simulateAllocateOrDeallocate(poolId, false, amountX, amountY);
        console2.log("Valid: ", valid);
        assert(valid);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 0.9 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 0.9 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.35 ether);

        dfmm.deallocate(poolId, swapData);
    }

    function test_constant_sum_fail_deallocate() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 amountX = 1.2 ether;
        uint256 amountY = 1.2 ether;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.simulateAllocateOrDeallocate(poolId, false, amountX, amountY);
    }
}
