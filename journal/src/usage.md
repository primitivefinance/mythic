# Usage

## General Strategy Interfacing

Arbitrageurs are agents that need to get specific pricing info from the arbitrage target. While the mechanics of the arbitrage might be different from each other, and require different variables, there is a universal set of actions and information they require.

Arbitrageur needs:
- To get the current price of a pool that is the arbitrage target
    - Depends on smart contract call to pool
- To get the reference market to fill the arbitrage with (target price)
    - Depends on price path / liquid exchange smart contract call
- Get the amount out of tokens given an amount in
    - Depends on smart contract call to pool
- Get the swap fee charged by the pool, since it affects the arbitrageur's incentive
    - Depends on smart contract call to pool
- Compute the amount to swap and expect out, given the above information
    - Depends on arbitrageur's implementation, whether root finding or algebra
- Execute the swap
    - Depends on smart contract call to arbitrageur's smart contract

Arbitrageur specifically needs to compute exactly the amount of tokens to swap to reach a target price that is within its desired arb bounds.


### Sim flow:
1. Create environment
2. Create each agent as their own structs with implementations, which deploy their respective contracts.
3. Initialize/add liquidity to the target pool
4. Update the block
5. Push the initial prices to the weight changer entity via weight_agent.init()
6. Start the event logger
7. Enter the loop over the price path
8. Update the price on the price path
9. Take a step on the arbitrageur loop
10. Update the block
11. Take a step on the weight changer agent

### Agents

All agents that interact with the sim have a `step` that they take which runs their own logic. They also have a `startup` method which should be called outside of the main simulation loop to initialize their state.

These methods can be a trait that is implemented by the agents, then the agents that implement these traits can be put into a vec that is iterated over, and that's our main sim loop.

### Strategies

Both the local agents, the arbitrageur and liquidity provider, need access to the Strategy methods to interact with the strategy, whether that's computing actions to take or taking the action. For example, the arbitrageur needs to get price info from the strategy's respective contracts, while the liquidity provider needs to initialize the pool and add tokens into it.

These agents are sharing the same interface, that can be a trait implemented over the contract instance.

## Standardization
We can define notation in math such as
- $w_x$ which is the weight of the x token in a pool, and give it a standard name in code by:
$$ w_x \equiv \mathtt{weight\_x} $$
This way we can stay aligned on how we reference everything.