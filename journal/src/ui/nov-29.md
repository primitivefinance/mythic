# progress

We have:
- portfolio table view
- adjust portfolio weights form fields
- stages for making a portfolio adjustment
    - Weight change
    - Parameter selection
    - Simulation
    - Execution
- rough create portfolio


Right now we load a portfolio from file, then propose adjustments. Once adjustments are proposed the changes in the portfolio are computed.
This can be improved a ton. Then these adjustments are packaged and sent to the "staging" screens. In the staging the user will then pick
parameters. These parameters define some of the strategy behavior, which gets stored in the screen as its own package. Finally, when we want to transition to the simulate screen we pass both of these packages to the simulate stage area. It runs the simulation and shows us the results, which is from the agent subscription data.


Here's what we need to work on:
- Dedicated portfolio computation, storage, and transmission. Right now we take the adjustments in, make modifications "in-flight", which are parsed/translated between string types to floats, which are then passed to the table's deltas + prepare stages. This can lead to errors, be difficult to debug, and also hard to make modifications to. A dedicated "bucket" with a clear input/output data structs with methods to modify/compute on that data is really needed.
- Simulation packaged input. The simulation stage needs a few pieces of input from the data as well as different data from the portfolio. This is all separated between the stages, which does separate how its modified, but it's not clear on everything that is going into the simulation stage and also everything that is coming out of it. This should be streamlined and it should be easy to debug all the user inputs/ data input/ data output of this stage.
- Execution needs to be built. This will require some work to bring the "scroll" into play to build the transaction payload. We will also need to improve the storage simulation + fork caching.
- Maybe we can save forking + state simulation for later and just get the execution running?
- To get execution we need to cleanup some of the chain connections I think, commit to making that before working on execution.
- We should clean up the old screens/components to make things easier
- Clearly define environment variables / config for running ui
- Load anvil contracts/deploy them in background on separate thread?

## Today:
Clean up portfolio creation + management up to simulation

We need:
- Clearly defined user inputs / outputs of each stage in the portfolio management flow
- All data structures for user input and outputs should be serializable and possible saved, so they can be reloaded.
- State should be saved even if we go to another app.
- Manipulation of user inputs should be handled with care.
- Computations (intermediary layers) on user inputs should be explicitly managed by a dedicated impl and also serialized, easily trackable, and debuggable.


## Tomorrow:
Work on the execution side.