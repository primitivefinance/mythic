// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../strategies/LogNormal/LogNormal.sol";
import "../SetUp.sol";
import "./LogNormalSolver.sol";

contract LogNormalSetUp is SetUp {
    LogNormal logNormal;
    LogNormalSolver solver;

    uint256 public POOL_ID;

    LogNormParameters defaultParams = LogNormParameters({
        strike: ONE,
        sigma: ONE,
        tau: ONE,
        swapFee: TEST_SWAP_FEE
    });

    uint256 defaultReserveX = 1 ether;
    uint256 defaultStrikePrice = 1 ether;
    bytes defaultInitialPoolData = computeInitialPoolData(
        defaultReserveX, defaultStrikePrice, defaultParams
    );

    function setUp() public {
        globalSetUp();

        tokenX = new MockERC20("tokenX", "X", 18);
        tokenY = new MockERC20("tokenY", "Y", 18);
        tokenX.mint(address(this), 100e18);
        tokenY.mint(address(this), 100e18);

        lex = new Lex(address(tokenX), address(tokenY), ONE);
        dfmm = new MultiDFMM();
        logNormal = new LogNormal(address(dfmm));
        solver = new LogNormalSolver(address(logNormal));

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    modifier init() {
        vm.warp(0);

        IMultiCore.InitParams memory defaultInitParams = IMultiCore.InitParams({
            strategy: address(logNormal),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: defaultInitialPoolData
        });

        (POOL_ID,,,) = dfmm.init(defaultInitParams);

        _;
    }
}
