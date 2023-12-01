pragma solidity ^0.8.14;

interface ExchangeLike {
    function swap(address token, uint256 amount) external;
}

interface StrategyLike {
    function swapAmountIn(
        bool swapDirection,
        uint256 nextLiquidity,
        uint256 amount
    ) external returns (uint256);
    function swap(
        bool swapDirection,
        uint256 nextLiquidity,
        uint256 amountIn,
        uint256 amountOut
    ) external returns (uint256);
}

interface TokenLike {
    function transferFrom(address, address, uint256) external;
    function transfer(address, uint256) external;
    function approve(address, uint256) external;
    function balanceOf(address) external view returns (uint256);
}

contract RMMAtomicArbitrage {
    error NotProfitable(
        uint256 input,
        uint256 output,
        uint256 difference
    );

    address public exchange;
    address public liquidExchange;
    address public asset;
    address public quote;

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

    function lower_exchange_price(
        uint256 input,
        uint256 output,
        uint256 nextLiquidity
    ) external {
        // pull in tokens from arbitrageur
        TokenLike(quote).transferFrom(msg.sender, address(this), input);

        // exchange quote for asset on LiquidExchange
        TokenLike(quote).approve(liquidExchange, input);
        ExchangeLike(liquidExchange).swap(quote, input);

        // do swap on Exchange
        uint256 asset_balance = TokenLike(asset).balanceOf(address(this));
        TokenLike(asset).approve(exchange, asset_balance);
        StrategyLike(exchange).swap(true, nextLiquidity, asset_balance, output);

        // send quote tokens to arbitrageur if we have tokens and the trade is profitable
        uint256 postQuoteBalance = TokenLike(quote).balanceOf(address(this));
        if (postQuoteBalance > input) {
            // revert Debug(quote_balance);
            TokenLike(quote).transfer(msg.sender, postQuoteBalance);
        } else {
            revert NotProfitable(input, postQuoteBalance, input - postQuoteBalance);
        }
    }

    function raise_exchange_price(
        uint256 input,
        uint256 output,
        uint256 nextLiquidity
    ) external {
        // pull in tokens from arbitrageur
        TokenLike(quote).transferFrom(msg.sender, address(this), input);

        // exchange quote for asset on Exchange
        TokenLike(quote).approve(exchange, input);
        StrategyLike(exchange).swap(false, nextLiquidity, input, output);

        // do swap on LiquidExchange
        uint256 asset_balance = TokenLike(asset).balanceOf(address(this));
        TokenLike(asset).approve(liquidExchange, asset_balance);
        ExchangeLike(liquidExchange).swap(asset, asset_balance);

        // send quote tokens to arbitrageur if we have tokens and the trade is profitable
        uint256 postQuoteBalance = TokenLike(quote).balanceOf(address(this));
        if (postQuoteBalance > input) {
            TokenLike(quote).transfer(msg.sender, postQuoteBalance);
        } else {
            revert NotProfitable(input, postQuoteBalance, input - postQuoteBalance);
        }
    }
}
