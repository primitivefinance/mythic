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

    function test_constant_sum_init() public basic { }

    function test_constant_sum_swap_x_in() public basic {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);
        console2.log("Valid: ", valid);
        console2.log("AmountOut: ", amountOut);
        dfmm.swap(poolId, swapData);
    }

    function test_constant_sum_swap_y_in() public basic {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);
        console2.log("Valid: ", valid);
        console2.log("AmountOut: ", amountOut);
        dfmm.swap(poolId, swapData);
    }
}
