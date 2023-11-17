## Tx Simulation + Execution

Overview of the process from start to end of building a transaction, simulation and then executing it.

- Gather inputs from user.
- Craft transaction payload.
- Seal the payload (i.e. make sure modifications can only happen if we revert back in the process).
- Instantiate the forker to fetch the target's account info and storage.
- Load this account into arbiter.
- Send the payload in arbiter.
- Load the target's account db via the `client.apply_cheatcode` method using the `Access` cheatcode.
- Compare the DbAccounts from before and after
- Return the result of this analysis for rendering on the UI
- Enable UI components (i.e. confirm).
- Execute the transaction on confirm.
- On successful transaction reciept, render the receipts details in the UI.