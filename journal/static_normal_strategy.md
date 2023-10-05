# Static (Log-)Normal Strategy

The normal strategy represents a log-normal distribution around a price $K$ with a width given by $\sigma$.
The trading function for this strategy is given by
$$
\Phi^{-1}\left(\frac{x}{L}\right)+\Phi^{-1}\left(\frac{y}{KL}\right)=-\sigma.
$$
In the equation above, $x$ and $y$ are reserves, and $L$ is *a* measure of liquidity.
Given the domain of $\Phi^{-1}$ we can see that $x\in[0,L]$ and $y\in[0,KL]$.
As the pool's liquidity increases, the maximal amount of each reserve increases.

## Determining $L$

There are a few distinct times where we need to determine the value of $L$, but they all come down to liquidity being deposited into the pool and not from swaps.
We want to disentangle swaps and liquidity provision/donation.
That will make this all clearer and easier to tackle, in my mind.

### Pool Initialization

When the pool is initialized, we need to determine the value of $L$.
The user will provide a price $S$ and an amount of $x$ or an amount of $y$ that they wish to tender. 
From there, we should be able to determine how much of both tokens must be allocated as well as the value of $L$.

#### Specifying $x$

Suppose that the user specifies the amount $x$ they wish to allocate and they also choose a price $S$.
Without showing all the work, we can recall that $\frac{x}{L}$ is one of the option binaries:
$$
\frac{x}{L} = 1-\Phi^{-1}\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)
$$
Since we know $x$ and we know $S$, we can solve for $L$ to find:
$$
\boxed{L(x,S) = \frac{x}{1-\Phi^{-1}\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)}}
$$
Further, we need to know how much $y$ to allocate, which we can also use the other binary:
$$
\frac{y}{KL} = \Phi^{-1}\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)
$$
At this point, we know $S$ and $L$ and so we can get:
$$
\boxed{y(x,S) = K\cdot L(x,S)\cdot \Phi^{-1}\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)}
$$
Note that the above is not simplified and likely could be drastically simplified.

#### Specifying $y$

Suppose that the user specifies the amount $y$ they wish to allocate and they also choose a price $S$.
The work here is basically a mirrored image of the above.
$$
\frac{y}{KL} = \Phi^{-1}\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)
$$
From here we get $L$:
$$
\boxed{L(y,S) = \frac{y}{K\cdot\Phi^{-1}\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)}}
$$
Now we need to get $x$:
$$
\boxed{x(y,S) = L(y,S)\cdot\left(1-\Phi^{-1}\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)\right)}
$$

### Adding Liquidity

When a user adds liquidity, they will specify an amount of $x$ or an amount of $y$, and the pool's price $S$ and liquidity $L$ will already be known. 
When adding liquidity, we assume that price will not change whatsoever and only the value of $L$ will change.

#### Specifying $x$
Given some amount of $\delta_x$ the user wants to add, we can just use the equation for $L(x,S)$ above to get:
$$
\boxed{L(x+\delta_x,S) = \frac{x+\delta_x}{1-\Phi^{-1}\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)}}
$$
In fact, $L$ is linear in the first variable, so:
$$
L(x+\delta_x,S) = L(x,S)+\underbrace{L(\delta_x,S)}_{\delta_L}
$$
can be used to make the calculation easier.

#### Specifying $y$
Given some amount of $\delta_y$ the user wants to add, we can just use the equation for $L(y,S)$ above to get:
$$
\boxed{L(y+\delta_y,S) = \frac{y+\delta_y}{K\cdot\Phi^{-1}\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)}}
$$
Again, $L$ is linear in the first variable, so:
$$
L(y+\delta_y,S) = L(y,S)+\underbrace{L(\delta_y,S)}_{\delta_L}
$$
can be used to make the calculation easier.

### Removing Liquidity

When a user removes liquidity, they will specify an amount of $x$ or an amount of $y$, and the pool's price $S$ and liquidity $L$ will already be known. 
When removing liquidity, we assume that price will not change whatsoever and only the value of $L$ will change.
We can just use the same formulation as above and note that $\delta_x$ and $\delta_y$ may be positive or negative.

### Swaps

When a user swaps, it must be that the trading function remains invariant:
$$
\Phi^{-1}\left(\frac{x+\Delta_x}{L}\right)+\Phi^{-1}\left(\frac{y+\Delta_y}{KL}\right)=-\sigma.
$$
Note again I'm allowing for $\Delta_x$ and $\Delta_y$ to be positive or negative.
In absence of fees, the liquidity $L$ is invariant, so it is a matter of finding the $\Delta_x(\Delta_y)$ or $\Delta_y(\Delta_x)$ that satisfies the above equation (which we definitely know).

#### With Fees
Assume now that there is a fee parameter $\gamma$ such that the fee invested into the pool is $1-\gamma$. 
Assume further that the fee is always taken out of the input token for the swap.
Think of the swap as a two step process:

1. Adding liquidity. E.g., $\delta_x \coloneqq (1-\gamma)\Delta_x$. 
This is the amount of the input token that is added to the pool and it is what is used to calculate the change in liquidity $\delta_L$.
2. Computing a no-fee swap with the remaining amount of the input token. E.g., $\widetilde{\Delta_x} \coloneqq \gamma\Delta_x$.

Then we can use all of the rules we defined here.