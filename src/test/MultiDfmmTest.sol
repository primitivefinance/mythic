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

    function test_basic() public basic { }
}
