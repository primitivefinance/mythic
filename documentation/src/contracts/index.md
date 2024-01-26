# Contracts
Excalibur features a set of smart contracts designed by the Primitive team.
These contracts have a single entry point, the `DFMM.sol` contract. 
The DFMM contract integrates with contracts that implement the `IStrategy.sol` interface.
The strategy interface employs a unique validation mechanism that only accepts correct inputs.
This way, the DFMM contract only makes valid state transitions.

## Interfaces
The contracts work with two primary interfaces, `DFMM` and `Strategy`.

### DFMM
The purpose of `DFMM` is to be the driver for the liquidity pools that can handle their states updating.
It is the main touch point for users who want to be able to allocate or swap with pools. 
`DFMM` is agnostic for how the internal `Strategy` of the pool is happy to control pools that are created with a `Strategy` interface.

### Strategy
The `Strategy` interface is the logic behind the operation of a specific DFMM pool. 
Specific implementations of the `validateSwap` and other methods must be implemented here.
The openness of the interface allows developers to add their own features to the basic trading functions so that their strategies can be enveloped in the `Strategy` interface.
For instance, we have some examples [here](LINK TO A PLACE WHERE WE HAVE SOME SPECIFIC STRATS).

## Pool Updates
When the pool is told to update, we supply a new state of the pool that the contract only serves to verify is valid.
This reduces the cost and complexity of the smart contract itself and offloads much of the compute to offchain resources.
For example, if we want to compute a swap, we need to tell the pool a new value for $R_X$, $R_Y$, and $L$. 
If a wants to swap in some amount of $R_X$, then they want to get out as much $R_Y$ as possible.
This computation often requires an optimization procedure that uses a *bisection search* which would be prohibitively expensive onchain yet proving the new state is valid essentially requires evaluation of one function.

## Tokenization
The LP positions in pools are tokenized when the pool is created.
We call these Liquidity Provider Tokens ($\texttt{LPT}$ s). 
When the pools are initialized a value for liquidity $L$ is determined, and this amount of $\texttt{LPT}$ s is granted to the initial LP.
This LPT is an ERC-20 which can be used in other protocols for composability.

### Liquidity Tracking
Since the pools over time can change parameters and accumulate fees, the $L$ of the pool changes and this is kept consistent with the $\texttt{LPT}$ s provided to the LPs. 
To do so, the pool tracks an effective exchange rate between the $\texttt{LPT}$ and the liquidity $L$ of the pool. 
For instance, at initialization, the exchange rate is 1 as one $\texttt{LPT}$ can be exchanged for deallocating 1 out of the total $L$.
Suppose now that we have a swap come in and a fee changes $L \to L+\delta_L$, then the exchange rate would now be that 1 $\texttt{LPT}$ can exchange for $\frac{L+\delta_L}{L}$ out of the total $L$.