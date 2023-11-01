# Dollar Cost Averaging

The [geometric mean](../trading_functions/geometric_mean.md) can be used to perform a type of *dollar cost averaging (DCA)* strategy.
For this, we need a few parameters.
Let $T$ be the amount of time we want to spend dollar cost averaging.
Then we can define the *proportion of time remaining* as $$ \tau = \frac{t}{T} $$ where $t$ is the amount of time that has passed.
From here, we can define dynamic function market maker (DFMM) curve as: $$ \varphi_t(x,y) = x^{\tau}y^{1-\tau} .$$
In this case, we start with 0/100 weight of $X$ to $Y$.
At expiry, $\tau =1$ and we have a 100/0 weight of $X$ to $Y$.

