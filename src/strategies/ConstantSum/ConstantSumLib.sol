// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./ConstantSum.sol";
import "src/lib/StrategyLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

library ConstantSumLib {
    enum ConstantSumUpdateCode {
        Invalid,
        SwapFee,
        Price,
        Controller
    }

    function encodeFeeUpdate(uint256 swapFee)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encode(ConstantSumUpdateCode.SwapFee, uint256(swapFee));
    }

    function decodeFeeUpdate(bytes memory data)
        internal
        pure
        returns (uint256)
    {
        (, uint256 swapFee) = abi.decode(data, (ConstantSumUpdateCode, uint256));
        return swapFee;
    }

    function encodePriceUpdate(uint256 newPrice)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encode(ConstantSumUpdateCode.Price, newPrice);
    }

    function decodePriceUpdate(bytes memory data)
        internal
        pure
        returns (uint256 newPrice)
    {
        (, newPrice) = abi.decode(data, (ConstantSumUpdateCode, uint256));
    }

    function encodeControllerUpdate(address controller)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encode(ConstantSumUpdateCode.Controller, controller);
    }

    function decodeControllerUpdate(bytes memory data)
        internal
        pure
        returns (address)
    {
        (, address controller) =
            abi.decode(data, (ConstantSumUpdateCode, address));
        return controller;
    }

    function tradingFunction(
        uint256 reserveX,
        uint256 reserveY,
        uint256 totalLiquidity,
        uint256 price
    ) internal pure returns (int256) {
        return int256(reserveX.divWadUp(totalLiquidity))
            + int256(reserveY.divWadUp(totalLiquidity.mulWadUp(price)))
            - int256(ONE);
    }
}
