# sims

Thinking about agent dependencies, here's kind of the graph of a simple agent dependency tree:

- Create new token admin
  - Inserts account when we create the revm middleware client.
  - Deploys arbiter token contracts using the client
  - Returns token admin { client, token0, token1 }
- Create price changer
  - Inserts account when we create revm middleware client
  - Dependent on token admin
  - Calls the token admin to get addresses, which are constructor args for price changer's deployment of liquidity exchange contract
  - Deploys liquid exchange contract
  - Calls token admin to mint tokens to the liquid exchange

so:
token admin -> [token, token] -> price changer -> exchange([token, token]) -> token admin mint tokens(exchange)

Does it load dependencies, or does it search for them? Searching would get complex because it would need to find the "right" one.

Well also, these dependencies are not just contracts, they are other agents, which have their own api.

Contracts -> bytecode
Agents -> account + api
Protocol -> contracts + api


I'm thinking we can search for agent entities by filtering their transactions. i.e. token admin does two deployment transactions with erc20s. this is their "dna" when they are created with `new`.

We can potentially add a receiver to a dedicated agent that collects all logs emitted from the transaction instruction. We can filter by the from address.


# generalize arb

I have a current price and target price.

I want to find a trade such that target price - price >= 0 if price is < target price. Else price - target price >= 0 if price is > target price


I want to raise the price. I need to buy X by selling y. I'm swapping y in.

I have at target price, i need to find a y in such that the price is slightly above the target price.

bounds for y:
- can trade as little as 1 y in.
- can trade as much as strike price - y / l


# swap ghosts!

I've been debugging the "lower price" case for the arbitrageur. This means we are swapping X -> Y, so swapInX = true.

The bisection was reverting because the "lower" result was returning 0 and the "higher result" was returning a value below the target price. This means the root is outside the points we chose...

I was trying to figure out why and I noticed that a very tiny input would leave a larger price! This would put us above the price target, making the bisection work since the points are around the target price.

So it's not intuitive right away, but when swapping X -> Y, a lower trade input = higher price, and higher trade input = lower price. So these results that you get in the bisection are sort of flipped from how you expect them to be. I thought a 0 lower price was normal, but that's not the case for the X -> Y bisection scenario!

So why didn't my original lower bound of 1 work? Well a low enough value and the approximated output amount will be rounded, resulting in an invalid trade. So you need to have just enough to where its the minimum trade size but just enough so its above the rounding error of ~1-3 wei.

I found a min x trade input to be 6. Which might not be universal for all parameters.

Also make sure to find an epsilon around ~5 wei thats positive!