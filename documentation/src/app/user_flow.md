# User Flow

Open app/Run sim
Online/offline mode
Login/Create new user
Create new user -> username, passcode
App opens with their user connected.
Open portfolio management app
Load/Create portfolio
Create new portfolio -> choose coins, What coins do you care about? We should run anvil from a loaded state that has our "developer" state which has tokens deployed, which we can populate a token list with.
Default rpc added is anvil running in background.
Default signer added is a local random one.
Coin balances fetched from signer. we can have these funded.
Strategies can be loaded from chain as well. So coin balances + strategy deposits are "protocol" clients. Maybe we make the user an "agent client" itself that has a dependency of the protocol?
This might make it easy in the team setting, then it might be more straightforward to load up new agents especially if teammates are agents.
I want to make an adjustment to my portfolio.
I choose the deltas, then the parameters of the adjustment. Simulate.
Simulate should show a "condensed" summary of the results. There should be a button that pops it out to the main simulation dashboard, i.e. load the sim. So we really need to define how the sims are saved and loaded.
Simulate dashboard is just loading revm db. Its really the agents that run on the sim that make it rich.