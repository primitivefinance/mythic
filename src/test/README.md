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

- Faulty strategy allowing to withdraw more tokens than the pool reserves.
- Rounding issues allowing to get more and more tokens by allocating and deallocating back and forth
- 