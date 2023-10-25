# Volatility Targeting
The G3M volatility targeting strategy should have its own analysis harness dedicated to it.
There are specific properties of this strategy that we want to test and analyze that extend beyond that scoped in the [general performance metrics](./index.md#general-performance-metrics).

## Notes
We should stick with a consistent choice of units for all of these simulations.
Consider using a timeframe of a year.

## Hypothesis Testing
The following are hypotheses that we can test with the volatility targeting strategy.
They should be done independently, but we will need some overlap between them.

### Adherence to Target Volatility
We should go through a slew of different approaches here, each more complicated and more realistic than the last.
If we find bad results early on, then we can stop and not waste time on the more complicated approaches.

#### Constant Volatility
Aside from the initialization stage, the strategy should seek to maintain a constant volatility.
To test this adherence, we should apply a range of GBM price paths with variable drifts and volatilities and collect data on how the strategy performs.
We can generate statistics on the volatility of the strategy and compare it to the target volatility baseline.

**Expectations:**
- A mean strategy volatility that is "close" to the target volatility
- A standard deviation of the strategy volatility that is "close" to zero
- Low to no dependence on the drift parameter

**Testing:**
- Generate a GBM with sweeps:
    - $\mu \in \{-1.0, -0.9, ..., 0.9, 1.0\}$
    - $\sigma \in \{0.05, 0.1, ..., 0.95, 1.0\}$
    - Run for 50 random paths for each parameter.
    - Run each path for 100,000 steps over the course of 5 years.

#### Variable Volatility
The strategy should be able to handle variable volatility and track accordingly.
To test this, we can use the same GBM price paths as above, but we can change the volatility at a given time step.
We can then see how the strategy reacts to the change in volatility.

**Expectations:**
- A mean strategy volatility that is "close" to the target volatility.
- A standard deviation of the strategy volatility that is "close" to zero.
    - We should compute distance from the target volatility as a function of time. 
    This result should be highly correlated to the time-varying volatility chosen for the process.

**Testing:**
- Generate a GBM with sweeps:
    - $\mu \in \{-1.0, -0.9, ..., 0.9, 1.0\}$
    - $\sigma(t) \in \{0.5\sin(t)+0.5, 0.5\sin(1.25t)+0.5, ..., 0.5\sin(3.0t)+0.5\}$. 
    By choosing sinusoidal volatility, we are allowing for a display of a smooth periodic change in volatility which is a very coarse model for seasonal volatility. 
    We can choose here to amplify the frequency of the volatility change to see how the strategy reacts to a more rapid change in volatility.
    - Run for 50 random paths for each parameter.
    - Run each path for 100,000 steps over the course of 5 years.

### Performance Metrics

#### Reweighting Speed
We have options for how we can choose to reweight the pool.
We can choose to do so very quickly or very slowly -- each has its own tradeoffs.

**Expectations:**
- Quicker reweighting leads to improved volatility tracking.
- Quicker reweighting leads to more loss due to arbitrage.
- Slower reweighting leads to worse volatility tracking.
- Slower reweighting leads to less loss due to arbitrage.

**Testing:**
- Generate a GBM with sweeps:
    - $\mu \in \{-1.0, -0.9, ..., 0.9, 1.0\}$
    - $\sigma(t) \in \{0.5\sin(t)+0.5, 0.5\sin(1.25t)+0.5, ..., 0.5\sin(3.0t)+0.5\}$.
    - Run for 50 random paths for each parameter.
    - Run each path for 100,000 steps over the course of 5 years.
- Choose reweight time from $t \in \{0.0005, 0.001, 0.0015, ..., 0.2\}$