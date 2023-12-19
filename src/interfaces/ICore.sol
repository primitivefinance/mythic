// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

/// @dev Contract that holds the reserve and liquidity state.
interface ICore {
    function getReservesAndLiquidity()
        external
        view
        returns (
            uint256 reserveXWad,
            uint256 reserveYWad,
            uint256 totalLiquidity
        );
}
