# Geometric Mean Market Maker

## Overview

A few notes:
- Balancer FixedPoint library is used, it would be interesting to swap it for a more efficient one.

The following things are missing:
- Single asset deposits / withdrawals
- Minimum / maximum amounts for swaps, liquidity deposits and withdrawals
- Burnt liquidity on first deposit
- Updatable swap fee?

## G3M Math

The following formulas are used in the G3M contract:

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
