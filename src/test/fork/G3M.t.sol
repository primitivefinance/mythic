/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "src/DFMM.sol";
import "src/strategies/G3M/G3M.sol";
import "src/solvers/G3M/G3MExtendedLib.sol";

interface USDC {
    function masterMinter() external view returns (address);
    function mint(address to, uint256 amount) external;
    function configureMinter(
        address minter,
        uint256 minterAllowedAmount
    ) external returns (bool);
}

contract G3MTestFork is Test {
    DFMM dfmm;
    ERC20 usdc;
    ERC20 weth;
    ERC20 dai;
    G3M g3m;

    function setUp() public {
        vm.createSelectFork(vm.envString("MAINNET_RPC_URL"));

        usdc = ERC20(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);
        weth = ERC20(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);
        dai = ERC20(0x6B175474E89094C44Da98b954EedeAC495271d0F);

        vm.prank(
            USDC(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48).masterMinter()
        );
        USDC(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48).configureMinter(
            address(this), type(uint256).max
        );
        USDC(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48).mint(
            address(this), 4000 * 10 ** 6
        );
        deal(address(weth), address(this), 1 ether);

        dfmm = new DFMM();
        g3m = new G3M(address(dfmm));

        usdc.approve(address(dfmm), type(uint256).max);
        weth.approve(address(dfmm), type(uint256).max);
    }

    function testFork_G3M_init_USDCWETHPool() public {
        uint256 reserveX = 2 ether;
        uint256 price = 2000 * 10 ** 6;

        G3M.G3MParams memory params = G3M.G3MParams({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: 0,
            controller: address(this)
        });

        dfmm.init(
            IDFMM.InitParams({
                strategy: address(g3m),
                tokenX: address(weth),
                tokenY: address(usdc),
                data: computeInitialPoolData(reserveX, price, params)
            })
        );
    }
}
