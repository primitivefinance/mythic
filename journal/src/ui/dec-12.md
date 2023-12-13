# providers and signers

We need three at least:
- revm middleware for arbiter
- provider w/ ws or http for a network
- anvil instance + provider w/ ws/http


The revm middleware is the sandbox. Provider is production network, and anvil can be a dev flag. If dev flag is on it can connect to anvil.
We want to have these all simultaneously.