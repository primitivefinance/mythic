# forking

Today matt brought up a good point that we need to cache the keys we want to look up in the mappings when forking.

For example I'm working on the token transfer call simulation. For this, we need to loop up the mapping `balance_of` of the caller.

For other calls, there might be more state that we need to load. We also need to load the target's balance to see the change in storage.

This is a simple example but I can see how it might get complicated.