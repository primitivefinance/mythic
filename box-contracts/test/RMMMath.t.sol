// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

import "../src/lib/RMMMath.sol";

contract RMMMathTest is Test {
    uint256 S = 2000 ether;
    uint256 K = 1800 ether;
    int256 sigma = 0.05 ether;

    function test_computeLGivenX() public view {
        uint256 x = 5000 ether;

        int256 lnSDivK =
            FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
        console.logInt(lnSDivK);
        uint256 halfSigmaPowTwo = FixedPointMathLib.mulWadUp(
            HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO)))
        );
        console.log(halfSigmaPowTwo);

        int256 plus = lnSDivK + int256(halfSigmaPowTwo);
        console.logInt(plus);
        int256 div = plus * 1e18 / int256(sigma);
        console.logInt(div);
        int256 cdf = Gaussian.cdf(div);
        console.logInt(cdf);
        int256 denominator = int256(1e18) - cdf;
        console.logInt(denominator);
        uint256 L = FixedPointMathLib.divWadUp(x, uint256(denominator));
        console.log(L);

        console.log(computeLGivenX(x, S, K, uint256(sigma)));
    }

    function test_computeYGivenL() public view {
        uint256 x = 5000 ether;
        uint256 L = computeLGivenX(x, S, K, uint256(sigma));
        console.log(L);

        int256 lnSDivK =
            FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
        console.logInt(lnSDivK);
        uint256 halfSigmaPowTwo = FixedPointMathLib.mulWadUp(
            HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO)))
        );
        console.log(halfSigmaPowTwo);
        int256 minus = lnSDivK - int256(halfSigmaPowTwo);
        console.logInt(minus);
        int256 div = minus * 1e18 / int256(sigma);
        console.logInt(div);
        int256 cdf = Gaussian.cdf(div);
        console.logInt(cdf);
        uint256 y = FixedPointMathLib.mulWadUp(
            K, FixedPointMathLib.mulWadUp(L, uint256(cdf))
        );
        console.log(y);
    }

    function test_computeLGivenY() public view {
        uint256 y = 5000 ether;

        int256 lnSDivK =
            FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
        console.logInt(lnSDivK);
        uint256 halfSigmaPowTwo = FixedPointMathLib.mulWadUp(
            HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO)))
        );
        console.log(halfSigmaPowTwo);
        int256 minus = lnSDivK - int256(halfSigmaPowTwo);
        console.logInt(minus);
        int256 divSigma = minus * 1e18 / int256(sigma);
        console.logInt(divSigma);
        int256 cdf = Gaussian.cdf(divSigma);
        console.logInt(cdf);
        uint256 L = FixedPointMathLib.divWadUp(
            y, FixedPointMathLib.mulWadUp(K, uint256(cdf))
        );
        console.log(L);
    }

    function test_computeXGivenL() public view {
        uint256 L = 151 ether;

        int256 lnSDivK =
            FixedPointMathLib.lnWad(int256(FixedPointMathLib.divWadUp(S, K)));
        console.logInt(lnSDivK);
        uint256 halfSigmaPowTwo = FixedPointMathLib.mulWadUp(
            HALF, uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO)))
        );
        console.log(halfSigmaPowTwo);
        int256 plus = lnSDivK + int256(halfSigmaPowTwo);
        console.logInt(plus);
        int256 divSigma = plus * 1e18 / int256(sigma);
        console.logInt(divSigma);
        int256 cdf = Gaussian.cdf(divSigma);
        console.logInt(cdf);
        uint256 x = FixedPointMathLib.mulWadUp(L, uint256(int256(ONE) - cdf));
        console.log(x);
    }

    function test_computeSpotPrice() public {
        uint256 reserveX = 5_000 ether;
        uint256 tau = 1 ether;

        uint256 liquidity = computeLGivenX(reserveX, S, K, uint256(sigma));
        console.log("liquidity:", liquidity);

        uint256 halfSigmaPower2Tau = FixedPointMathLib.mulWadDown(
            HALF,
            FixedPointMathLib.mulWadDown(
                uint256(FixedPointMathLib.powWad(int256(sigma), int256(TWO))),
                tau
            )
        );
        console.log("halfSigmaPower2Tau:", halfSigmaPower2Tau);

        uint256 sigmaSqrtTau = FixedPointMathLib.mulWadDown(
            uint256(sigma), FixedPointMathLib.sqrt(tau)
        ) * 10 ** 9;
        console.log("sigmaSqrtTau:", sigmaSqrtTau);

        uint256 R1 = FixedPointMathLib.divWadDown(reserveX, liquidity);
        console.log("R1", R1);

        uint256 price = FixedPointMathLib.mulWadUp(
            K,
            uint256(
                FixedPointMathLib.expWad(
                    int256(
                        FixedPointMathLib.mulWadDown(
                            uint256(Gaussian.ppf(int256(ONE - R1))),
                            sigmaSqrtTau
                        ) - halfSigmaPower2Tau
                    )
                )
            )
        );
        console.log("price:", price);
    }
}
