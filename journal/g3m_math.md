# Exchange Math

This will be all the math involved with the geometric mean market maker. 
Note that there are two ways to write it out, and we should use whichever is simpler.

## Starting work

The trading function is defined by:
$$
\psi(x,y) = x^{w_x} y^{w_y} = k
$$
where $w_x+w_y = 1$.

### Swap Calculation

If we want to tender $\delta$ (could be $\delta_x$ or $\delta_y$) to the pool, then the invariant must hold.
So we can write:
$$
k = (x+\delta_x)^{w_x}(y+\delta_y)^{w_y}
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

### Price
Then we can get the price of the pool (for $x$ in terms of $y$):
$$
p = \frac{\nabla_x \psi}{\nabla_y \psi}
$$
for which we can take the derivatives to get
$$
p = \frac{w_x}{w_y}\frac{y}{x}
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
\Delta_y = p\frac{w_y}{w_x}(x+\Delta_x)-y
$$
and similarly if they choose a given $\Delta_y$, then they must have:
$$
\Delta_x = \frac{1}{p}\frac{w_x}{w_y}(y+\Delta_y)-x
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