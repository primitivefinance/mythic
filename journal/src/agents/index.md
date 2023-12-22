# Agents

Agents have behaviors that can influence the market. 
They can be used to simulate the behavior of different types of traders.
They can also be used to take administrative control of AMMs.
Our approach to agent design has been hierarchical in terms of simplicity.
An agent is basic if it doesn't depend on any other agent's behavior. 

Basic agents can be thought of as lower-level or more systemic, where their logic is consistent and non-dynamic. 
Some examples of basic agents are:

>**Block updater**: This agent would allow you to progress the chain according to any questions you want.

>**Price Oracle**: This agent is your price oracle allowing you to model oracle risk.

>**Token Admin**: Token admins are responsible for deploying tokens and managing their approvals and mints.

Some examples of more complex agents are:

>**Liquidity Providers**: These agents provide liquidity to the pool by allocating a portion of their portfolio. These agents are still non-dynamic in that they don't change their behavior based on other agents' behavior, but they depend on the token admin to mint tokens.

Some examples of dynamic agents are:

>[**Arbitrageur**](arbitrageur.md): This agent arbitrages between pools. Traditionally, LPs are considered losing to the arbitrageurs in the short term. This agent's behavior depends on the token admin, price updater oracle, and liquidity provider. Since the price updater is an oracle, this agent also depends on the oracle.

>**Weight Changer**This agent changes the weights of the pool. Depending on the heuristics of this pool, these agents vary. 
