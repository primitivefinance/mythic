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