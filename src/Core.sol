/// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "solmate/utils/SafeTransferLib.sol";
import "./interfaces/IStrategy.sol";

interface NewCore {
    error AlreadyInitialized();
    error NotInitialized();
    error Locked();

    error Invalid(bool negative, int256 swapConstantGrowth);

    event Init(address account, uint256 x, uint256 y, uint256 L);

    event Allocate(
        address indexed account, uint256 deltaX, uint256 deltaY, uint256 deltaL
    );

    event Deallocate(
        address indexed account, uint256 deltaX, uint256 deltaY, uint256 deltaL
    );
}

contract Core is NewCore {
    using FixedPointMathLib for uint256;

    address public immutable strategy;
    address public immutable tokenX;
    address public immutable tokenY;

    bool public inited;

    uint256 public reserveXWad;
    uint256 public reserveYWad;
    uint256 public totalLiquidity;
    uint256 public feeGrowth = 1 ether;
    uint256 public swapFeePercentageWad;

    mapping(address account => uint256 balance) public balanceOf;
    mapping(address account => uint256 lastFeeGrowth) public lastFeeGrowthOf;

    uint256 internal locked = 1;

    modifier initialized() {
        if (!inited) revert NotInitialized();
        _;
    }

    modifier lock() {
        if (locked == 2) revert Locked();
        locked = 2;
        _;
        locked = 1;
    }

    constructor(
        address strategy_,
        address tokenX_,
        address tokenY_,
        uint256 swapFeePercentageWad_
    ) {
        strategy = strategy_;
        tokenX = tokenX_;
        tokenY = tokenY_;
        swapFeePercentageWad = swapFeePercentageWad_;
    }

    function init(bytes calldata data)
        public
        lock
        returns (uint256, uint256, uint256)
    {
        if (inited) revert AlreadyInitialized();

        (bool valid, int256 swapConstantGrowth, uint256 x, uint256 y, uint256 L)
        = IStrategy(strategy).init(data);

        if (!valid) {
            revert Invalid(swapConstantGrowth < 0, swapConstantGrowth);
        }

        inited = true;
        reserveXWad = x;
        reserveYWad = y;
        totalLiquidity = L;

        balanceOf[msg.sender] = L;
        lastFeeGrowthOf[msg.sender] = feeGrowth;

        SafeTransferLib.safeTransferFrom(
            ERC20(tokenX), msg.sender, address(this), x
        );

        SafeTransferLib.safeTransferFrom(
            ERC20(tokenY), msg.sender, address(this), y
        );

        emit Init(msg.sender, x, y, L);

        return (x, y, L);
    }

    function allocate(bytes calldata data)
        public
        lock
        initialized
        returns (uint256, uint256, uint256)
    {
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePool(true, data);

        _updateBalance();

        balanceOf[msg.sender] += deltaL;
        lastFeeGrowthOf[msg.sender] = feeGrowth;

        SafeTransferLib.safeTransferFrom(
            ERC20(tokenX), msg.sender, address(this), deltaX
        );

        SafeTransferLib.safeTransferFrom(
            ERC20(tokenY), msg.sender, address(this), deltaY
        );

        emit Allocate(msg.sender, deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    function deallocate(bytes calldata data)
        public
        lock
        initialized
        returns (uint256, uint256, uint256)
    {
        (uint256 deltaX, uint256 deltaY, uint256 deltaL) =
            _updatePool(false, data);

        _updateBalance();

        balanceOf[msg.sender] -= deltaL;
        lastFeeGrowthOf[msg.sender] = feeGrowth;

        SafeTransferLib.safeTransfer(ERC20(tokenX), msg.sender, deltaX);

        SafeTransferLib.safeTransfer(ERC20(tokenY), msg.sender, deltaY);

        emit Deallocate(msg.sender, deltaX, deltaY, deltaL);
        return (deltaX, deltaY, deltaL);
    }

    function _updatePool(
        bool isAllocate,
        bytes calldata data
    ) private returns (uint256, uint256, uint256) {
        (bool valid, int256 invariant, uint256 rx, uint256 ry, uint256 L) =
            IStrategy(strategy).validateAllocateOrDeallocate(data);

        if (!valid) revert Invalid(invariant < 0, invariant);

        uint256 deltaX = isAllocate ? rx - reserveXWad : reserveXWad - rx;
        uint256 deltaY = isAllocate ? ry - reserveYWad : reserveYWad - ry;
        uint256 deltaL = isAllocate ? L - totalLiquidity : totalLiquidity - L;

        reserveXWad = rx;
        reserveYWad = ry;
        totalLiquidity = L;

        return (deltaX, deltaY, deltaL);
    }

    function _updateBalance() private {
        if (
            balanceOf[msg.sender] > 0
                && feeGrowth != lastFeeGrowthOf[msg.sender]
        ) {
            uint256 growth = feeGrowth.mulWadDown(lastFeeGrowthOf[msg.sender]);
            balanceOf[msg.sender] = balanceOf[msg.sender].mulWadDown(growth);
        }
    }
}
