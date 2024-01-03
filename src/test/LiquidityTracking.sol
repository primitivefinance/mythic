// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/Vm.sol";
import "forge-std/console2.sol";

contract LiquidityTracking is Test {
    mapping(address => uint256) public balanceOf;
    mapping(address => uint256) public feeGrowthWhenUpdated;

    uint256 public totalFeeGrowth = 1 ether;
    uint256 public totalLiquidity;

    function updatePosition(address user) public {
        if (balanceOf[user] != 0) {
            uint256 growth =
                totalFeeGrowth * 1 ether / feeGrowthWhenUpdated[user];
            balanceOf[user] = balanceOf[user] * growth / 1 ether;
        }

        feeGrowthWhenUpdated[user] = totalFeeGrowth;
    }

    function addLiquidity(uint256 amount) public {
        updatePosition(msg.sender);
        balanceOf[msg.sender] += amount;
        totalLiquidity += amount;
    }

    function removeLiquidity(uint256 amount) public {
        updatePosition(msg.sender);
        balanceOf[msg.sender] -= amount;
        totalLiquidity -= amount;
    }

    function swap() public {
        uint256 preLiquidity = totalLiquidity;
        totalLiquidity += 0.1 ether;

        console.log("totalLiquidity:", totalLiquidity);
        console.log("preLiquidity:", preLiquidity);

        uint256 growth = totalLiquidity * 1 ether / preLiquidity;
        console.log("growth:", growth);
        totalFeeGrowth = totalFeeGrowth * growth / 1 ether;
    }
}

contract LiquidityTrackingTest is Test {
    LiquidityTracking liquidityTracking;

    function setUp() public {
        liquidityTracking = new LiquidityTracking();
    }

    // Alice adds 1 liquidity
    // Swap happens, total liquidity is now 1.1
    // Bob adds 1 liquidity
    // Liquidity is now 2.1
    // Swap happens, total liquidity is now 2.2
    // Now Alice is 1.15 and Bob is 1.05
    // Parameters are changing

    function test_liquidity_update() public {
        vm.prank(address(0xaaaa));

        liquidityTracking.addLiquidity(1 ether);
        liquidityTracking.swap();

        vm.prank(address(0xb0b));
        liquidityTracking.addLiquidity(1 ether);
        liquidityTracking.swap();

        liquidityTracking.updatePosition(address(0xaaaa));
        liquidityTracking.updatePosition(address(0xb0b));

        assertEq(
            liquidityTracking.balanceOf(address(0xaaaa))
                + liquidityTracking.balanceOf(address(0xb0b)),
            2.2 ether
        );

        /*
        liquidityTracking.addLiquidity(1 ether);
        assertEq(liquidityTracking.balanceOf(address(this)), 1 ether);
        assertEq(liquidityTracking.totalLiquidity(), 1 ether);
        assertEq(liquidityTracking.feeGrowthWhenUpdated(address(this)), 1 ether);

        liquidityTracking.swap();
        assertEq(liquidityTracking.balanceOf(address(this)), 1 ether);
        assertEq(liquidityTracking.totalLiquidity(), 1.1 ether);
        assertEq(
            liquidityTracking.totalFeeGrowth(),
            1.1 ether,
            "total fee growth is wrong"
        );

        liquidityTracking.addLiquidity(0);
        assertEq(
            liquidityTracking.balanceOf(address(this)),
            1.1 ether,
            "user balance is wrong"
        );
        assertEq(liquidityTracking.totalLiquidity(), 1.1 ether);
        */
    }
}
