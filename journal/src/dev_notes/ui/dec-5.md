# MVP Spec
- DFMM Protocol Client
    - Execute transacations on the dfmm protocol
    - Query the dfmm protocol for its state
    - Index dfmm protocol events and display them in real time.
- ABS Simulation of DFMM strategies.
    - Simulate n steps with agents connected to the DFMM protocol.
    - Display state data returned after the sim to the user in terminal
- RPC Management
    - List of available rpc urls to connect to
    - Connecting to rpc will make it an available provider
    - Default connection to local anvil in background
- Signer Management
    - List of available signers
    - Selecting a signer will make it available to use in the app
    - Default to a LocalWallet signer.
- Contacts
    - List of addresses, categorized and classified.
    - Selecting an address will copy it
    - Form to add addresses



The RPC/Signer/Contacts is a similar component that stores information the user can choose which has an effect on whats available in the underlying app.

The DFMM protocol client is the most straightforward application to make.

The ABS simulation will be the most complex, so we can do that last.


Problems to figure out:
- How to make signers/rpcs/contacts universally accessed by the app's functionalities.
- How to manage multiple connections. I.e. transactions are connected to RPC #1 but fetching state is connected to RPC #2.

Potentially solution:
- Root app holds the state or everything it needs to fetch a signer/rpc. Components that require a signer/rpc are "boxed" and can be created with multiple instances using different inputs (signer/rpcs).
- Example: Fetch and Render component queries protocol state and renders it for the user. It requires an RPC connection. Create the component and pass the rpc to its `new()` function. Then if we want to add a new component with a different connection we just push a new one with the other rpc.