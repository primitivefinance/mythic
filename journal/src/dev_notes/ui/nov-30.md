# Rough portfolio adjustment flow

A user inputs a desired delta to adjust one or more of the portfolio's weights by.
We might want them to specify a target weight instead of a weight delta, but we can come back to that later.

First thing that should happen is that all the other weights should "soft-lock", i.e. they are dependent on the original delta applied by the user. Actually, we should just soft-lock all the weights that are not changed and push their weights up or down depending on that.

Current process:
1. User inputs delta -> emits BeginAdjustment message
2. Get the delta from the table form
3. If all deltas are empty, just return early a stage screen update.
4. Get all the current portfolio weights.
5. Apply the proposed changes to the portfolio weights using the adjust_weights_algorithm
6. Using the new weights, compute the balance adjustments with compute_balance_adjustments_algorithm. This will loop through the weights and update the balance such that the balance = adjusted weight(index) * portfolio value / price(index).
7. Compute the adjusted market values by taking the product of balances and weights.
8. Update the table form delta cells for balance and market values of each position (red/green)
9. Return the stages message "Start"
10. Create a new prepare screen with a clone of the portfolio.
11. Creating a new prepare screen will take in the original portfolio, pass it to a PreparePayload, which holds the original and adjusted portfolios.
12. It then loops over the vector of position deltas (cost, balance, weight) and applies them to the adjusted portfolio in the payload.
13. When submit is clicked it will emit the stages message "Step".
14. Stepping from the Prepare stage to the next stage, Review, will create a new Review page with a form for the user.
15. The user will input their choices in the form, which is stored in `review.package` on submit.

16. When submit happens, stages gets stepped from Review to Simulate, the review package gets copied and stored into the stages.strategy_parameters.
17. We call stages.construct(), which creates a MiniWorldBuilder which applies the information in portfolio to a ConfigBuilder, for the sim.
18. Construct populates the config with default values, it only takes the Coins from the portfolio positions and adds those to the config.
19. After construction we are still in stages.step() function so this is where we apply the actual config changes.
20. We apply all the form inputs the user filled out in Review to the config builder.
21. We then take the Prepare payload which has all the weight adjustments and apply the original weight and new weight to the config for the "X" coin.
22. We take the original x balance and pass it to the config.
23. We create a new simulation screen, and pass in the miniworld builder (which just had its config builder edited).
24. We switch to the simulate screen.
25. Finally, we return a command that spawns the simulation and runs it to completion.


Twenty-five steps! Can we bring that down?
1. User inputs a delta for a single weight
2. A new "adjusted" portfolio gets created that absorbs that adjustment, returning a portfolio with different balances & weights. Therefore changing the market values of each position in the portfolio.
3. All stages should be made available to route to except for execute, i.e. as tabs. No deltas -> no tabs/screens. One delta -> Prepare (proposed adjustments) -> Review (parameters form) -> Simulate (simulation form). Routing between these pages should be smooth, i.e. i just switch tabs.
4. Adjusted portfolio stored in Prepare stage. User continues after looking at the prepare stage to the review form.
5. User fills out the review form, selections are stored in the Review page.
6. User continues to Simulation screen.
7. Simulation holds the stage for its own form (simulation settings) and builder (i.e. config builder via MiniWorldBuilder). User fills out simulation form.
8. User clicks "simulate".
9. Stages parent collapses all the stage's states and runs the simulation using all the information.
10. User sees the simulation results!


# finished
Finished at 11:00pm on the dot Nov 30. 

There are some quirks. Like you need to click the submit button in the flow (cant just go in tabs) or else it breaks right now. Also once you arm a sim, if you arm it and there's no review form submission, breaks the simulation button from running.

Also has some quirks on negative inputs and parsing it. Also some not so clear behavior with the portfolio data type arithmetic. All things we can clean up.
