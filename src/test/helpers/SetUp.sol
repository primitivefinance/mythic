// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "../../DFMM.sol";
import "../helpers/Lex.sol";
import "../helpers/MockStrategy.sol";

contract SetUp is Test {
    DFMM dfmm;
    Lex lex;
    MockERC20 tokenX;
    MockERC20 tokenY;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public virtual {
        tokenX = new MockERC20("tokenX", "X", 18);
        tokenY = new MockERC20("tokenY", "Y", 18);
        tokenX.mint(address(this), 100e18);
        tokenY.mint(address(this), 100e18);

        lex = new Lex(address(tokenX), address(tokenY), 1 ether);
        dfmm = new DFMM();

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    function getPoolLiquidityToken(uint256 poolId)
        public
        view
        returns (address)
    {
        IDFMM.Pool memory pool = dfmm.getPool(poolId);
        return pool.liquidityToken;
    }
}
