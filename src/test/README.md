# DFMM Testing Suite

## Overview

DFMM tests are separated into 4 different types:
- `unit` tests are checking the behavior of specific functions within a local setup composed of mock contracts
- `fork` tests are similar to unit tests but are run against real world contracts on forked networks
- `attack` tests are simulating attacks against the contracts based on previous DeFi protocol exploits
- `invariant` tests are checking if the contracts are holding specific invariant

## WIP

This section is just a draft and random notes for now:

### Previous attacks

| Designation | Description | Example | Scope |
|---|---|---|---|
| Rounding attacks |  |   |  |
| Exchange rate manipulation |  |   |  |
| Inflation attack |   |  https://blog.openzeppelin.com/a-novel-defense-against-erc4626-inflation-attacks |  |
| Minimal Proxy  |  |   |  |
| Reentrancy  |  |   |  |
| Liquidity manipulation  |  |   |  |


### Potential exploits

Potential exploits and how we can prevent them:

| Exploit | Prevention |
|---|---|
| Faulty strategy allowing to withdraw more tokens than the pool reserves. | Pool reserves are tracked in the `Pool` structure and the amount of tokens out is withdrawn before performing the actual transfer, this means that removing more than the pool reserves will trigger an underflow and revert the transaction. |
| Rounding issues allowing to get more and more tokens by allocating and deallocating back and forth. | The amount of minted or burnt liquidity is always rounded in the favor of the DFMM. |
| Sending tokens directly into the DFMM contract to manipulate the reserves and withdraw more than the pool reserves. | The reserves are not fetched using `balanceOf` but are tracked in the `Pool` structure. During a `transfer` the amount of tokens received is checked and only what is expected is added to the reserves. |
| Receiving more tokens by swapping back and forth. | TBD |
| LPTs inflation attack | A small portion of the initial LPTs is burnt |
| Reentrancy | All the functions are using a reentrancy lock and external calls are performed at the end of each function. |