## Current state
- Core app structure is solid, extensible, and maintainable, so goals on that front achieves.
- world management / interaction is pretty strong right now. We can manage the simulation instance and communicate them in thread.
- tracing is verbose, but strong. Stack traces can be upgraded in straightforward ways, i.e. adding more traces.


## What's weak
- Data pipes. The app is piped data from stack traces in a... suboptimal format. The AppEventLog
- Separation of compute, rendering, and business logic. For example, view components are handling logic and parsing data then rendering, which happens repeatedly and slows down the entire application. View components are basically forced to be pure to have high performance
- Application updates are coupled with simulation updates, which could drag/entangle the performance of simulations with  the application runtime, also not desired.
- Still missing some primitive components for styling, but not too much of a priority
- Testing is weak
- No measures in place to measure performance!
- We don't use channels too much for app messages, could be good or bad thing?
- Currently no config customization


## How we can upgrade
- i want to plug in data pipes. I need a "data center" to handle those pipes. I should be able to easily plug a new data source into the core app so other components of the app have access to it. For example, we have no smart contract level events being caught, i should be able to plug those into the app if I want!
- Sim manager only does slow interactive based sims. We also want the fast parallized kind with no inter thread comms, and also a sim class in the middle that is performant but emergent (i.e. event driven).
- Rendering data from event logs output after a fast "optimizing sweep" sim runs.
- Dashboard maybe has an overview of the instances? Would be too much if we have 100 in parallel.
- Dashboard should just render the key results of sim so we can look at that, then we have views for more granular look.
- Automated checking for anomalies across parallized sims
- Views for ONE sim. List of all sims. One / many heuristics to each sim. I.e. table list of all sim instances, each has their own "health". Health is measured by the success rate of agents, and thats something defined by the agent themselves. Click view more to view the sim in the view to see what happened.
- Contracts management. View and manage all contracts involved in my simulation.
- module management. edit stuff in base module / strategy module, etc.
- data export. we output event logs, we should export stack traces.
