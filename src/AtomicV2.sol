pragma solidity ^0.8.14;

import "forge-std/console2.sol";

interface LiquidExchange {
    function swap(address token, uint256 amount) external;
}

interface StrategyLike {
    function swap(bytes calldata data) external;
    function simulateSwap(
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
    function internalPrice() external view returns (uint256);
    function source() external view returns (address);
    function getReservesAndLiquidity()
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
    function swapFeePercentageWad() external view returns (uint256);
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

    error UnprofitableArbitrage(
        uint256 start_y_balance,
        uint256 end_y_balance,
        uint256 absolute_difference
    );

    event Profit(uint256 profit);
    event Loss(uint256 loss);

    address public exchange;
    address public liquidExchange;
    address public asset;
    address public quote;

    /// @dev Since token x is transferred inside the arbitrage loop, this stores that value in the last arb loop.
    uint256 public intermediateTokenXBalance;
    uint256 public intermediateTokenYStartBalance;
    uint256 public intermediateTokenYEndBalance;

    /// @dev Accumulates the profit from each arbitrage loop in Y tokens.
    uint256 public cumulativeProfit;

    constructor(
        address exchangeAddress,
        address liquidExchangeAddress,
        address assetAddress,
        address quoteAddress
    ) {
        exchange = exchangeAddress;
        liquidExchange = liquidExchangeAddress;
        asset = assetAddress;
        quote = quoteAddress;
    }

    bool public XTOY = true;
    bool public YTOX = false;

    function findTrade(
        bool swapXIn,
        uint256 priceTarget,
        uint256 lower,
        uint256 upper,
        uint256 epsilon,
        uint256 max_iters
    ) public view returns (uint256 amountIn) {
        require(lower < upper, "lower must be less than upper");

        uint256 lowerPrice;
        try StrategyLike(exchange).simulateSwap(swapXIn, lower) returns (
            bool, uint256, uint256 price, bytes memory
        ) {
            lowerPrice = price;
        } catch (bytes memory err) {
            // If you get here it means the lower bound input caused the simulate swap to revert. Usually we can just pass through, but this might return a price of 0 when a small trade would actually return a high price.
            console2.log("lowerPrice error");
            revert FindingTradeError(err);
        }

        uint256 upperPrice;
        try StrategyLike(exchange).simulateSwap(swapXIn, upper) returns (
            bool, uint256, uint256 price, bytes memory
        ) {
            upperPrice = price;
        } catch (bytes memory err) {
            console2.log("upperPrice error");
            revert FindingTradeError(err);
        }

        console2.log("lowerPrice", lowerPrice);
        console2.log("upperPrice", upperPrice);
        console2.log("targtPrice", priceTarget);

        int256 lowerResult = int256(lowerPrice) - int256(priceTarget);
        int256 upperResult = int256(upperPrice) - int256(priceTarget);
        console2.logInt(lowerResult);
        console2.logInt(upperResult);

        require(lowerResult * upperResult < 0, "root must be between bounds");

        uint256 distance = upper - lower;

        uint256 iterations;
        do {
            amountIn = (lower + upper) / 2;

            uint256 price;
            try StrategyLike(exchange).simulateSwap(swapXIn, amountIn) returns (
                bool, uint256, uint256 _price, bytes memory
            ) {
                price = _price;
            } catch (bytes memory err) { }

            console2.log("amountIn", amountIn);
            console2.log("target price", priceTarget);
            console2.log("result price", price);
            int256 result = int256(price) - int256(priceTarget);
            console2.logInt(result);

            if (result * lowerResult <= 0) {
                upper = amountIn;
            } else {
                lower = amountIn;
                lowerResult = result;
            }

            distance = upper - lower;
            iterations += 1;
        } while (distance > epsilon && iterations < max_iters);
    }

    function try_arbitrage_until_target_price(
        uint256 target_price,
        uint256 max_iterations
    ) public view returns (uint256 amountIn) {
        uint256 start_price = StrategyLike(exchange).internalPrice();
        (uint256 strikePriceWad,,) =
            StrategySource(StrategyLike(exchange).source()).dynamicSlot();
        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            StrategyLike(exchange).getReservesAndLiquidity();

        if (start_price < target_price) {
            // If the start price is less than the target price, we need to raise the price.
            // Therefore, swap Y -> X on DEX until the price is greater than the target price.
            uint256 minAmountIn = 10;
            uint256 maxAmountIn =
                strikePriceWad - reserveY * 1 ether / liquidity;

            maxAmountIn -= 10;

            console2.log("minAmountIn", minAmountIn);
            console2.log("maxAmountIn", maxAmountIn);

            amountIn =
                findTrade(YTOX, target_price, minAmountIn, maxAmountIn, 5, 3);
        } else {
            // If the start price is greater than the target price, we need to lower the price.
            // Therefore, swap X -> Y on DEX until the price is less than the target price.
            uint256 minAmountIn = 10;
            console2.log(
                "Computed max in", 1 ether - reserveX * 1 ether / liquidity
            );
            uint256 maxAmountIn = (liquidity - reserveX); //1 ether - reserveX * 1 ether / liquidity;
            maxAmountIn -= 10;

            console2.log("minAmountIn", minAmountIn);
            console2.log("maxAmountIn", maxAmountIn);

            amountIn =
                findTrade(XTOY, target_price, minAmountIn, maxAmountIn, 5, 3);
        }
    }

    function lower_exchange_price(uint256 input) external {
        // Arbitrageur Y -> AtomicV2
        _invoice(input);

        // Y -> X on LEX
        _lex_swap(YTOX, input);

        // X -> Y on DEX
        _dex_swap(XTOY, TokenLike(asset).balanceOf(address(this)));

        // AtomicV2 Y -> Arbitrageur
        _payout();
    }

    function raise_exchange_price(uint256 input) external {
        // Arbitrageur Y -> AtomicV2
        _invoice(input);

        // Y -> X on DEX
        _dex_swap(YTOX, input);

        // X -> Y on LEX
        _lex_swap(XTOY, TokenLike(asset).balanceOf(address(this)));

        // AtomicV2 Y -> Arbitrageur
        _payout();
    }

    /// @dev Handles the payment from the arbitrageur.
    function _invoice(uint256 amount) internal {
        uint256 quote_balance = TokenLike(quote).balanceOf(msg.sender);
        if (quote_balance < amount) {
            revert InsufficientBalanceY(quote_balance, amount);
        }

        uint256 quote_approval =
            TokenLike(quote).allowance(msg.sender, address(this));
        if (quote_approval < amount) {
            revert InsufficientApprovalY(quote_approval, amount);
        }

        TokenLike(quote).transferFrom(msg.sender, address(this), amount);

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

    function _dex_swap(bool swapXIn, uint256 amountIn) internal {
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
        ) = StrategyLike(exchange).simulateSwap(swapXIn, amountIn);

        if (!valid) {
            revert SimulatedSwapFailure(
                valid, estimatedOut, estimatedPrice, payload
            );
        }

        // Execute the swap.
        try StrategyLike(exchange).swap(payload) {
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
}
