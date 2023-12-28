# Building forking into interface

## Anvil
- Problem: Need to test using a local network. Anvil is the solution. Error: program not found
- Solution: Add anvil `.foundry/bin` to user path.

## Ethersdb
- Problem: Ethersdb instantiation uses `block_on`. Breaks if within tokio runtime. Tests are inside tokio runtime!
- Solution: Do `EthersDb::<basic>` inside a separate __non-tokio__ thread.
- `#[tokio::test(flavor = "multi_thread", worker_threads = 1)]` is clutch.