# Geometric Mean

This will be all the math involved with the Geometric Mean (G3M) trading function. 

## Conceptual Overview

G3M (for two assets) consists of a pool of reserves and their associated weights.

The G3M effectively gives the LP a portfolio that consists of a fixed ratio of the two assets based on the internal pricing mechanism.
For instance, if we pick the weight of the $X$-token $0.80$ and $0.20$ for the $Y$-token, then the LP will have a portfolio that is 80% in $X$ and 20% $Y$ by price.
This is a basic building block for a lot of portfolio designs.

## Core

Mechanically, G3M of two variable parameters:
- $w_x \equiv \mathtt{weight\_x}$
- $w_y \equiv \mathtt{weight\_y}$ 
- These parameters must satisfy $w_x+w_y=1$.

Next, we define the trading function to be:
$$
\varphi(x,y) = x^{w_x} y^{w_y} = k
$$
where $k$ is the invariant of the pool. 
We can put:
$$
k \equiv \mathtt{invariant}
$$

### Price
If we compute the derivatives and simplify the expression, we get that the pool price is:
$$
\boxed{p = \frac{w_x}{w_y}\frac{y}{x}}
$$

### Swap 

We require that the trading function remain invariant like so:
$$
k = (x+\Delta_x)^{w_x}(y+\Delta_y)^{w_y}
$$
Now let's get the $\delta_y$ from $\delta_x$:
$$
\begin{align*}
k = (x+\delta_x)^{w_x}(y+\delta_y)^{w_y} \\
\implies \qquad (y+\delta_y)^{w_y} = \frac{k}{(x+\delta_x)^{w_x}}\\
\implies \qquad y+\delta_y = \left(\frac{k}{(x+\delta_x)^{w_x}}\right)^{1/w_y}\\
\implies \qquad \boxed{ \delta_y = \left(\frac{k}{(x+\delta_x)^{w_x}}\right)^{1/w_y} - y }
\end{align*}
$$
On the other hand, if we want to get out $\delta_x$ from $\delta_y$:
$$
\begin{align*}
k = (x+\delta_x)^{w_x}(y+\delta_y)^{w_y} \\
\implies \qquad (x+\delta_x)^{w_x} = \frac{k}{(y+\delta_y)^{w_y}}\\
\implies \qquad \boxed{ \delta_x = \left(\frac{k}{(y+\delta_y)^{w_y}}\right)^{1/w_x} - x }
\end{align*}
$$



### Liquidity Provision
It must be that adding liquidity does not change the price of the pool. 
This makes it quite simple to add liquidity. 
If a user wants to add liquidity, they can just add the tokens such that the ratio of the reserves does not change.
If a user wants to input $\Delta_x$ and $\Delta_y$ to the pool, then they must have:
$$
p = \frac{w_x}{w_y} \frac{y}{x}  = \frac{w_x}{w_y} \frac{y+\Delta_y}{x+\Delta_x}
$$
which implies if they choose a given $\Delta_x$, then they must have:
$$
\Delta_y = \frac{y}{x}(x+\Delta_x)-y
$$
and similarly if they choose a given $\Delta_y$, then they must have:
$$
\Delta_x = \frac{x}{y}(y+\Delta_y)-x
$$



---

## G3M Arbitrage Math


We can solve for each variable in terms of the other and the invariant $k$:
$$
x^{w_x}y^{w_y} = k
$$

First, $x$:
$$
\implies \boxed{x = \left(\frac{k}{y^{w_y}}\right)^{1/w_x} }
$$

The work is analogous for $y$:
$$
\implies \boxed{y = \left(\frac{k}{x^{w_x}}\right)^{1/w_y}}
$$

### Getting the arbitrage calculation

#### For Lowering Price
Suppose that we need the price to move $p\mapsto p'$ with $p'<p$. 
This means we tender $x$ in the swap so $x\mapsto x+\delta_x$. 
Then we want $p'$ and $x\mapsto x+\delta_x$:
$$
p' = \frac{w_x}{w_y}\frac{y+\delta_y}{x+\delta_x}
$$
Now we can replace the $y+\delta_y$ with our equation above to get:
$$
p'=\frac{w_x}{w_y}\frac{\left( \frac{k}{(x+\delta_x)^{w_x}}\right)^{1/w_y}}{x+\delta_x}
$$
Then solving for $\delta_x$ yields
$$
\implies  \delta_x = \left(\frac{w_x}{w_y}\frac{k^{1/w_y}}{p'}\right)^{\frac{1}{1+w_x/w_y}}-x 
$$
Which we can simplified to:
$$
\implies \boxed{ \delta_x = k\left(\frac{w_x}{w_y}\frac{1}{p'}\right)^{w_y}-x }
$$

#### For Raising Price
Suppose that we need the price to move $p\mapsto p'$ with $p'>p$. 
This means we tender $x$ in the swap so $y\mapsto y+\delta_x$. 
Then we want $p'$ and $y\mapsto y+\delta_y$ with:
$$
p' = \frac{w_x}{w_y}\frac{y+\delta_y}{x+\delta_x}
$$
Now we can replace the $y+\delta_y$ with our equation above to get:
$$
p'=\frac{w_x}{w_y}\frac{y+\delta_y}{\left( \frac{k}{(y+\delta_y)^{w_y}}\right)^{1/w_x}}
$$
Then solving for $\delta_x$ yields
$$
\implies  \delta_y = \left(\frac{w_y}{w_x}p'k^{1/w_x}\right)^{\frac{1}{1+w_y/w_x}}-y 
$$

This can be simplified to:
$$
\implies \boxed{ \delta_y = k\left(\frac{w_y}{w_x}p'\right)^{w_x}-y }
$$

---
# Works in Progress
### WIP: Liquidity
The way we can think about liquidity is how many tokens would be in the pool if their prices were equal, i.e., $p=1$. 
If this is the case, then:
$$
x=\frac{w_x}{w_y}y
$$
Using the trading function:
$$
x^{w_x}y^{w_y} = \left(\frac{w_x}{w_y}y\right)^{w_x} y^{w_y}= \left(\frac{w_x}{w_y}\right)^{w_x} y = k
$$