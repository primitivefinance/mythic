pragma solidity ^0.8.14;

interface ExchangeLike {
    function swap(address token, uint256 amount) external;
}

interface StrategyLike {
    function swapAmountIn(bool swapDirection, uint256 amount) external returns (uint256);
}

interface TokenLike {
    function transferFrom(address, address, uint256) external;
    function transfer(address, uint256) external;
    function approve(address, uint256) external;
    function balanceOf(address) external view returns (uint256);
}

contract AtomicArbitrage {
    error NotProfitable(uint256 first_swap_output, uint256 second_swap_output);

    address public exchange;
    address public liquidExchange;
    address public asset;
    address public quote;

    constructor(address exchangeAddress, address liquidExchangeAddress, address assetAddress, address quoteAddress) {
        exchange = exchangeAddress;
        liquidExchange = liquidExchangeAddress;
        asset = assetAddress;
        quote = quoteAddress;
    }


    function lower_exchange_price(
        uint256 input
    ) external {
        // pull in tokens from arbitrageur
        TokenLike(quote).transferFrom(msg.sender, address(this), input);

        // exchange quote for asset on LiquidExchange
        TokenLike(quote).approve(liquidExchange, input);
        ExchangeLike(liquidExchange).swap(quote, input);

        // do swap on Exchange
        uint256 asset_balance = TokenLike(asset).balanceOf(address(this));
        TokenLike(asset).approve(exchange, asset_balance);
        StrategyLike(exchange).swapAmountIn(true, asset_balance);

        // send quote tokens to arbitrageur if we have tokens and the trade is profitable
        uint256 quote_balance = TokenLike(quote).balanceOf(address(this));
        if (quote_balance > 0) {
            TokenLike(quote).transfer(msg.sender, quote_balance);
            }
        else {
            revert NotProfitable(asset_balance, quote_balance);
        }
        // require(quote_balance > input, "Not profitable");
    }
    event TestEvent0(uint256 t1);
    event TestEvent1(uint256 t1);
    event TestEvent2(uint256 t1);
    event TestEvent3(uint256 t1);
    event TestEvent4(uint256 t1);
    function raise_exchange_price(
        uint256 input
    ) external {
        // pull in tokens from arbitrageur
        emit TestEvent0(input);
        TokenLike(quote).transferFrom(msg.sender, address(this), input);
        emit TestEvent0(input);

        // exchange quote for asset on Exchange
        TokenLike(quote).approve(exchange, input);
        emit TestEvent1(input);
        StrategyLike(exchange).swapAmountIn(false, input);
        emit TestEvent2(input);

        // do swap on LiquidExchange
        uint256 asset_balance = TokenLike(asset).balanceOf(address(this));
        emit TestEvent3(asset_balance);
        TokenLike(asset).approve(liquidExchange, asset_balance);
        emit TestEvent4(asset_balance);
        ExchangeLike(liquidExchange).swap(asset, asset_balance);

        // send quote tokens to arbitrageur if we have tokens and the trade is profitable
        uint256 quote_balance = TokenLike(quote).balanceOf(address(this));
        if (quote_balance > 0) {
            TokenLike(quote).transfer(msg.sender, quote_balance);
            }
        else {
            revert NotProfitable(asset_balance, quote_balance);
        }
    }

}
