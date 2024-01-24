# App

## A Vision
Excalibur aims to be a be-all-end all desktop application to service modern decentralized finance. 
The changing financial landscape requires new tools for management for everyone.
It begins with a portfolio management application that allows users to manage their assets and strategies.
These strategies are given not just as Ethereum smart contracts, but they are strapped alongside of the ability to simulate them in a sandbox environment, deploy them to a network, monitor their status, feed updates to offline agents, update the onchain system, and more.

## Contract Interface
Instead of a generalized block explorer like etherscan, Excalibur is a focused block explorer that knows all about specific contract's possible transactions and interactions. 
This lets us work with state changes in a much more human digestible way.

## Agents
Agents are created as a client with an address. 
Each transaction can have a view that goes into more detail of whats going on in the transaction.
Agents are the glue for the relationship between a simultion and strategy or offline and online.  

When we run a sim, we are looking at the heuristics defined by the strategy, its basically whats important to us, since we plan on executing it.
These, at the momeent, are more specific to the portfolio management app.

Portfolio management -> strategy -> simulation -> agents mock our execution -> simulation terminal renders the simulation -> execution will execute the strategy.

## RPC Management
We should make it easy to manage rpcs. 
For example, anvil starts in the background of the app. But it creates an endpoint with a random port. 
We should be able to add an rpc, i.e. the background anvil, and use it for a provider connection.

## Signer Management
In dev mode we load a local private key from an environment variable, which serves as a local signer. 
We should be able to add a signer, i.e. a hardware wallet, and make it an automatic choice when executing transactions (from address?).

## Address Book
- List of addresses, categorized and classified.
- Selecting an address will copy it
- Form to add addresses

