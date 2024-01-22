# Dollar Cost Averaging

To analyze the DCA strategy we should consider the following approach.

1. Define a weight changer agent that changes the weight of the DFMM curve over time as prescribed in [Dollar Cost Averaging](dollar_cost_averaging.md).
2. Define a swapper agent to swap between $X$ and $Y$ at prescribed timeframes.

We can compare the values of the portfolio held by an LP into the DCA strategy versus the swapper agent.

## Static Testing
Let's first test the DCA strategy with a single set of reasonable parameters.
We can compare the DCA strategy in a GBM style market with a drift of $0.1$ and volatility of $0.5$ running over the course of $1.0$ years.
Over the same time frame, we can let a swapper agent swap 12 times representing a monthly DCA purchase.

We can use the following structure for a toml:
```toml
[trajectory]
process = "gbm"
num_steps = 36500
seed = 1
num_paths = 50

[trajectory.initial_price]
fixed = 1.0
[trajectory.t_0]
fixed = 0.0
[trajectory.t_n]
fixed = 1.0

[gbm]
[gbm.drift]
fixed = 0.1
[gbm.volatility]
fixed = 0.25

[pool]
weight_x = 0.01
fee_basis_points = 30

[lp]
x_liquidity = 1.0

[weight_changer]
time_to_expiry = 1.0

[swapper]
num_swaps = 12

[block]
timestep_size = 15
```

### Performance
Performance over the 50 paths will be used to see the initial efficacy of this system.
We can plot the portfolio values of both the LP and the swapper and see how they track over time.