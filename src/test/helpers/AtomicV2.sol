// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.14;

import "solstat/Gaussian.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "src/solvers/BisectionLib.sol";
import "forge-std/console2.sol";

interface LiquidExchange {
    function swap(address token, uint256 amount) external;
    function price() external returns (uint256);
}

interface StrategyLike {
    function swap(uint256 poolId, bytes calldata data) external;
    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    )
        external
        view
        returns (
            bool valid,
            uint256 estimatedOut,
            uint256 estimatedPrice,
            bytes memory payload
        );
    function internalPrice(uint256 poolId) external view returns (uint256);
    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256, uint256, uint256);
}

interface TokenLike {
    function transferFrom(address, address, uint256) external;
    function transfer(address, uint256) external;
    function approve(address, uint256) external;
    function balanceOf(address) external view returns (uint256);
    function allowance(address, address) external view returns (uint256);
}

/// @dev Takes Y tokens from Arbitrageur, swaps on either LEX or DEX, then swaps on the opposite exchange and returns the output Y tokens to Arbitrageur.
contract AtomicV2 {
    using FixedPointMathLib for int256;
    using FixedPointMathLib for uint256;

    error InsufficientBalanceY(uint256 balance, uint256 payment);
    error InsufficientBalanceX(uint256 balance, uint256 payment);
    error InsufficientApprovalY(uint256 allowance, uint256 payment);
    error DexSwapFailure(string reason, bytes err);
    error SimulatedSwapFailure(
        bool valid, uint256 estimatedOut, uint256 estimatedPrice, bytes payload
    );

    error UnprofitableArbitrage(
        uint256 start_y_balance,
        uint256 end_y_balance,
        uint256 absolute_difference
    );

    event Profit(uint256 profit);
    event Loss(uint256 loss);
    event Price(uint256 price, uint256 timestamp);

    address public liquidExchange;
    address public exchange;
    address public solver;
    address public asset;
    address public quote;

    /// @dev Since token x is transferred inside the arbitrage loop, this stores that value in the last arb loop.
    uint256 public intermediateTokenXBalance;
    uint256 public intermediateTokenYStartBalance;
    uint256 public intermediateTokenYEndBalance;

    /// @dev Accumulates the profit from each arbitrage loop in Y tokens.
    uint256 public cumulativeProfit;

    constructor(
        address solverAddress,
        address exchangeAddress,
        address liquidExchangeAddress,
        address assetAddress,
        address quoteAddress
    ) {
        solver = solverAddress;
        exchange = exchangeAddress;
        liquidExchange = liquidExchangeAddress;
        asset = assetAddress;
        quote = quoteAddress;
    }

    bool public XTOY = true;
    bool public YTOX = false;

    error AttemptedProfit(int256 profit);

    function lower_exchange_price(uint256 poolId, uint256 input) external {
        uint256 price = StrategyLike(solver).internalPrice(poolId);
        emit Price(price, block.timestamp);
        // Arbitrageur Y -> AtomicV2
        _invoice(input);

        // Y -> X on LEX
        _lex_swap(YTOX, input);

        // X -> Y on DEX
        _dex_swap(poolId, XTOY, TokenLike(asset).balanceOf(address(this)));

        // AtomicV2 Y -> Arbitrageur
        _payout();
    }

    function raise_exchange_price(uint256 poolId, uint256 input) external {
        uint256 price = StrategyLike(solver).internalPrice(poolId);
        emit Price(price, block.timestamp);
        // Arbitrageur Y -> AtomicV2
        _invoice(input);

        // Y -> X on DEX
        _dex_swap(poolId, YTOX, input);

        // X -> Y on LEX
        _lex_swap(XTOY, TokenLike(asset).balanceOf(address(this)));

        // AtomicV2 Y -> Arbitrageur
        _payout();
    }

    /// @dev Handles the payment from the arbitrageur.
    function _invoice(uint256 amount_y) internal {
        uint256 quote_balance = TokenLike(quote).balanceOf(msg.sender);
        if (quote_balance < amount_y) {
            revert InsufficientBalanceY(quote_balance, amount_y);
        }

        uint256 quote_approval =
            TokenLike(quote).allowance(msg.sender, address(this));
        if (quote_approval < amount_y) {
            revert InsufficientApprovalY(quote_approval, amount_y);
        }

        TokenLike(quote).transferFrom(msg.sender, address(this), amount_y);

        intermediateTokenYStartBalance =
            TokenLike(quote).balanceOf(address(this));
    }

    function _lex_swap(bool swapXIn, uint256 input) internal {
        if (swapXIn) {
            // If X -> Y
            // Approve LiquidExchange to spend AtomicV2's asset tokens
            TokenLike(asset).approve(liquidExchange, input);

            // Exchange asset for quote on LiquidExchange
            LiquidExchange(liquidExchange).swap(asset, input);

            // Don't store the intermediate Y balance!
        } else {
            // If Y -> X
            // Approve LiquidExchange to spend AtomicV2's quote tokens
            TokenLike(quote).approve(liquidExchange, input);

            // Exchange quote for asset on LiquidExchange
            LiquidExchange(liquidExchange).swap(quote, input);

            // Store the intermediate X balance.
            intermediateTokenXBalance =
                TokenLike(asset).balanceOf(address(this));
        }
    }

    function _dex_swap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) internal {
        if (swapXIn) {
            // If X -> Y
            // Approve Exchange to spend AtomicV2's asset tokens
            TokenLike(asset).approve(exchange, amountIn);
        } else {
            // If Y -> X
            // Approve Exchange to spend AtomicV2's quote tokens
            TokenLike(quote).approve(exchange, amountIn);
        }

        // Simulating the swap will get the swap payload data to send.
        // Warning! This is very vulnerable. Do not use in production.
        (
            bool valid,
            uint256 estimatedOut,
            uint256 estimatedPrice,
            bytes memory payload
        ) = StrategyLike(solver).simulateSwap(poolId, swapXIn, amountIn);

        if (!valid) {
            revert SimulatedSwapFailure(
                valid, estimatedOut, estimatedPrice, payload
            );
        }

        // Execute the swap.
        try StrategyLike(exchange).swap(poolId, payload) {
            // Swap succeeded, do nothing other than store the intermediary balance.
            if (swapXIn) {
                // If X -> Y
                // Don't store the intermediate Y balance!
            } else {
                // If Y -> X
                // Get the X balance after the swap.
                intermediateTokenXBalance =
                    TokenLike(asset).balanceOf(address(this));
            }
        } catch (bytes memory err) {
            revert DexSwapFailure("DEX swap failed with data", err);
        }
    }

    function _payout() internal {
        // Get the Y balance after the swap.
        intermediateTokenYEndBalance = TokenLike(quote).balanceOf(address(this));

        if (intermediateTokenYEndBalance < intermediateTokenYStartBalance) {
            emit Loss(
                intermediateTokenYStartBalance - intermediateTokenYEndBalance
            );
            revert UnprofitableArbitrage(
                intermediateTokenYStartBalance,
                intermediateTokenYEndBalance,
                uint256(
                    ~(
                        int256(intermediateTokenYStartBalance)
                            - int256(intermediateTokenYEndBalance)
                    )
                )
            );
        }

        // Accumulate the profit and send the balance!
        uint256 profit =
            intermediateTokenYEndBalance - intermediateTokenYStartBalance;
        cumulativeProfit += profit;
        emit Profit(profit);

        TokenLike(quote).transfer(msg.sender, intermediateTokenYEndBalance);
    }

    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    )
        public
        view
        returns (
            bool valid,
            uint256 estimatedOut,
            uint256 estimatedPrice,
            bytes memory payload
        )
    {
        return StrategyLike(solver).simulateSwap(poolId, swapXIn, amountIn);
    }

    function cdf(int256 input) public pure returns (int256 output) {
        output = Gaussian.cdf(input);
    }

    function pdf(int256 input) public pure returns (int256 output) {
        output = Gaussian.pdf(input);
    }

    function ppf(int256 input) public pure returns (int256 output) {
        output = Gaussian.ppf(input);
    }

    function mulWadDown(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.mulWadDown(x, y);
    }

    function mulWadUp(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.mulWadUp(x, y);
    }

    function divWadDown(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.divWadDown(x, y);
    }

    function divWadUp(uint256 x, uint256 y) public pure returns (uint256 z) {
        z = FixedPointMathLib.divWadUp(x, y);
    }

    function log(int256 x) public pure returns (int256 z) {
        z = FixedPointMathLib.lnWad(x);
    }

    function sqrt(uint256 x) public pure returns (uint256 z) {
        z = FixedPointMathLib.sqrt(x);
    }
}
