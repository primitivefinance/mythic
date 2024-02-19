// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "src/interfaces/IDFMM.sol";
import "src/interfaces/IStrategy.sol";
import "src/lib/DynamicParamLib.sol";
import "src/lib/StrategyLib.sol";

contract ConstantSum is IStrategy {
    using FixedPointMathLib for uint256;
    using DynamicParamLib for DynamicParam;

    struct InternalParams {
        DynamicParam price;
        uint256 swapFee;
        address controller;
    }

    struct ConstantSumParams {
        uint256 price;
        uint256 swapFee;
        address controller;
    }

    address public dfmm;

    mapping(uint256 => InternalParams) public internalParams;

    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    function init(
        address,
        uint256 poolId,
        bytes calldata data
    )
        public
        override
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        ConstantSumParams memory params;
        (reserveX, reserveY, totalLiquidity, params) =
            abi.decode(data, (uint256, uint256, uint256, ConstantSumParams));

        internalParams[poolId].price.lastComputedValue = params.price;

        // Get the trading function and check this is valid
        invariant = ConstantSumLib.tradingFunction(
            reserveX, reserveY, totalLiquidity, params.price
        );

        // Doing this for now
        return (true, 0, reserveX, reserveY, totalLiquidity);
    }
}
