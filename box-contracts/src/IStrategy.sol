// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

/// @dev Draft of a generic interface that we could reuse for all strategies.
interface IStrategy {
    // Actions
    function instantiate(uint initial_x_wad, uint initial_price_wad) external;

    // Getters
    function get_spot_price() external view returns(uint spot_price_wad);
    function get_swap_fee() external view returns(uint fee);
    function get_reserve_x() external view returns(uint reserve_x_wad);
    function get_reserve_y() external view returns(uint reserve_y_wad);
    function get_invariant() external view returns(uint invariant);
    function get_strategy_data() external view returns(bytes memory strategy_data);

}
