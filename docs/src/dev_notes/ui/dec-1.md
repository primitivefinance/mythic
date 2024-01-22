# its time to __really__ cook

Now that we have a rough marble cutout of everything we need it can be refined with a much higher degree of precision.


Here's some problems that I've come across:
- Getting state from a simulation environment.
- Debugging revert messages.
- Heuristic for knowing if something is working is by roughly looking at the stack traces and seeing if we are hitting the expected debug logs (i.e. no reverts).
- Underlying unit types are misused or difficult to analyze.
- Setting up agents.
- Editing agents and their parameters.
- IStrategy interface propagating changes to other parts of the codebase.
- Rough flow is change sim/contracts/config -> run sim, see what breaks -> debug -> fix.
- Difficult to debug pure functions in smart contracts
- Difficult to debug intermediary functions and values in the contracts.
- Testing infrastructure is lacking because its hard to setup the context to test different functionalities in.
- Need to create libraries/contracts with public functions to expose the functions of libraries.
- Values are displayed in native types with not much information about their type (i.e. fee/gamma)
- No easy way to analyze sim results right now.
- No easy way to analyze event logs.
- Agents take in dependencies in their constructor


Ideal solution:
- Run sim `cargo run sim`
- Sim runs in background, application loads.
- On sim complete, application loads the simulation's db, event logs.
- With the db and event logs we can recreate the instance, build graphs of the events, fetch state, and render it all for the user.
- "Simulation dashboard", maybe the portfolio management routes users to the simulation dashboard.
- "Agent receipts" show a summary of an agent's activity in a sim over the lifetime. I should be able to go through all the transactions that were executed in the sim, these should also be labeled. I want graphs of the event data rendered.

In the portfolio management, we run the sim in-window, but maybe we can add a button to "Open in Simulation Dashboard" for users to deep dive into a strategy. The simulation in the portfolio management could be a condensed version?

Okay so the simulation dashboard is the BIG part of this whole thing, and should ideally just load data in. We can pass a flag to run the app using a sim run outside of it? I.e. run sim -> pass flag to app -> load simulation dashboard in app from the sim that was run, maybe just a path.

Okay these are our two apps then:
- Terminal - Simulation dashboard with state data, transaction history, event log graphs, contracts deployed, agents deployed, addresses interacting.
- Portfolio Management - Create a portfolio of assets. Propose adjustments. Simulate and execute those adjustment strategies.


Then we can have these portfolios saved and loaded back up. They should also have their active strategies saved? We can have a developer portfolio that has the initial config parameters for example, i.e. x/y coins.

We are definitely missing the strategy part of this. Right now there's no standardized way to get the information of a strategy or kind of understand what's going on. How do we know a portfolio has an active strategy? The portfolio should probably be attached to an account, or is it an abstraction of positions across accounts?

Then we have things like signer management, rpc management. Signer/provider. Chain management. What's the account hierarchy? We need an account creation flow, and team accounts. Where is account info saved?


## terminal
The terminal is a rough block explorer effectively.
- We want to see a list of all transactions
- We want to see the event logs of those transactions
- We want to see state changes of the transactions

Ah, it kind of clicked here for me. Instead of a generalized block explorer like etherscan, this is a focused block explorer that knows about all the possible transactions / abis. This lets us decode the events + state changes in a much more human-readable manner.

Each agent is created as a client with an address, so itd really be like looking at a specific account on etherscan. Each transaction can have a view that goes into more detail of whats going on in the transaction.

The relationship between a sim and a strategy for us is important then. If we take the strategy as the primitive underlying type, then we can plug it into the sim context and also execute it. When we run a sim, we are looking at the heuristics defined by the strategy, its basically whats important to us, since we plan on executing it.

Okay so maybe they are more specific to the portfolio management app.

Portfolio management -> strategy -> simulation -> agents mock our execution -> simulation terminal renders the simulation -> execution will execute the strategy.

# contracts/interface

We probably should just make a portfolio client for communicating with the protocol. the simulation could probably leverage this client as well. We need ways to get the strategies, this will probably be by indexing an event from the chain.

itd be easier to query this than by using the agents as a proxy. the agents should communicate with this client directly? it creates another dependency though. it would replace the direct bindings connection, which might be better actually anyway. is protocol client part of an agent's core code?


# rpc management
We should make it easy to manage rpcs. For example, anvil starts in the background of the app. But it creates an endpoint with a random port. We should be able to add an rpc, i.e. the background anvil, and use it for a provider connection.

# signer management
In dev mode we load a local private key from an environment variable, which serves as a local signer. We should be able to add a signer, i.e. a hardware wallet, and make it an automatic choice when executing transactions (from address?).


## Things to build:
- Channel that gets all the receiving transactions in arbiter
- Channel that gets all rpc calls made by non-arbiter client
- Agent transactions get their own channel? Or do we filter the global transactions by the agent address. we probably pass the raw transactions from the global channel to the agent directly and it can "human readable" the transactions for us. Viewing an agent maybe just applies the filter to the global transaction and the agent applies a "lens"? the lens is like glasses into the transaction that lets us see its labeled information better.
- Create new username + encrypted passcode in db
- Auth user login from db
- fetch user strategy deposits in a protocol. protocol + protocol client probably determines this.
- connect protocol client to arbiter instance. clicking on the protocol's smart contract will access the protocol client to give us a view into it.
- okay so agents are like an account client. got it.
- Portfolios. These are abstract containers for tokens. We will initially create a portfolio of two tokens, but then want to put them into a strategy. We can make the assumption that allocating to a strategy uses the entire portfolio of assets? I.e. if we dollar cost average, we need to deposit all of our input tokens so they can get swapped to output tokens.
- Portfolios + strategies we load from chain? We probably should, then we have our abstract "target" portfolio.
- If we save our anvil instance that we initially ran in the background then we can load it back up. Ah yep we have rpc methods for this, dump state / load state.'


# full user flow:
- Open app/Run sim
- Online/offline mode
- Login/Create new user
- Create new user -> username, passcode
- App opens with their user connected.
- Open portfolio management app
- Load/Create portfolio
- Create new portfolio -> choose coins, What coins do you care about? We should run anvil from a loaded state that has our "developer" state which has tokens deployed, which we can populate a token list with.
- Default rpc added is anvil running in background.
- Default signer added is a local random one.
- Coin balances fetched from signer. we can have these funded.
- Strategies can be loaded from chain as well. So coin balances + strategy deposits are "protocol" clients. Maybe we make the user an "agent client" itself that has a dependency of the protocol?
- This might make it easy in the team setting, then it might be more straightforward to load up new agents especially if teammates are agents.
- I want to make an adjustment to my portfolio.
- I choose the deltas, then the parameters of the adjustment. Simulate. 
- Simulate should show a "condensed" summary of the results. There should be a button that pops it out to the main simulation dashboard, i.e. load the sim. So we really need to define how the sims are saved and loaded.
- Simulate dashboard is just loading revm db. Its really the agents that run on the sim that make it rich.