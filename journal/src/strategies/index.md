# Strategies

Strategies are components that combine agent behavior with trading functions. 
These can be used to create more complex behavior. 
Concretely, we can think of strategies as the financial implications of how variables are changed in trading functions.
Doing this ensures oracle / or decentralization risk. 
The oracle risk can be reduced if it is updated about internal systemic variables. For example like, [eip1559](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1559.md) fee utilizes control theory where the gas price is a function of how close the target gas for the block was to the actual gas. A simple example of a strategy is dollar cost averaging with a DFMM.

## Dollar Cost Averaging
The [geometric mean](../trading_functions/geometric_mean.md) can be used to perform a type of *dollar cost averaging (DCA)* strategy.
For this, we need a few parameters.
Let $T$ be the amount of time we want to spend dollar cost averaging.
Then we can define the *proportion of time remaining* as $$ \tau = \frac{t}{T} $$ where $t$ is the amount of time that has passed.
From here, we can define dynamic function market maker (DFMM) curve as: $$ \varphi_t(x,y) = x^{\tau}y^{1-\tau} .$$
In this case, we start with 0/100 weight of $X$ to $Y$.
At expiry, $\tau =1$ and we have a 100/0 weight of $X$ to $Y$.
How does this strategy compare to DCAing over more discrete intervals?

## Momentum
TODO

## Volatility Targeting
TODO