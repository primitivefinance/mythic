// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./LogNormalMath.sol";
import "./LogNormal.sol";
import "forge-std/console2.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

enum LogNormalUpdateCode {
    Invalid,
    SwapFee,
    Strike,
    Sigma,
    Tau,
    Controller
}

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.SwapFee, uint256(swapFee));
}

function decodeFeeUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 swapFee) = abi.decode(data, (LogNormalUpdateCode, uint256));
    return swapFee;
}

function encodeStrikeUpdate(
    uint256 targetStrike,
    uint256 targetTimestamp
) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.Strike, targetStrike, targetTimestamp);
}

function decodeStrikeUpdate(bytes memory data)
    pure
    returns (uint256 targetStrike, uint256 targetTimestamp)
{
    (, targetStrike, targetTimestamp) =
        abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
}

function encodeSigmaUpdate(
    uint256 targetSigma,
    uint256 targetTimestamp
) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.Sigma, targetSigma, targetTimestamp);
}

function decodeSigmaUpdate(bytes memory data)
    pure
    returns (uint256 targetSigma, uint256 targetTimestamp)
{
    (, targetSigma, targetTimestamp) =
        abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
}

function encodeTauUpdate(
    uint256 targetTau,
    uint256 targetTimestamp
) pure returns (bytes memory) {
    return abi.encode(LogNormalUpdateCode.Tau, targetTau, targetTimestamp);
}

function decodeTauUpdate(bytes memory data)
    pure
    returns (uint256 targetTau, uint256 targetTimestamp)
{
    (, targetTau, targetTimestamp) =
        abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
}

function encodeControllerUpdate(address controller)
    pure
    returns (bytes memory data)
{
    return abi.encode(LogNormalUpdateCode.Controller, controller);
}

function decodeControllerUpdate(bytes memory data)
    pure
    returns (address controller)
{
    (, controller) = abi.decode(data, (LogNormalUpdateCode, address));
}

function tradingFunction(
    uint256 rx,
    uint256 ry,
    uint256 L,
    LogNormal.LogNormalParams memory params
) pure returns (int256) {
    require(rx < L, "tradingFunction: invalid x");

    int256 AAAAA;
    int256 BBBBB;
    if (FixedPointMathLib.divWadDown(rx, L) >= ONE) {
        AAAAA = int256(2 ** 255 - 1);
        revert ErrorA(AAAAA);
    } else {
        AAAAA = Gaussian.ppf(int256(FixedPointMathLib.divWadDown(rx, L)));
    }
    if (
        FixedPointMathLib.divWadDown(
            ry, FixedPointMathLib.mulWadDown(params.strike, L)
        ) >= ONE
    ) {
        BBBBB = int256(2 ** 255 - 1);
        revert ErrorB(BBBBB);
    } else {
        BBBBB = Gaussian.ppf(
            int256(
                FixedPointMathLib.divWadDown(
                    ry, FixedPointMathLib.mulWadDown(params.strike, L)
                )
            )
        );
    }

    int256 CCCCC = int256(computeSigmaSqrtTau(params.sigma, params.tau));

    return AAAAA + BBBBB + CCCCC;
}

function computeHalfSigmaSquared(uint256 sigma) pure returns (uint256) {
    int256 sigmaSquaredWad = int256(sigma).powWad(int256(TWO));
    return HALF.mulWadDown(uint256(sigmaSquaredWad));
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePriceGivenX(
    uint256 rx,
    uint256 L,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 price) {
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(params.sigma, params.tau);
    uint256 halfSigmaSquared = computeHalfSigmaTauSquared(params.sigma, params.tau);
    uint256 halfSigmaSquaredTau = halfSigmaSquared.mulWadDown(params.tau);

    // Gaussian.ppf has a range of [-inf, inf], so we need to make sure the input is in [0, 1].
    int256 reserveXDivLiquidity = int256(rx.divWadDown(L));
    // As x -> 1, price -> 0.
    if (reserveXDivLiquidity >= int256(ONE)) {
        return 0;
    }
    // As x -> 0, price -> infinity.
    if (reserveXDivLiquidity <= int256(ZERO)) {
        // todo: can returning an infinity price be worse than returning zero or reverting?
        return INFINITY_IS_NOT_REAL;
    }
    // The output can be negative so we have to be careful not to lose that information by casting.
    int256 inverse_cdf_result = Gaussian.ppf(int256(ONE) - reserveXDivLiquidity);
    int256 exponent = inverse_cdf_result * int256(sigmaSqrtTau) / int256(ONE)
        - int256(halfSigmaSquaredTau);

    // This result cannot be negative!
    int256 exp_result = exponent.expWad();
    uint256 exp_result_uint = toUint(exp_result);
    price = params.strike.mulWadUp(exp_result_uint);
}

function computePriceGivenY(
  uint256 ry,
  uint256 L,
  LogNormal.LogNormalParams memory params
) pure returns (uint256 price) {
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(params.sigma, params.tau);
    uint256 halfSigmaSquared = computeHalfSigmaTauSquared(params.sigma, params.tau);
    uint256 halfSigmaSquaredTau = halfSigmaSquared.mulWadDown(params.tau);

    // Gaussian.ppf has a range of [-inf, inf], so we need to make sure the input is in [0, 1].
    int256 yOverKL = int256(ry.divWadDown(params.strike.mulWadDown(L)));
    // As x -> 1, price -> 0.
    if (yOverKL >= int256(ONE)) {
        return 0;
    }
    // As x -> 0, price -> infinity.
    if (yOverKL <= int256(ZERO)) {
        // todo: can returning an infinity price be worse than returning zero or reverting?
        return INFINITY_IS_NOT_REAL;
    }
    // The output can be negative so we have to be careful not to lose that information by casting.
    int256 inverse_cdf_result = Gaussian.ppf(yOverKL);
    int256 exponent = inverse_cdf_result * int256(sigmaSqrtTau) / int256(ONE)
        + int256(halfSigmaSquaredTau);

    // This result cannot be negative!
    int256 exp_result = exponent.expWad();
    uint256 exp_result_uint = toUint(exp_result);
    price = params.strike.mulWadUp(exp_result_uint);
}

/// @dev Casts a positived signed integer to an unsigned integer, reverting if `x` is negative.
function toUint(int256 x) pure returns (uint256) {
    unchecked {
        require(x >= 0, "toUint: negative");
        return uint256(x);
    }
}
