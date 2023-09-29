# Geometric Mean Market Maker

## Overview

This contract is very basic implementation of an Automated Market Maker using the
Geometric Mean formula. Here are some of the current specifications (likely to change):
- One pool per contract
- Two tokens per pool (X and Y)
- Real-time weights update
- Fixed swap fee

## Usage

### Pool Creation

A pool can be created by deploying the G3M contract with the following parameters:
- Two different token addresses (X and Y)
- The weight of token X (the weight of token Y will be calculated automatically
using `1 - weightX`)

*Note that the weights are expressed in WAD units (10^18).*

### Pool Initialization

Then, the pool needs to be initialized with an initial amount of liquidity. This is done by calling the `initPool` function and sending arbitrary amounts for both tokens. These two amounts are important and will be used for two specific things:
- Set the initial spot price of the pool (see [Spot Price](#spot-price) for details)
- Compute the initial amount of liquidity in the pool (currently set to two times
the invariant)

Additionally, a small amount of liquidity is burnt, this is done to prevent the
pool from being completely drained and initialized again with a different spot
price.

### Pool Lifecycle

Once a pool is initialized, liquidity providers can supply or withdraw liquidity
and arbitrageurs can swap tokens.
The admin of the pool can also update the weights of the tokens. This action is
currently instantaneous but could be made gradual in the future.

## Notes

Here a few things to note about the current implementation:
- Balancer FixedPoint library is used, it would be interesting to swap it for a
more precise / efficient one
- WAD units are often used but not explicitly mentioned in the code. This should be fixed

Also, some extra features could be added:
- Single asset deposits / withdrawals
- Minimum / maximum amounts for swaps, liquidity deposits and withdrawals
- Updatable swap fee
- Gradual weights update

## G3M Math

The following formulas are used in the G3M contract:

*Note that these formulas are mathematical representations and do not necessarily reflect the actual Solidity implementation, for example, amounts in the contract are stored in WAD units.*

### Invariant

Computes $k$, the invariant of the pool:

$$k ={R_x}^{W_x}\cdot{R_y}^{W_y}$$

With $R_x$ the reserve of token X, $R_y$ the reserve of token Y, $W_x$ the weight of token X and $W_y$ the weight of token Y.

### Spot Price

Computes $p$, the spot price of the pool:

$$p = \frac{\frac{R_i}{W_i}}{\frac{R_o}{W_o}}$$

### Liquidity In

Computes $i$, the required amount of tokens to deposit an exact amount of liquidity:

$$i = (\frac{t + l}{t} - 1) \cdot r$$

With $l$ the liquidity to deposit and $r$ the reserve of input token.

### Liquidity Out

Computes $o$, the received amount of tokens when withdrawing an exact amount of liquidity:

$$o = (1-\frac{t-l}{t})\cdot r$$

With $l$ the liquidity to deposit and $r$ the reserve of input token.

### Amount out given amount in

Computes $A_o$, an amount out given an exact amount in:

$$A_o = B_o \cdot (1 - (\frac{B_i}{B_i+A_i}^\frac{W_i}{W_o})$$

With $B_0$ the balance of output token, $B_i$ the balance of input token, $W_i$ the weight of input token, $W_o$ the weight of output token and $A_i$ the amount in.

Please note that the amount in refers to the amount sent by the user minus the fees, as expressed here:

$$A_i = A_{sent} \cdot (1 - swapFee)$$

### Amount in given amount out

Computes $A_i$, an amount in given an exact amount out:

$$A_i = B_i \cdot ((\frac{B_o}{B_o - A_o})^\frac{W_o}{W_i}-1)$$

With $B_0$ the balance of output token, $B_i$ the balance of input token, $W_i$ the weight of input token, $W_o$ the weight of output token and $A_i$ the amount in.
