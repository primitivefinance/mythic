// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../MultiDFMM.sol";
import "./helpers/Lex.sol";

contract SetUp is Test {
    MultiDFMM dfmm;
    Lex lex;
    MockERC20 tokenX;
    MockERC20 tokenY;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = new MockERC20("tokenX", "X", 18);
        tokenY = new MockERC20("tokenY", "Y", 18);
        tokenX.mint(address(this), 100e18);
        tokenY.mint(address(this), 100e18);

        lex = new Lex(address(tokenX), address(tokenY), 1 ether);
        dfmm = new MultiDFMM();

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    function getPool(uint256 poolId)
        public
        view
        returns (IMultiCore.Pool memory pool)
    {
        (
            pool.inited,
            pool.controller,
            pool.strategy,
            pool.tokenX,
            pool.tokenY,
            pool.reserveX,
            pool.reserveY,
            pool.totalLiquidity,
            pool.feeGrowth
        ) = dfmm.pools(poolId);
    }
}
