// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.14;

import "solstat/Gaussian.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "src/solvers/BisectionLib.sol";

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

interface StrategySource {
    function staticSlot()
        external
        view
        returns (
            uint256 strikePriceWad,
            uint256 sigmaPercentWad,
            uint256 tauYearsWad
        );
    function dynamicSlot()
        external
        view
        returns (
            uint256 strikePriceWad,
            uint256 sigmaPercentWad,
            uint256 tauYearsWad
        );
    function swapFee() external view returns (uint256);
}

interface TokenLike {
    function transferFrom(address, address, uint256) external;
    function transfer(address, uint256) external;
    function approve(address, uint256) external;
    function balanceOf(address) external view returns (uint256);
    function allowance(address, address) external view returns (uint256);
}

interface MockTokenLike {
    function mint(address, uint256) external;
}

contract ProfitFinder {
    AtomicV2 public atomic;

    using FixedPointMathLib for uint256;
    using FixedPointMathLib for uint128;
    using FixedPointMathLib for int256;

    constructor() {
        atomic = AtomicV2(msg.sender);
    }
}

/// @dev Takes Y tokens from Arbitrageur, swaps on either LEX or DEX, then swaps on the opposite exchange and returns the output Y tokens to Arbitrageur.
contract AtomicV2 {
    using FixedPointMathLib for int256;
    using FixedPointMathLib for uint256;

    error InsufficientBalanceY(uint256 balance, uint256 payment);
    error InsufficientBalanceX(uint256 balance, uint256 payment);
    error InsufficientApprovalY(uint256 allowance, uint256 payment);
    error CatastrophicSwapFailure();
    error DexSwapFailure(string reason, bytes err);
    error SimulatedSwapFailure(
        bool valid, uint256 estimatedOut, uint256 estimatedPrice, bytes payload
    );

    error NotProfitable(uint256 first_swap_output, uint256 second_swap_output);

    error FindingTradeError(bytes err);
    error MaximizingProfitTrade(uint256 trade, uint256 profit);

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

    ProfitFinder public profitFinder;

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
        profitFinder = new ProfitFinder();
    }

    bool public XTOY = true;
    bool public YTOX = false;

    struct TradePacket {
        uint256 block_number;
        uint256 block_timestamp;
        bool raisePrice;
        uint256 amountIn;
        uint256 profit;
    }

    TradePacket public tradePacket;

    error AttemptedProfit(int256 profit);

    function searchMaxArbProfit(
        uint256 poolId,
        uint256 best_guess,
        bool xForY
    )
        external
        returns (
            uint256 best_amount_in,
            int256 best_profit,
            uint256 expectedPrice
        )
    {
        uint256 mid;
        int256 derivative;
        uint256 iteration = 0;
        uint256 start = best_guess.mulDivDown(30, 1000);
        uint256 end = best_guess.mulDivUp(1000, 100);

        while (start < end && iteration < 64) {
            mid = start + (end - start) / 2;
            derivative = derivativeOfProfit(poolId, xForY, mid);

            // Check the sign of the derivative to determine the direction of the bisection
            if (derivative > 0) {
                // If the derivative is positive, the maximum profit is to the right
                start = mid;
            } else {
                // If the derivative is negative, the maximum profit is to the left
                end = mid;
            }

            iteration++;
        }

        best_amount_in = (start + end) / 2;
        (best_profit, expectedPrice) =
            calculateProfit(poolId, xForY, best_amount_in);
    }

    function derivativeOfProfit(
        uint256 poolId,
        bool xForY,
        uint256 guess
    ) public returns (int256) {
        uint256 tradeDecrement = guess - 10_000;
        uint256 tradeIncrement = guess + 10_000;
        console2.log("guess", guess);
        (int256 profitUp,) = calculateProfit(poolId, xForY, tradeIncrement);
        (int256 profitDown,) = calculateProfit(poolId, xForY, tradeDecrement);
        console2.log("profitUp", profitUp);
        console2.log("profitDown", profitDown);
        console2.log("difference", profitUp - profitDown);

        int256 derivative = (
            profitUp * int256(1e18) - profitDown * int256(1e18)
        ) / int256(20_000);

        return derivative;
    }

    function calculateProfit(
        uint256 poolId,
        bool xForY,
        uint256 amountIn
    ) public returns (int256, uint256) {
        uint256 price = LiquidExchange(liquidExchange).price();
        (bool valid, uint256 estimatedOut, uint256 estimatedPrice,) =
            StrategyLike(solver).simulateSwap(poolId, xForY, amountIn);
        require(valid, "Invalid swap simulation");

        uint256 valueIn = xForY ? amountIn.mulWadUp(price) : amountIn;
        uint256 valueOut = xForY ? estimatedOut : estimatedOut.mulWadUp(price);

        return (int256(valueOut) - int256(valueIn), estimatedPrice);
    }

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

    // Always reverts with the profit at the end.
    function try_lower_exchange_price(
        uint256 poolId,
        uint256 input_y_amount
    ) external {
        // Arbitrageur Y -> AtomicV2
        _invoice(input_y_amount);

        // Y -> X on LEX
        _lex_swap(YTOX, input_y_amount);

        // X -> Y on DEX
        _dex_swap(poolId, XTOY, TokenLike(asset).balanceOf(address(this)));

        intermediateTokenYEndBalance = TokenLike(quote).balanceOf(address(this));
        revert AttemptedProfit(
            int256(intermediateTokenYEndBalance)
                - int256(intermediateTokenYStartBalance)
        );
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

    function try_raise_exchange_price(
        uint256 poolId,
        uint256 input_y_amount
    ) external {
        // Arbitrageur Y -> AtomicV2
        _invoice(input_y_amount);

        // Y -> X on DEX
        _dex_swap(poolId, YTOX, input_y_amount);

        // X -> Y on LEX
        _lex_swap(XTOY, TokenLike(asset).balanceOf(address(this)));

        intermediateTokenYEndBalance = TokenLike(quote).balanceOf(address(this));
        revert AttemptedProfit(
            int256(intermediateTokenYEndBalance)
                - int256(intermediateTokenYStartBalance)
        );
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
