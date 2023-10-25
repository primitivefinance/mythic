# Analysis
To validate our strategies, we need to be able to simulate them. 
This is done by creating a simulation environment that can be used to run the strategies and log the results.
Once they are simulated, we can analyze the results to see how well the strategies performed.
Simulation and analysis work together in tandem via the scientific method. 
We can use the results of the analysis to inform our next simulation, and so on.
We can set out goals for performance, and then use the analysis to see if we are meeting those goals.
These goals will also help us design analysis suites for individual strategies as well as give us ideas on what we consider to be general performance metrics.

## General Performance Metrics
These are metrics that should be able to be tested across any strategy.

### Raw contract-level performance metrics:
- Gas costs for swaps, LP, maintenance.


### Financial performance metrics:
- Fee generation (with arbitrage flow and with retail flow)
- Liquidity depth + gas costs -> fee generation
- Sharpe ratio
- Volatility compared to external market
- Max drawdown compared to external market
- High water mark compared to external market

### Stability/Robustness performance metrics:
- How does the financial performance change given incremental (from small to large) changes in parameter values. 
Do we ever see a critical point where a small change in choice of parameter leads to a large change in performance?
If so, this is a sign of instability.
- How does the performance handle shocks to the system?
- Does the performance look smooth and "convex" around the optimal parameters? Or is it jagged and has multiple local optima?
- What is the worst case scenario for the performance? How likely is it to occur?
- What is the best case scenario for the performance? How likely is it to occur?

### Security performance metrics:
- Can the contract enter an unintended state that we consider anomalous? 
- Are there specific rational agents that can trigger a bad state?
- Are there irrational agents that can trigger a bad state?
- Are there boundary cases in the contract?
- How does the contract fair after an immense amount of time and volume? (e.g., 1,000,000,000 swaps)

<!-- ADD ABOUT WHAT DIFFERENT PRICE PROCESSES CAN DO and tell us -->