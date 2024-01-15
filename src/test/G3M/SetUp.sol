// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../strategies/G3M/G3M.sol";
import "../SetUp.sol";
import "./G3MSolver.sol";

contract G3MSetUp is SetUp {
    G3M g3m;
    G3MSolver solver;

    uint256 public POOL_ID;

    G3M.PublicParams defaultParams = G3M.PublicParams({
        wX: 0.5 ether,
        wY: 0.5 ether,
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
        dfmm = new DFMM();
        g3m = new G3M(address(dfmm));
        solver = new G3MSolver(address(g3m));

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    modifier init() {
        vm.warp(0);

        IDFMM.InitParams memory defaultInitParams = IDFMM.InitParams({
            strategy: address(g3m),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: defaultInitialPoolData
        });

        (POOL_ID,,,) = dfmm.init(defaultInitParams);

        _;
    }
}
