# portfolio table stuff

Here's the rough flow:
- Create a new dashboard for a portfolio with a given name.
- Call `load()`, which triggers the dashboard's message `Load`.
    - Update the portfolio state in the dashboard to the loaded portfolio.
    - Call `table.update()`, to propagate the changes to the table.
        - Update the positions in the table's state, which get rendered by the table.


Then when one of the table's form fields (e.g. balance delta) gets edited:
- Triggers a `DeltaForm::` message. When messages get triggered in these children components, they are basically moved to the top of the parent/children stack and propagated down. For example, it's possible to capture the event in a parent component. But in this case we just capture it and send it directly to the component that it was triggered in (the table).
- Table gets the `DeltaForm` message, and propagates the changes to the `DeltaForm`, which the table holds its own state of.
- The form is a child of Table because the form is in the table, and the form does not have its own view message. Making the form a child of table made sense because they are integrated with eachother.

Now when it comes to the summary of the form's changes, it's not as straightforward. The table should only manage the table and its cells. This is reflected in it's view message only returning the position table built and nothing more.

For rendering a summary of the changes we want to:
- 1. Only render the summary after the user does an action to look at the summary
- 2. Render all the Some(_) values in the form.

The form is in the table, so we need to get that information by communicating with that form.
I think the summary should directly get that information, so maybe the table can expose it. Another idea was to capture the form events and put that data into the summary component, but that makes it complicated because we are like syncing the summary with the form instead of directly calling it.

So to summarize the hierarchy:
- Dashboard/Screen
    - Table
        - (Sibling component) Summary
            - (Child component) Form 


# staging

Getting kind of caught up in the complexity in the staging for the portfolio adjustment flow, so going to describe it here:
1. Start with just a view of table.
2. Edit field in table
3. Review table edits (renders the summary table)
4. Submit -> Review adjustment transaction
5.  -> Renders its own view that has a button to trigger the next step.


# next phase

Finished the scaffolding for the portfolio management flow. This codebase is just massive... Finished this at 4:15pm Nov 26, started on Nov 23. Now I need to "make it alive" by having it do real simulation/transactions!


First thing is lets write out the process of what happens:
- I have a portfolio of some coins. This portfolio is just the sum of the coins effectively, its not allocated to an LP or anything.
- I change the target weights for my tokens, say 60/40.
- After reviewing these weight changes, I go forward to prepare the adjustment.
    - I select a start and end time for my adjustment to occur.
    - I select the fee on my pool
    - I select the strategy (i.e. dca)
- Then I simulate this.
    - The simulation will execute the strategy over the timeline I specified in arbiter
    - It renders the outcome of this simulation in the simulation screen
- After i review the simulated results, i execute
    - Execute will deposit the coins in the strategy
    - I am redirected to my portfolio with a new "strategy" implemented at the top.
    - My portfolio tracks the balances in the strategy instead of actual token balances.

We can hook up the sim stuff before connecting it to the real form, so lets do that.