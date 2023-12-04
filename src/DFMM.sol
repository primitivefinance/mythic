// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "forge-std/console2.sol";
import "./v3/BisectionLib.sol";
import "./LogNormal.sol";

/// @title DFMM
/// @notice Dynamic Function Market Maker
contract DFMM is Core {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    address public source;
    bool public inited;
    uint256 public locked = 1;
    address public tokenX;
    address public tokenY;
    uint256 public reserveXWad;
    uint256 public reserveYWad;
    uint256 public totalLiquidity;
    mapping(address account => uint256 balance) public balanceOf;

    constructor(
        address tokenX_,
        address tokenY_,
        uint256 swapFeePercentageWad
    ) {
        tokenX = tokenX_;
        tokenY = tokenY_;

        // todo: can update later to allow for different sources.
        source = address(new LogNormal(swapFeePercentageWad));
    }

    error Invalid(bool negative, uint256 swapConstantGrowth);

    event Swap(
        address indexed swapper,
        address source,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        int256 liquidityDelta
    );

    modifier initialized() {
        require(inited, "not initialized");
        _;
    }

    modifier lock() {
        require(locked == 1, "locked");
        locked = 0;
        _;
        locked = 1;
    }

    function getSwapConstant() public view returns (int256) {
        bytes memory data = abi.encode(reserveXWad, reserveYWad, totalLiquidity);
        return LogNormal(source).computeSwapConstant(data);
    }

    function getReservesAndLiquidity()
        public
        view
        returns (uint256, uint256, uint256)
    {
        return (reserveXWad, reserveYWad, totalLiquidity);
    }

    /// @dev Gets the approximated price of the pool given x reserves and liquidity.
    function internalPrice() public view returns (uint256 price) {
        price = LogNormal(source).internalPrice(reserveXWad, totalLiquidity);
    }

    /// @param data The data to be passed to the source strategy contract for pool initialization & validation.
    function init(bytes calldata data)
        public
        lock
        returns (uint256, uint256, uint256)
    {
        (
            bool valid,
            int256 swapConstantGrowth,
            uint256 XXXXXXX,
            uint256 YYYYYY,
            uint256 LLLLLL
        ) = Source(source).init(data);
        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }
        inited = true;
        reserveXWad = XXXXXXX;
        reserveYWad = YYYYYY;
        totalLiquidity = LLLLLL;
        balanceOf[msg.sender] = LLLLLL;
        ERC20(tokenX).transferFrom(msg.sender, address(this), XXXXXXX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), YYYYYY);
        return (XXXXXXX, YYYYYY, LLLLLL);
    }

    /// @dev Use this function to prepare swaps!
    /// @param swapXIn Whether the swap is X in, Y out.
    /// @param amountIn The amount of the input token to swap.
    /// @return valid Whether the swap is valid, as returned by source.validate().
    /// @return estimatedOut The estimated amount of the output token.
    /// @return estimatedPrice The computed price after the swap.
    /// @return swapData The data to be passed to the source strategy contract for swap validation.
    function simulateSwap(
        bool swapXIn,
        uint256 amountIn
    )
        public
        view
        returns (
            bool valid,
            uint256 estimatedOut,
            uint256 estimatedPrice,
            bytes memory swapData
        )
    {
        return LogNormal(source).simulateSwap(swapXIn, amountIn);
    }

    /// @param data The data to be passed to the source strategy contract for swap validation.
    function swap(bytes calldata data) public lock initialized {
        (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta,
            uint256 XXXXXXX,
            uint256 YYYYYY,
            uint256 LLLLLL
        ) = Source(source).validate(data);
        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        totalLiquidity = LLLLLL;

        {
            // Avoids stack too deep.
            (
                address inputToken,
                address outputToken,
                uint256 inputAmount,
                uint256 outputAmount
            ) = _settle({
                adjustedReserveXWad: XXXXXXX,
                adjustedReserveYWad: YYYYYY
            });

            address swapper = msg.sender;
            address strategy = source;
            int256 liquidityDelta = liquidityDelta;
            emit Swap(
                swapper,
                strategy,
                inputToken,
                outputToken,
                inputAmount,
                outputAmount,
                liquidityDelta
            );
        }
    }

    /// @dev Computes the changes in reserves and transfers the tokens in and out.
    function _settle(
        uint256 adjustedReserveXWad,
        uint256 adjustedReserveYWad
    )
        internal
        returns (
            address inputToken,
            address outputToken,
            uint256 inputAmount,
            uint256 outputAmount
        )
    {
        (uint256 originalReserveXWad, uint256 originalReserveYWad) =
            (reserveXWad, reserveYWad);

        if (adjustedReserveXWad > originalReserveXWad) {
            inputToken = tokenX;
            outputToken = tokenY;
            inputAmount = adjustedReserveXWad - originalReserveXWad;
            require(
                originalReserveYWad > adjustedReserveYWad,
                "invalid swap: inputs x and y"
            );
            outputAmount = originalReserveYWad - adjustedReserveYWad;
        } else {
            inputToken = tokenY;
            outputToken = tokenX;
            inputAmount = adjustedReserveYWad - originalReserveYWad;
            require(
                originalReserveXWad > adjustedReserveXWad,
                "invalid swap: inputs x and y"
            );
            outputAmount = originalReserveXWad - adjustedReserveXWad;
        }

        // Do the state updates to the reserves before calling untrusted addresses.
        reserveXWad = adjustedReserveXWad;
        reserveYWad = adjustedReserveYWad;

        uint256 inputBalance = ERC20(inputToken).balanceOf(address(this));
        uint256 outputBalance = ERC20(outputToken).balanceOf(address(this));
        ERC20(inputToken).transferFrom(msg.sender, address(this), inputAmount);
        ERC20(outputToken).transfer(msg.sender, outputAmount);
        require(
            ERC20(inputToken).balanceOf(address(this))
                >= inputBalance + inputAmount,
            "invalid swap: input token transfer"
        );
        require(
            ERC20(outputToken).balanceOf(address(this))
                >= outputBalance - outputAmount,
            "invalid swap: output token transfer"
        );
    }
}
