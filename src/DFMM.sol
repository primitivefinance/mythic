// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/utils/FixedPointMathLib.sol";
import "solmate/utils/SafeTransferLib.sol";
import "solmate/utils/LibString.sol";
import { WETH } from "solmate/tokens/WETH.sol";
import "solstat/Units.sol";
import "./interfaces/IDFMM.sol";
import "./interfaces/IStrategy.sol";
import "./lib/ScalingLib.sol";
import "./LPToken.sol";

/**
 * @title DFMM
 * @author Primitive
 * @notice Dynamic Function Market Maker
 */
contract DFMM is IDFMM {
    using FixedPointMathLib for uint256;

    /// @inheritdoc IDFMM
    Pool[] public pools;

    /// @inheritdoc IDFMM
    address public immutable lpTokenImplementation;

    address public immutable weth;

    /// @dev Part of the reentrancy lock, 1 = unlocked, 2 = locked.
    uint256 private _locked = 1;

    /// @dev Amount of liquidity that is burnt on initialization.
    uint256 private constant BURNT_LIQUIDITY = 1000;

    /// @dev Prevents reentrancy.
    modifier lock() {
        if (_locked == 2) revert Locked();
        _locked = 2;
        _;
        _locked = 1;
    }

    receive() external payable {
        if (msg.sender != weth) revert OnlyWETH();
    }

    /**
     * @dev The implementation of the LPToken contract is also
     * deployed at the same time. It'll be used later to deploy
     * new LPTokens using the [clone factory pattern](https://eips.ethereum.org/EIPS/eip-1167).
     */
    constructor(address weth_) {
        weth = weth_;
        lpTokenImplementation = address(new LPToken());
        LPToken(lpTokenImplementation).initialize("", "");
    }

    /// @inheritdoc IDFMM
    function init(InitParams calldata params)
        external
        payable
        lock
        returns (uint256, uint256, uint256, uint256)
    {
        if (params.tokenX == params.tokenY) revert InvalidTokens();

        (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        ) = IStrategy(params.strategy).init(
            msg.sender, pools.length, params.data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        LPToken liquidityToken = LPToken(clone(lpTokenImplementation));

        string memory tokenMetadata =
            _prepareTokenMetadata(params.strategy, params.tokenX, params.tokenY);
        liquidityToken.initialize(tokenMetadata, tokenMetadata);
        liquidityToken.mint(msg.sender, totalLiquidity - BURNT_LIQUIDITY);
        liquidityToken.mint(address(0), BURNT_LIQUIDITY);

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

        _transferFrom(params.tokenX, reserveX);
        _transferFrom(params.tokenY, reserveY);

        emitInit(poolId);

        return (poolId, reserveX, reserveY, totalLiquidity - BURNT_LIQUIDITY);
    }

    function emitInit(uint256 poolId) private {
        Pool memory pool = pools[poolId];

        emit Init(
            msg.sender,
            pool.strategy,
            pool.tokenX,
            pool.tokenY,
            poolId,
            pool.reserveX,
            pool.reserveY,
            pool.totalLiquidity
        );
    }

    function _prepareTokenMetadata(
        address strategy,
        address tokenX,
        address tokenY
    ) internal view returns (string memory) {
        return string.concat(
            "DFMM-",
            IStrategy(strategy).name(),
            "-",
            ERC20(tokenX).symbol(),
            "-",
            ERC20(tokenY).symbol(),
            "-",
            LibString.toString(pools.length)
        );
    }

    /// @inheritdoc IDFMM
    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external payable lock returns (uint256, uint256, uint256) {
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePoolReserves(poolId, true, data);

        _transferFrom(pools[poolId].tokenX, deltaX);
        _transferFrom(pools[poolId].tokenY, deltaY);

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

        _transfer(pools[poolId].tokenX, msg.sender, deltaX);
        _transfer(pools[poolId].tokenY, msg.sender, deltaY);

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
            int256 invariant,
            ,
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedTotalLiquidity
        ) = IStrategy(pools[poolId].strategy).validateSwap(
            msg.sender, poolId, data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
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
            if (adjustedReserveY >= originalReserveY) revert InvalidSwap();
            inputToken = pools[poolId].tokenX;
            outputToken = pools[poolId].tokenY;
            inputAmount = adjustedReserveX - originalReserveX;
            outputAmount = originalReserveY - adjustedReserveY;
        } else {
            if (adjustedReserveX >= originalReserveX) revert InvalidSwap();
            inputToken = pools[poolId].tokenY;
            outputToken = pools[poolId].tokenX;
            inputAmount = adjustedReserveY - originalReserveY;
            outputAmount = originalReserveX - adjustedReserveX;
        }

        // Do the state updates to the reserves before calling untrusted addresses.
        pools[poolId].reserveX = adjustedReserveX;
        pools[poolId].reserveY = adjustedReserveY;

        uint256 preInputBalance = ERC20(inputToken).balanceOf(address(this));
        uint256 preOutputBalance = ERC20(outputToken).balanceOf(address(this));

        _transferFrom(inputToken, inputAmount);
        _transfer(outputToken, msg.sender, outputAmount);

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

    /**
     * @dev Transfers `amount` of `token` from the sender to the contract.
     * Note that if ETH is present in the contract, it will be wrapped to WETH.
     * @param token Address of the token to transfer.
     * @param amount Amount to transfer expressed in WAD.
     */
    function _transferFrom(address token, uint256 amount) internal {
        if (address(this).balance >= amount) {
            WETH(payable(weth)).deposit{ value: amount }();

            if (address(this).balance > 0) {
                SafeTransferLib.safeTransferETH(
                    msg.sender, address(this).balance
                );
            }
        } else {
            uint256 downscaledAmount =
                downscaleUp(amount, computeScalingFactor(token));
            SafeTransferLib.safeTransferFrom(
                ERC20(token), msg.sender, address(this), downscaledAmount
            );
        }
    }

    /**
     * @dev Transfers `amount of `token` from the contract to the recipient
     * `to`. Note that WETH is automatically unwrapped to ETH.
     * @param token Address of the token to transfer.
     * @param to Address of the recipient.
     * @param amount Amount to transfer expressed in WAD.
     */
    function _transfer(address token, address to, uint256 amount) internal {
        if (token == weth) {
            WETH(payable(weth)).withdraw(amount);
            SafeTransferLib.safeTransferETH(to, amount);
        } else {
            uint256 downscaledAmount =
                downscaleDown(amount, computeScalingFactor(token));
            SafeTransferLib.safeTransfer(ERC20(token), to, downscaledAmount);
        }
    }

    /**
     * @dev Validates the adjusted reserves and liquidity and updates the
     * reserves and liquidity of a pool during an allocation or deallocation.
     */
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

    /**
     * @dev Mints or burns liquidity tokens.
     */
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
     * This function uses the create opcode, which should never revert.
     * This function is taken from https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/proxy/Clones.sol#L23.
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

    /// @notice Returns the amount of initialized pools.
    function nonce() external view returns (uint256) {
        return pools.length;
    }

    /// @notice Returns the pool `poolId` as a Pool struct.
    function getPool(uint256 poolId) external view returns (Pool memory) {
        return pools[poolId];
    }

    /// @notice Returns the reserves and liquidity of pool `poolId`.
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
     * @notice Returns the amount of liquidity owned by `account` for
     * the pool `poolId`.
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
