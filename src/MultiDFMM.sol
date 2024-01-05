/// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "./strategies/G3M.sol";
import "./strategies/LogNormal.sol";
import "solmate/tokens/ERC20.sol";
import "./interfaces/IMultiCore.sol";

struct Pool {
    bool inited;
    address controller;
    address strategy;
    address tokenX;
    address tokenY;
    uint256 reserveXWad;
    uint256 reserveYWad;
    uint256 totalLiquidity;
    uint256 feeGrowth;
    uint256 swapFeePercentageWad;
}

/// @title MultiDFMM
/// @notice Dynamic Function Market Maker
contract MultiDFMM is IMultiCore {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    uint256 public locked = 1;
    Pool[] public pools;

    mapping(address account => mapping(uint256 poolId => uint256 balance))
        public balanceOf;
    mapping(address account => mapping(uint256 poolId => uint256 balance))
        public feeGrowthLast;

    modifier initialized(uint256 poolId) {
        if (!pools[poolId].inited) revert NotInitialized();
        _;
    }

    modifier lock() {
        require(locked == 1, "locked");
        locked = 0;
        _;
        locked = 1;
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        return (
            pools[poolId].reserveXWad,
            pools[poolId].reserveYWad,
            pools[poolId].totalLiquidity
        );
    }

    function init(InitParams calldata params)
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
        ) = IStrategy(params.strategy).init(params.data);
        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        Pool memory pool = Pool({
            inited: true,
            controller: msg.sender,
            strategy: params.strategy,
            tokenX: params.tokenX,
            tokenY: params.tokenY,
            reserveXWad: XXXXXXX,
            reserveYWad: YYYYYY,
            totalLiquidity: LLLLLL,
            feeGrowth: 1 ether,
            swapFeePercentageWad: params.swapFeePercentageWad
        });

        pools.push(pool);

        balanceOf[msg.sender][pools.length - 1] = LLLLLL;
        feeGrowthLast[msg.sender][pools.length - 1] = 1 ether;

        ERC20(params.tokenX).transferFrom(msg.sender, address(this), XXXXXXX);
        ERC20(params.tokenY).transferFrom(msg.sender, address(this), YYYYYY);
        emit Init(msg.sender, params.strategy, XXXXXXX, YYYYYY, LLLLLL);
        return (XXXXXXX, YYYYYY, LLLLLL);
    }

    function _updateBalance(uint256 poolId) internal {
        if (
            balanceOf[msg.sender][poolId] != 0
                && pools[poolId].feeGrowth != feeGrowthLast[msg.sender][poolId]
        ) {
            uint256 growth = pools[poolId].feeGrowth.mulWadDown(
                feeGrowthLast[msg.sender][poolId]
            );
            balanceOf[msg.sender][poolId] =
                balanceOf[msg.sender][poolId].mulWadDown(growth);
        }
    }

    function _updatePool(
        uint256 poolId,
        bool isAllocate,
        bytes calldata data
    ) internal returns (uint256 deltaX, uint256 deltaY, uint256 deltaL) {
        (bool valid, int256 invariant, uint256 rx, uint256 ry, uint256 L) =
            IStrategy(pools[poolId].strategy).validateAllocateOrDeallocate(data);

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        deltaX = isAllocate
            ? rx - pools[poolId].reserveXWad
            : pools[poolId].reserveXWad - rx;
        deltaY = isAllocate
            ? ry - pools[poolId].reserveYWad
            : pools[poolId].reserveYWad - ry;
        deltaL = isAllocate
            ? L - pools[poolId].totalLiquidity
            : pools[poolId].totalLiquidity - L;

        pools[poolId].reserveXWad = rx;
        pools[poolId].reserveYWad = ry;
        pools[poolId].totalLiquidity = L;
    }

    function allocate(
        uint256 poolId,
        bytes calldata data
    ) public lock returns (uint256, uint256, uint256) {
        _updateBalance(poolId);

        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePool(poolId, true, data);

        balanceOf[msg.sender][poolId] += deltaL;
        feeGrowthLast[msg.sender][poolId] = pools[poolId].feeGrowth;

        ERC20(pools[poolId].tokenX).transferFrom(
            msg.sender, address(this), deltaX
        );
        ERC20(pools[poolId].tokenY).transferFrom(
            msg.sender, address(this), deltaY
        );

        emit Allocate(deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) public lock returns (uint256, uint256, uint256) {
        _updateBalance(poolId);
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePool(poolId, false, data);

        balanceOf[msg.sender][poolId] -= deltaL;
        feeGrowthLast[msg.sender][poolId] = pools[poolId].feeGrowth;

        ERC20(pools[poolId].tokenX).transfer(msg.sender, deltaX);
        ERC20(pools[poolId].tokenY).transfer(msg.sender, deltaY);

        emit Deallocate(deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    /// @param data The data to be passed to the source strategy contract for swap validation.
    function swap(
        uint256 poolId,
        bytes calldata data
    ) public lock initialized(poolId) {
        (
            bool valid,
            int256 swapConstantGrowth,
            int256 liquidityDelta, // this is unused, should we remove it?
            uint256 XXXXXXX,
            uint256 YYYYYY,
            uint256 LLLLLL
        ) = IStrategy(pools[poolId].strategy).validateSwap(data);
        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        uint256 preLiquidity = pools[poolId].totalLiquidity;
        pools[poolId].totalLiquidity = LLLLLL;
        uint256 growth = pools[poolId].totalLiquidity.divWadDown(preLiquidity);
        pools[poolId].feeGrowth = pools[poolId].feeGrowth.mulWadDown(growth);

        _settle({
            poolId: poolId,
            adjustedReserveXWad: XXXXXXX,
            adjustedReserveYWad: YYYYYY
        });
    }

    /// @dev Computes the changes in reserves and transfers the tokens in and out.
    function _settle(
        uint256 poolId,
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
            (pools[poolId].reserveXWad, pools[poolId].reserveYWad);

        if (adjustedReserveXWad > originalReserveXWad) {
            inputToken = pools[poolId].tokenX;
            outputToken = pools[poolId].tokenY;
            inputAmount = adjustedReserveXWad - originalReserveXWad;
            require(
                originalReserveYWad > adjustedReserveYWad,
                "invalid swap: inputs x and y"
            );
            outputAmount = originalReserveYWad - adjustedReserveYWad;
        } else {
            inputToken = pools[poolId].tokenY;
            outputToken = pools[poolId].tokenX;
            inputAmount = adjustedReserveYWad - originalReserveYWad;
            require(
                originalReserveXWad > adjustedReserveXWad,
                "invalid swap: inputs x and y"
            );
            outputAmount = originalReserveXWad - adjustedReserveXWad;
        }

        // Do the state updates to the reserves before calling untrusted addresses.
        pools[poolId].reserveXWad = adjustedReserveXWad;
        pools[poolId].reserveYWad = adjustedReserveYWad;

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
