// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/utils/FixedPointMathLib.sol";
import "solmate/utils/SafeTransferLib.sol";
import "solstat/Units.sol";
import "./interfaces/IDFMM.sol";
import "./interfaces/IStrategy.sol";
import "./LPToken.sol";

/// @title DFMM
/// @notice Dynamic Function Market Maker
contract DFMM is IDFMM {
    using FixedPointMathLib for uint256;

    /// @inheritdoc IDFMM
    Pool[] public pools;

    /// @inheritdoc IDFMM
    address public immutable lpTokenImplementation;

    uint256 private _locked = 1;

    modifier lock() {
        if (_locked == 2) revert Locked();
        _locked = 2;
        _;
        _locked = 1;
    }

    constructor() {
        lpTokenImplementation = address(new LPToken());
        LPToken(lpTokenImplementation).initialize("", "");
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
        ) = IStrategy(params.strategy).init(
            msg.sender, pools.length, params.data
        );

        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        LPToken liquidityToken = LPToken(clone(lpTokenImplementation));

        // TODO: Add name / symbol
        liquidityToken.initialize("", "");
        liquidityToken.mint(msg.sender, totalLiquidity);
        // TODO: Burn some initial liquidity

        Pool memory pool = Pool({
            strategy: params.strategy,
            tokenX: params.tokenX,
            tokenY: params.tokenY,
            reserveX: reserveX,
            reserveY: reserveY,
            totalLiquidity: totalLiquidity,
            liquidityToken: address(liquidityToken)
        });

        pools.push(pool);
        uint256 poolId = pools.length - 1;

        SafeTransferLib.safeTransferFrom(
            ERC20(params.tokenX), msg.sender, address(this), reserveX
        );
        SafeTransferLib.safeTransferFrom(
            ERC20(params.tokenY), msg.sender, address(this), reserveY
        );

        emit Init(
            msg.sender,
            params.strategy,
            params.tokenX,
            params.tokenY,
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
    ) external lock returns (uint256, uint256, uint256) {
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePoolReserves(poolId, true, data);

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
    ) external lock returns (uint256, uint256, uint256) {
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePoolReserves(poolId, false, data);

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
    ) external lock returns (uint256, uint256) {
        (
            bool valid,
            int256 swapConstantGrowth,
            ,
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedTotalLiquidity
        ) = IStrategy(pools[poolId].strategy).validateSwap(
            msg.sender, poolId, data
        );

        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, abs(swapConstantGrowth));
        }

        pools[poolId].totalLiquidity = adjustedTotalLiquidity;

        (bool isSwapXForY,,, uint256 inputAmount, uint256 outputAmount) =
            _settle(poolId, adjustedReserveX, adjustedReserveY);

        emit Swap(msg.sender, poolId, isSwapXForY, inputAmount, outputAmount);

        return (inputAmount, outputAmount);
    }

    /// @inheritdoc IDFMM
    function update(uint256 poolId, bytes calldata data) external lock {
        IStrategy(pools[poolId].strategy).update(msg.sender, poolId, data);
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

        if (isSwapXForY) {
            require(originalReserveY > adjustedReserveY, "invalid swap");
        } else {
            require(originalReserveX > adjustedReserveX, "invalid swap");
        }

        inputToken = isSwapXForY ? pools[poolId].tokenX : pools[poolId].tokenY;
        outputToken = isSwapXForY ? pools[poolId].tokenY : pools[poolId].tokenX;
        inputAmount = isSwapXForY
            ? adjustedReserveX - originalReserveX
            : adjustedReserveY - originalReserveY;
        outputAmount = isSwapXForY
            ? originalReserveY - adjustedReserveY
            : originalReserveX - adjustedReserveX;

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

    function _updatePoolReserves(
        uint256 poolId,
        bool isAllocate,
        bytes calldata data
    ) internal returns (uint256 deltaX, uint256 deltaY, uint256 deltaL) {
        (
            bool valid,
            int256 invariant,
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedTotalLiquidity
        ) = IStrategy(pools[poolId].strategy).validateAllocateOrDeallocate(
            msg.sender, poolId, data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        deltaX = isAllocate
            ? adjustedReserveX - pools[poolId].reserveX
            : pools[poolId].reserveX - adjustedReserveX;
        deltaY = isAllocate
            ? adjustedReserveY - pools[poolId].reserveY
            : pools[poolId].reserveY - adjustedReserveY;
        deltaL = isAllocate
            ? adjustedTotalLiquidity - pools[poolId].totalLiquidity
            : pools[poolId].totalLiquidity - adjustedTotalLiquidity;

        _manageTokens(poolId, isAllocate, deltaL);

        pools[poolId].reserveX = adjustedReserveX;
        pools[poolId].reserveY = adjustedReserveY;
        pools[poolId].totalLiquidity = adjustedTotalLiquidity;
    }

    function _manageTokens(
        uint256 poolId,
        bool isAllocate,
        uint256 deltaL
    ) internal {
        LPToken liquidityToken = LPToken(pools[poolId].liquidityToken);
        uint256 totalSupply = liquidityToken.totalSupply();
        uint256 totalLiquidity = pools[poolId].totalLiquidity;

        if (isAllocate) {
            uint256 amount =
                deltaL.mulWadDown(totalSupply.divWadDown(totalLiquidity));
            liquidityToken.mint(msg.sender, amount);
        } else {
            uint256 amount =
                deltaL.mulWadUp(totalSupply.divWadUp(totalLiquidity));
            liquidityToken.burn(msg.sender, amount);
        }
    }

    /**
     * @dev Deploys and returns the address of a clone that mimics the behaviour of `implementation`.
     *
     * This function uses the create opcode, which should never revert.
     */
    function clone(address implementation)
        internal
        returns (address instance)
    {
        /// @solidity memory-safe-assembly
        assembly {
            // Cleans the upper 96 bits of the `implementation` word, then packs the first 3 bytes
            // of the `implementation` address with the bytecode before the address.
            mstore(
                0x00,
                or(
                    shr(0xe8, shl(0x60, implementation)),
                    0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000
                )
            )
            // Packs the remaining 17 bytes of `implementation` with the bytecode after the address.
            mstore(
                0x20,
                or(shl(0x78, implementation), 0x5af43d82803e903d91602b57fd5bf3)
            )
            instance := create(0, 0x09, 0x37)
        }
        if (instance == address(0)) {
            revert ERC1167FailedCreateClone();
        }
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

    /**
     * @dev This function should NOT be used in a non-view call, as the
     * values can be manipulated. In the future this function might be
     * removed.
     */
    function liquidityOf(
        address account,
        uint256 poolId
    ) public view returns (uint256) {
        LPToken liquidityToken = LPToken(pools[poolId].liquidityToken);
        uint256 balance = liquidityToken.balanceOf(account);
        uint256 totalSupply = liquidityToken.totalSupply();
        uint256 totalLiquidity = pools[poolId].totalLiquidity;
        return balance.mulWadDown(totalLiquidity.divWadDown(totalSupply));
    }
}
