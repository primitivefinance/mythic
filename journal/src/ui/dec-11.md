# Status

We have built every primitive requirement we need to ship this app, now we need to tidy it up and make sure it runs smoothly without panics.

## We built:
- Abstraction for middleware/signer/provider connections
- Profile + other config saving to local directory
- Chart library
- UI components library + design system
- Data model
- View model


## todo:
- [] Deprecate "dev" / ledger client in favor of Cortex trait.
- [] Block subscriptions to refetch data
- [] Potentially simplify the screens to a more monolithic file
- [] User transaction flows are not done
- [] Settings screen is not done
- [] Make a better underlying portfolio data model that can be updated from the core data model easier.
- [] Profile saving + portfolio saving/loading needs to be reworked with the data model.
- [] Add target weights to table
- [] Spec out the simulation tab/component, what do we want in there?
- [] Connect rpc settings to the application
- [] Improve loading time of the sandbox. If we do setup in one call maybe its faster?
- [] Historical data, this might be a challenge/where indexing comes in.
- [] Fix liquidity distribution chart
- [] Add token balance series to stack the live portfolio value series.
- [] Add theoretical curve to the liquidity curve chart
- [] Add price curve for the liquidity curve chart
- [] Bug: initializing with a larger deposit slows the app to a freeze?


# refactor to do now:
- Lift the data model to the root application so that going between dashboard/settings does not clear its state.
- Redo the App's state. Remove: streams, chains, ledger, dev client, storage. Add: instead of storage, just a single loaded profile or a data dir to get it, cortex, data model. We should have: cache, window, profile/datadir, cortex, sidebar.