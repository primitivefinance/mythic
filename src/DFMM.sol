// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "./strategies/G3M.sol";
import "./strategies/LogNormal.sol";
import "solmate/tokens/ERC20.sol";

/// @title DFMM
/// @notice Dynamic Function Market Maker
contract DFMM is ICore {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    address public strategy;
    bool public inited;
    uint256 public locked = 1;
    address public tokenX;
    address public tokenY;
    uint256 public reserveXWad;
    uint256 public reserveYWad;
    uint256 public totalLiquidity;

    uint256 public feeGrowth;

    mapping(address account => uint256 balance) public balanceOf;
    mapping(address account => uint256 balance) public feeGrowthLast;

    event LogPoolStats(
        uint256 rx,
        uint256 ry,
        uint256 L,
        int256 invariant,
        uint256 sigma,
        uint256 strike,
        uint256 tau,
        uint256 timestamp
    );

    constructor(
        bool isLogNormal, // temp way to handle either lognorm or g3m
        address tokenX_,
        address tokenY_,
        uint256 swapFeePercentageWad
    ) {
        tokenX = tokenX_;
        tokenY = tokenY_;

        // todo: can update later to allow for different sources.
        if (isLogNormal) {
            strategy = address(new LogNormal(swapFeePercentageWad));
        } else {
            strategy = address(new G3M(swapFeePercentageWad));
        }
    }

    error Invalid(bool negative, uint256 swapConstantGrowth);

    event Init(
        address indexed swapper,
        address indexed strategy,
        uint256 XXXXXXX,
        uint256 YYYYYY,
        uint256 LLLLLL
    );

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

    function getReservesAndLiquidity()
        public
        view
        returns (uint256, uint256, uint256)
    {
        return (reserveXWad, reserveYWad, totalLiquidity);
    }

    function allocate(bytes calldata data)
        public
        lock
        returns (uint256, uint256, uint256)
    {
        (bool valid, int256 invariant, uint256 rx, uint256 ry, uint256 L) =
            IStrategy(strategy).validateAllocationOrDeallocation(data);
        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }
        if (
            balanceOf[msg.sender] != 0 && feeGrowth != feeGrowthLast[msg.sender]
        ) {
            uint256 growth = feeGrowth.mulWadDown(feeGrowthLast[msg.sender]);
            balanceOf[msg.sender] = balanceOf[msg.sender].mulWadDown(growth);
        }
        uint256 deltaX = rx - reserveXWad;
        uint256 deltaY = ry - reserveYWad;
        uint256 deltaL = L - totalLiquidity;
        reserveXWad = rx;
        reserveYWad = ry;
        totalLiquidity = L;
        balanceOf[msg.sender] += deltaL;
        feeGrowthLast[msg.sender] = feeGrowth;
        ERC20(tokenX).transferFrom(msg.sender, address(this), deltaX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), deltaY);
        return (reserveXWad, reserveYWad, totalLiquidity);
    }

    function deallocate(bytes calldata data)
        public
        lock
        returns (uint256, uint256, uint256)
    {
        (bool valid, int256 invariant, uint256 rx, uint256 ry, uint256 L) =
            IStrategy(strategy).validateAllocationOrDeallocation(data);
        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        if (
            balanceOf[msg.sender] != 0 && feeGrowth != feeGrowthLast[msg.sender]
        ) {
            uint256 growth = feeGrowth.mulWadDown(feeGrowthLast[msg.sender]);
            balanceOf[msg.sender] = balanceOf[msg.sender].mulWadDown(growth);
            console2.log("in here");
        }

        uint256 deltaX = reserveXWad - rx;
        uint256 deltaY = reserveYWad - ry;
        uint256 deltaL = totalLiquidity - L;
        reserveXWad = rx;
        reserveYWad = ry;
        totalLiquidity = L;
        balanceOf[msg.sender] -= deltaL;
        feeGrowthLast[msg.sender] = feeGrowth;
        ERC20(tokenX).transfer(msg.sender, deltaX);
        ERC20(tokenY).transfer(msg.sender, deltaY);
        return (reserveXWad, reserveYWad, totalLiquidity);
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
        ) = IStrategy(strategy).init(data);
        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }
        inited = true;
        reserveXWad = XXXXXXX;
        reserveYWad = YYYYYY;
        totalLiquidity = LLLLLL;
        balanceOf[msg.sender] = LLLLLL;
        feeGrowth = 1 ether;
        feeGrowthLast[msg.sender] = feeGrowth;
        ERC20(tokenX).transferFrom(msg.sender, address(this), XXXXXXX);
        ERC20(tokenY).transferFrom(msg.sender, address(this), YYYYYY);
        emit Init(msg.sender, strategy, XXXXXXX, YYYYYY, LLLLLL);
        return (XXXXXXX, YYYYYY, LLLLLL);
    }

    /// @param data The data to be passed to the source strategy contract for swap validation.
    function swap(bytes calldata data) public lock initialized {
        (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta, // this is unused, should we remove it?
            uint256 XXXXXXX,
            uint256 YYYYYY,
            uint256 LLLLLL
        ) = IStrategy(strategy).validate(data);
        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        uint256 preLiquidity = totalLiquidity;
        totalLiquidity = LLLLLL;
        uint256 growth = totalLiquidity.divWadDown(preLiquidity);
        feeGrowth = feeGrowth.mulWadDown(growth);

        {
            _settle({ adjustedReserveXWad: XXXXXXX, adjustedReserveYWad: YYYYYY });

            // bytes memory strategyData = IStrategy(strategy).dynamicSlot();
            // (uint256 strike, uint256 sigma, uint256 tau) =
            //     abi.decode(strategyData, (uint256, uint256, uint256));

            // emit LogPoolStats(
            //     XXXXXXX,
            //     YYYYYY,
            //     LLLLLL,
            //     swapConstantGrowth,
            //     sigma,
            //     strike,
            //     tau,
            //     block.timestamp
            // );
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
