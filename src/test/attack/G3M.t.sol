/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "src/DFMM.sol";
import "src/strategies/G3M/G3M.sol";
import "src/solvers/G3M/G3MExtendedLib.sol";

contract G3MAttackTest is Test {
    DFMM dfmm;
    MockERC20 tokenX;
    MockERC20 tokenY;
    G3M g3m;

    function setUp() public {
        tokenX = new MockERC20("Token X", "X", 18);
        tokenY = new MockERC20("Token Y", "Y", 18);
        tokenX.mint(address(this), 100_000 ether);
        tokenY.mint(address(this), 100_000 ether);

        dfmm = new DFMM(address(0));
        g3m = new G3M(address(dfmm));

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    function test_G3M_attack() public {
        uint256 reserveX = 20_000 ether;
        uint256 price = 1 ether;

        G3M.G3MParams memory params = G3M.G3MParams({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: 0,
            controller: address(this)
        });

        uint256 L = computeLGivenX(reserveX, price, params);
        console.log("Liquidity:", L);
        uint256 reserveY = computeYGivenL(L, price, params);
        console.log("Reserve Y", reserveY);
        console.log("Reserve X", reserveX);

        bytes memory data = computeInitialPoolData(reserveX, price, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(g3m),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: data
        });

        (uint256 poolId, uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            dfmm.init(initParams);

        console.log("DeltaX: %s", deltaX);
        console.log("DeltaY: %s", deltaY);
        console.log("DeltaL: %s", deltaL);

        IDFMM.Pool memory pool = dfmm.getPool(poolId);
        uint256 balance = ERC20(pool.liquidityToken).balanceOf(address(this));
        console.log(balance);
    }
}
