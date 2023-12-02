# portfolio management mvp

Need to really simplify the portfolio management and polish the current flow / cut the fat.

Right now we have multiple deltas that can be made by the user, i think we cut it to just the weight %. Then from the weight % change we can compute all the deltas in the others.
We can then put in editing other deltas later (i.e. add 100 tokens to my balance).

# Adjustments review
Add better deltas table summary


# Simulation review
- hardcode strategy select in review
- Add time selects for start/end time
- Connect fee to changing the config

# Simulation
- Update the sim running logic to be much faster/parallel so we can have multiple instances.
- Let the user choose the price path from a hardcoded selection. Maybe in review? or as a sim setting.

# Execution
- Implement execution...