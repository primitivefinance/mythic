/// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "solmate/utils/SafeTransferLib.sol";
import "solstat/Units.sol";
import "./interfaces/IDFMM.sol";
import "./interfaces/IStrategy.sol";

/// @title DFMM
/// @notice Dynamic Function Market Maker
contract DFMM is IDFMM {
    using FixedPointMathLib for uint256;

    Pool[] public pools;
    uint256 private _locked = 1;

    /// @inheritdoc IDFMM
    mapping(address account => mapping(uint256 poolId => uint256 balance))
        public balanceOf;

    mapping(address account => mapping(uint256 poolId => uint256 lastFeeGrowth))
        public lastFeeGrowthOf;

    modifier initialized(uint256 poolId) {
        if (!pools[poolId].inited) revert NotInitialized();
        _;
    }

    modifier lock() {
        if (_locked == 2) revert Locked();
        _locked = 2;
        _;
        _locked = 1;
    }

    function multicall(bytes[] memory data) external returns (bytes[] memory) {
        bytes[] memory results = new bytes[](data.length);

        for (uint256 i = 0; i == data.length;) {
            (bool success, bytes memory result) =
                address(this).delegatecall(data[i]);

            if (!success) {
                if (result.length == 0) revert();
                assembly {
                    revert(add(32, result), mload(result))
                }
            }

            results[i] = result;

            unchecked {
                ++i;
            }
        }

        return results;
    }

    /// @inheritdoc IDFMM
    function init(InitParams calldata params)
        external
        lock
        returns (uint256, uint256, uint256, uint256)
    {
        if (params.tokenX == params.tokenY) revert InvalidTokens();

        (
            bool valid,
            int256 swapConstantGrowth,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        ) = IStrategy(params.strategy).init(pools.length, params.data);

        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        Pool memory pool = Pool({
            inited: true,
            controller: msg.sender,
            strategy: params.strategy,
            tokenX: params.tokenX,
            tokenY: params.tokenY,
            reserveX: reserveX,
            reserveY: reserveY,
            totalLiquidity: totalLiquidity,
            feeGrowth: FixedPointMathLib.WAD
        });

        pools.push(pool);
        uint256 poolId = pools.length - 1;

        balanceOf[msg.sender][poolId] = totalLiquidity;
        lastFeeGrowthOf[msg.sender][poolId] = FixedPointMathLib.WAD;

        SafeTransferLib.safeTransferFrom(
            ERC20(params.tokenX), msg.sender, address(this), reserveX
        );
        SafeTransferLib.safeTransferFrom(
            ERC20(params.tokenY), msg.sender, address(this), reserveY
        );

        emit Init(
            msg.sender,
            params.strategy,
            poolId,
            reserveX,
            reserveY,
            totalLiquidity
        );

        return (poolId, reserveX, reserveY, totalLiquidity);
    }

    /// @inheritdoc IDFMM
    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external lock initialized(poolId) returns (uint256, uint256, uint256) {
        _updateBalance(poolId);

        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePoolReserves(poolId, true, data);

        balanceOf[msg.sender][poolId] += deltaL;
        lastFeeGrowthOf[msg.sender][poolId] = pools[poolId].feeGrowth;

        SafeTransferLib.safeTransferFrom(
            ERC20(pools[poolId].tokenX), msg.sender, address(this), deltaX
        );
        SafeTransferLib.safeTransferFrom(
            ERC20(pools[poolId].tokenY), msg.sender, address(this), deltaY
        );

        emit Allocate(msg.sender, poolId, deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    /// @inheritdoc IDFMM
    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external lock initialized(poolId) returns (uint256, uint256, uint256) {
        _updateBalance(poolId);

        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePoolReserves(poolId, false, data);

        balanceOf[msg.sender][poolId] -= deltaL;
        lastFeeGrowthOf[msg.sender][poolId] = pools[poolId].feeGrowth;

        ERC20(pools[poolId].tokenX).transfer(msg.sender, deltaX);
        ERC20(pools[poolId].tokenY).transfer(msg.sender, deltaY);

        SafeTransferLib.safeTransfer(
            ERC20(pools[poolId].tokenX), msg.sender, deltaX
        );
        SafeTransferLib.safeTransfer(
            ERC20(pools[poolId].tokenY), msg.sender, deltaY
        );

        emit Deallocate(msg.sender, poolId, deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    /// @inheritdoc IDFMM
    function swap(
        uint256 poolId,
        bytes calldata data
    ) external lock initialized(poolId) returns (uint256, uint256) {
        (
            bool valid,
            int256 swapConstantGrowth,
            ,
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedTotalLiquidity
        ) = IStrategy(pools[poolId].strategy).validateSwap(poolId, data);

        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        _updatePoolGrowth(poolId, adjustedTotalLiquidity);

        (bool isSwapXForY,,, uint256 inputAmount, uint256 outputAmount) =
            _settle(poolId, adjustedReserveX, adjustedReserveY);

        emit Swap(msg.sender, poolId, isSwapXForY, inputAmount, outputAmount);

        return (inputAmount, outputAmount);
    }

    /// @inheritdoc IDFMM
    function update(
        uint256 poolId,
        bytes calldata data
    ) external lock initialized(poolId) {
        if (msg.sender != pools[poolId].controller) revert NotController();
        IStrategy(pools[poolId].strategy).update(poolId, data);
    }

    /// @dev Computes the changes in reserves and transfers the tokens in and out.
    function _settle(
        uint256 poolId,
        uint256 adjustedReserveX,
        uint256 adjustedReserveY
    )
        internal
        returns (
            bool isSwapXForY,
            address inputToken,
            address outputToken,
            uint256 inputAmount,
            uint256 outputAmount
        )
    {
        uint256 originalReserveX = pools[poolId].reserveX;
        uint256 originalReserveY = pools[poolId].reserveY;

        isSwapXForY = adjustedReserveX > originalReserveX;

        inputToken = isSwapXForY ? pools[poolId].tokenX : pools[poolId].tokenY;
        outputToken = isSwapXForY ? pools[poolId].tokenY : pools[poolId].tokenX;
        inputAmount = isSwapXForY
            ? adjustedReserveX - originalReserveX
            : adjustedReserveY - originalReserveY;
        outputAmount = isSwapXForY
            ? originalReserveY - adjustedReserveY
            : originalReserveX - adjustedReserveX;

        if (isSwapXForY) {
            require(originalReserveY > adjustedReserveY, "invalid swap");
        } else {
            require(originalReserveX > adjustedReserveX, "invalid swap");
        }

        // Do the state updates to the reserves before calling untrusted addresses.
        pools[poolId].reserveX = adjustedReserveX;
        pools[poolId].reserveY = adjustedReserveY;

        uint256 preInputBalance = ERC20(inputToken).balanceOf(address(this));
        uint256 preOutputBalance = ERC20(outputToken).balanceOf(address(this));

        SafeTransferLib.safeTransferFrom(
            ERC20(inputToken), msg.sender, address(this), inputAmount
        );

        SafeTransferLib.safeTransfer(
            ERC20(outputToken), msg.sender, outputAmount
        );

        uint256 postInputBalance = ERC20(inputToken).balanceOf(address(this));
        uint256 postOutputBalance = ERC20(outputToken).balanceOf(address(this));

        if (postInputBalance < preInputBalance + inputAmount) {
            revert InvalidSwapInputTransfer();
        }

        if (postOutputBalance < preOutputBalance - outputAmount) {
            revert InvalidSwapOutputTransfer();
        }

        return (isSwapXForY, inputToken, outputToken, inputAmount, outputAmount);
    }

    // Internals

    function _updateBalance(uint256 poolId) internal {
        if (
            balanceOf[msg.sender][poolId] != 0
                && pools[poolId].feeGrowth != lastFeeGrowthOf[msg.sender][poolId]
        ) {
            uint256 growth = pools[poolId].feeGrowth.mulWadDown(
                lastFeeGrowthOf[msg.sender][poolId]
            );
            balanceOf[msg.sender][poolId] =
                balanceOf[msg.sender][poolId].mulWadDown(growth);
        }
    }

    function _updatePoolReserves(
        uint256 poolId,
        bool isAllocate,
        bytes calldata data
    ) internal returns (uint256 deltaX, uint256 deltaY, uint256 deltaL) {
        (bool valid, int256 invariant, uint256 rx, uint256 ry, uint256 L) =
        IStrategy(pools[poolId].strategy).validateAllocateOrDeallocate(
            poolId, data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        deltaX = isAllocate
            ? rx - pools[poolId].reserveX
            : pools[poolId].reserveX - rx;
        deltaY = isAllocate
            ? ry - pools[poolId].reserveY
            : pools[poolId].reserveY - ry;
        deltaL = isAllocate
            ? L - pools[poolId].totalLiquidity
            : pools[poolId].totalLiquidity - L;

        pools[poolId].reserveX = rx;
        pools[poolId].reserveY = ry;
        pools[poolId].totalLiquidity = L;
    }

    function _updatePoolGrowth(
        uint256 poolId,
        uint256 adjustedTotalLiquidity
    ) internal {
        uint256 preLiquidity = pools[poolId].totalLiquidity;
        pools[poolId].totalLiquidity = adjustedTotalLiquidity;
        uint256 growth = pools[poolId].totalLiquidity.divWadDown(preLiquidity);
        pools[poolId].feeGrowth = pools[poolId].feeGrowth.mulWadDown(growth);
    }

    // Lens

    function nonce() external view returns (uint256) {
        return pools.length;
    }

    function getPool(uint256 poolId) external view returns (Pool memory) {
        return pools[poolId];
    }

    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256, uint256, uint256)
    {
        return (
            pools[poolId].reserveX,
            pools[poolId].reserveY,
            pools[poolId].totalLiquidity
        );
    }
}
