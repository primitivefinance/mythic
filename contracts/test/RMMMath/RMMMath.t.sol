// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

import "../../src/lib/RMMMath.sol";

contract RMMMathTest is Test {
    uint256 S = 2000 ether;
    uint256 K = 1800 ether;
    uint256 sigma = 0.25 ether;
    uint256 tau = 1 ether;
    uint256 gamma = 30;

    function test_compute_backAndForth() public view {
        uint256 x = 5_000 ether;
        uint256 L = computeLGivenX(x, S, K, sigma, tau);
        uint256 y = computeYGivenL(L, S, K, sigma, tau);

        console.log("y", y);
        console.log("L1", L);
        console.log("L2", computeLGivenY(y, S, K, sigma, tau));
        console.log("x2", computeXGivenL(L, S, K, sigma, tau));
    }

    function test_computeLGivenX() public view {
        uint256 x = 5000 ether;
        console.log(computeLGivenX(x, S, K, sigma, tau));
    }

    function test_computeYGivenL() public view {
        uint256 x = 5000 ether;
        uint256 L = computeLGivenX(x, S, K, sigma, tau);
        uint256 y = computeYGivenL(L, S, K, sigma, tau);
        console.log(y);
    }

    function test_computeLGivenY() public view {
        uint256 y = 5000 ether;
        console.log(computeLGivenY(y, S, K, sigma, tau));
    }

    function test_computeXGivenL() public view {
        uint256 x = 5000 ether;
        uint256 L = computeLGivenX(x, S, K, sigma, tau);
        console.log(computeXGivenL(L, S, K, sigma, tau));
    }

    function test_computeSpotPrice() public view {
        uint256 reserveX = 5_000 ether;

        uint256 liquidity = computeLGivenX(reserveX, S, K, sigma, tau);
        console.log("liquidity:", liquidity);
        console.log(computeSpotPrice(reserveX, liquidity, K, sigma, tau));
    }

    function test_computeOutputYGivenX() public view {
        uint256 reserveX = 5_000 ether;
        uint256 deltaX = 500 ether;
        uint256 fees = deltaX * (10_000 - 30) / 10_000;
        uint256 deltaX2 = deltaX - fees;
        console.log("deltaX2:", deltaX2);

        uint256 L = computeLGivenX(reserveX, S, K, sigma, tau);
        console.log("L:", L);
        uint256 reserveY = computeYGivenL(L, S, K, sigma, tau);
        console.log("reserveY", reserveY);
        uint256 deltaL = computeLGivenX(deltaX2, S, K, sigma, tau);
        console.log("deltaL:", deltaL);
        uint256 dy = computeYGivenL(deltaL, S, K, sigma, tau);
        console.log("dy:", dy);

        uint256 KL = FixedPointMathLib.mulWadDown(K, L + deltaL);
        console.log("KL", KL);

        int256 ppf = Gaussian.ppf(
            int256(FixedPointMathLib.divWadDown(reserveX + deltaX, L + deltaL))
        );

        console.log("ppf");
        console.logInt(ppf);
        int256 cdf = Gaussian.cdf(
            -int256(sigma)
                - Gaussian.ppf(
                    int256(
                        FixedPointMathLib.divWadDown(reserveX + deltaX, L + deltaL)
                    )
                )
        );

        console.log("cdf");
        console.logInt(cdf);

        console.log(
            "KL*cdf", uint256(FixedPointMathLib.mulWadDown(KL, uint256(cdf)))
        );
        console.logInt(
            int256(FixedPointMathLib.mulWadDown(KL, uint256(cdf)))
                - int256(reserveY) - int256(dy)
        );

        console.logInt(
            computeOutputYGivenX(
                reserveX, deltaX, reserveY, dy, L, deltaL, K, sigma
            )
        );
    }
}
