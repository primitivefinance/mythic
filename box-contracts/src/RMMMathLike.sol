// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./lib/RMMMath.sol" as RMMMath;

contract RMMMathLike {
    function toWad(uint256 a) public view returns (uint256) {
        return RMMMath.toWad(a);
    }

    function fromWad(uint256 a) public view returns (uint256) {
        return RMMMath.fromWad(a);
    }

    function computeLGivenX(
        uint256 x,
        uint256 S,
        uint256 K,
        uint256 sigma
    ) public view returns (uint256 L) {
        return RMMMath.computeLGivenX(x, S, K, sigma);
    }

    function computeLGivenY(
        uint256 y,
        uint256 S,
        uint256 K,
        uint256 sigma
    ) public view returns (uint256 L) {
        return RMMMath.computeLGivenY(y, S, K, sigma);
    }

    function computeXGivenL(
        uint256 L,
        uint256 S,
        uint256 K,
        uint256 sigma
    ) public view returns (uint256 x) {
        return RMMMath.computeXGivenL(L, S, K, sigma);
    }

    function computeYGivenL(
        uint256 L,
        uint256 S,
        uint256 K,
        uint256 sigma
    ) public view returns (uint256 y) {
        return RMMMath.computeYGivenL(L, S, K, sigma);
    }

    function computeOutputYGivenX(
        uint256 x,
        uint256 y,
        uint256 deltaX,
        uint256 L,
        uint256 deltaL,
        uint256 K,
        uint256 sigma
    ) public view returns (int256 outputY) {
        return RMMMath.computeOutputYGivenX(
            x, y, deltaX, L, deltaL, K, sigma
        );
    }

    function computeOutputXGivenY(
        uint256 x,
        uint256 y,
        uint256 deltaY,
        uint256 L,
        uint256 deltaL,
        uint256 K,
        uint256 sigma
    ) public view returns (int256 outputX) {
        return RMMMath.computeOutputXGivenY(
            x, y, deltaY, L, deltaL, K, sigma
        );
    }

    function computeSpotPrice(
        uint256 x,
        uint256 L,
        uint256 K,
        uint256 sigma,
        uint256 tau
    ) public view returns (uint256 spotPrice) {
        return RMMMath.computeSpotPrice(x, L, K, sigma, tau);
    }

    function computeInvariant(
        uint256 reserveX,
        uint256 liquidity,
        uint256 reserveY,
        uint256 strikePrice
    ) public view returns (int256 invariant) {
        return
            RMMMath.computeInvariant(reserveX, liquidity, reserveY, strikePrice);
    }
}