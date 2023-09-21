# Exchange Math

This will be all the math involved with the geometric mean market maker. 
Note that there are two ways to write it out, and we should use whichever is simpler.

## Starting work

### Trading Function with Extra Constants
The trading function is defined by:
$$
\psi(x,y) = \left(\frac{x}{w_x}\right)^{w_x} \left( \frac{y}{w_y} \right)^{w_y} = k
$$
It also must be that $w_x+w_y = 1$.


#### Price
Then we can get the price of the pool (for $x$ in terms of $y$):
$$
p = \frac{\nabla_x \psi}{\nabla_y \psi}
$$
for which:
$$
\begin{align*}
\nabla_x \psi &= w_x\frac{x^{w_x-1}}{w_x^{w_x}}\left(\frac{y}{w_y}\right)^{w_y} = \left(\frac{x}{w_x}\right)^{-w_y}\left(\frac{y}{w_y}\right)^{w_y}\\
\nabla_y \psi &= w_y\left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{w_y-1} = \left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{-w_x}
\end{align*}
$$
therefore:
$$
p = \frac{\left(\frac{x}{w_x}\right)^{-w_y}\left(\frac{y}{w_y}\right)^{w_y}}{\left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{-w_x}}
$$
$$
\implies \qquad \boxed{ p = \frac{w_x}{w_y}\frac{y}{x} }
$$

#### Swap Calculation

ANNOYING

#### Price
Then we can get the price of the pool (for $x$ in terms of $y$):
$$
p = \frac{\nabla_x \psi}{\nabla_y \psi}
$$
for which:
$$
\begin{align*}
\nabla_x \psi &= w_x\frac{x^{w_x-1}}{w_x^{w_x}}\left(\frac{y}{w_y}\right)^{w_y} = \left(\frac{x}{w_x}\right)^{-w_y}\left(\frac{y}{w_y}\right)^{w_y}\\
\nabla_y \psi &= w_y\left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{w_y-1} = \left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{-w_x}
\end{align*}
$$
therefore:
$$
p = \frac{\left(\frac{x}{w_x}\right)^{-w_y}\left(\frac{y}{w_y}\right)^{w_y}}{\left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{-w_x}} = \frac{w_x}{w_y}\frac{y}{x}
$$

### Trading Function without Extra Constants
If instead the trading function is defined by:
$$
\psi(x,y) = x^{w_x} y^{w_y} = k
$$
where $w_x+w_y = 1$.

#### Swap Calculation

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

#### Price
Then we can get the price of the pool (for $x$ in terms of $y$):
$$
p = \frac{\nabla_x \psi}{\nabla_y \psi}
$$
for which:
$$
\begin{align*}
\nabla_x \psi &= w_x\frac{x^{w_x-1}}{w_x^{w_x}}\left(\frac{y}{w_y}\right)^{w_y} = \left(\frac{x}{w_x}\right)^{-w_y}\left(\frac{y}{w_y}\right)^{w_y}\\
\nabla_y \psi &= w_y\left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{w_y-1} = \left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{-w_x}
\end{align*}
$$
therefore:
$$
p = \frac{\left(\frac{x}{w_x}\right)^{-w_y}\left(\frac{y}{w_y}\right)^{w_y}}{\left(\frac{x}{w_x}\right)^{w_x}\left(\frac{y}{w_y}\right)^{-w_x}} = \frac{w_x}{w_y}\frac{y}{x}
$$

### Liquidity Provision
It must be that adding liquidity does not change the price of the pool. 
This makes it quite simple to add liquidity. 
If a user wants to add liquidity, they can just add the tokens such that the ratio of the reserves does not change.
If a user wants to input $\Delta_x$ and $\Delta_y$ to the pool, then they must have:
$$
\frac{x+\Delta_x}{y+\Delta_y} = \frac{x}{y} 
$$
which implies if they choose a given $\Delta_x$, then they must have:
$$
\Delta_y = \frac{y(x+\Delta_x)}{x}-y 
$$
and similarly if they choose a given $\Delta_y$, then they must have:
$$
\Delta_x = \frac{x(y+\Delta_y)}{y}-x  
$$



---

## G3M Arbitrage Math

### Trading Function with Extra Constants

We can solve for each variable in terms of the other and the invariant $k$:

First, $x$:
$$
\left(\frac{x}{w_x} \right)^{w_x} = k \left(\frac{y}{w_y}\right)^{-w_y}\\
$$

$$
\implies \boxed{x = w_x k^{1/w_x} \left(\frac{y}{w_y}\right)^{-w_y/w_x} }
$$

The work is analogous for $y$:
$$
\implies \boxed{y = w_y k^{1/w_y} \left(\frac{x}{w_x}\right)^{-w_x/w_y}}
$$

#### Getting the arbitrage calculation

##### For Lowering Price
Suppose that we need the price to move $p\mapsto p'$ with $p'<p$. 
This means we tender $x$ in the swap so $x\mapsto x+\delta_x$. 
Then using our equation for $y$, we can write
$$
\begin{align*}
p &= \frac{w_x}{w_y}\frac{ w_y k^{1/w_y} \left(\frac{x}{w_x}\right)^{-w_x/w_y}}{x}\\
&= k^{1/w_y} \left(\frac{x}{w_x}\right)^{-1-w_x/w_y}
\end{align*}
$$
Then we want $p'$ and $x\mapsto x+\delta_x$:
$$
p' =k^{1/w_y} \left(\frac{x+\delta_x}{w_x}\right)^{-1-w_x/w_y}
$$
$$
\implies \boxed{ \delta_x = w_x\left(\frac{p'}{k^{1/w_y}}\right)^{\frac{-1}{1+w_x/w_y}}-x}
$$

##### For Raising Price
Suppose that we need the price to move $p\mapsto p'$ with $p'>p$. 
This means we tender $x$ in the swap so $y\mapsto y+\delta_x$. 
Then using our equation for $x$, we can write
$$
\begin{align*}
p &= \frac{w_x}{w_y}\frac{y}{ w_x k^{1/w_x} \left(\frac{y}{w_y}\right)^{-w_y/w_x}}\\
&= k^{-1/w_x} \left(\frac{y}{w_y}\right)^{1+w_y/w_x}
\end{align*}
$$
Then we want $p'$ and $y\mapsto y+\delta_y$:
$$
p' = k^{-1/w_x} \left(\frac{y+\delta_y}{w_y}\right)^{1+w_y/w_x}
$$
$$
\implies \boxed{ \delta_y = w_y\left(p'k^{1/w_x}\right)^{\frac{1}{1+w_y/w_x}}-y}
$$

--- 

### Trading Function without Extra Constants

#### Starting work


We can solve for each variable in terms of the other and the invariant $k$:

First, $x$:
$$
x^{w_x} = k y^{-w_y}\\
$$

$$
\implies \boxed{ x = k^{1/w_x} y^{-w_y/w_x} }
$$

The work is analogous for $y$:
$$
\implies \boxed{y = k^{1/w_y} x^{-w_x/w_y}}
$$

Then we can get the price of the pool (for $x$ in terms of $y$):
$$
p = \frac{\nabla_x \psi}{\nabla_y \psi}
$$
for which:
$$
\begin{align*}
\nabla_x \psi &= w_xx^{w_x-1}y^{w_y}\\
\nabla_y \psi &= w_yx^{w_x}y^{w_y-1}
\end{align*}
$$
therefore:
$$
p  = \frac{w_x}{w_y}\frac{y}{x}
$$
which is the same as before, which we expect since the difference was the trading function was just modified by a scalar.

#### Getting the arbitrage calculation

##### For Lowering Price
Suppose that we need the price to move $p\mapsto p'$ with $p'<p$. 
This means we tender $x$ in the swap so $x\mapsto x+\delta_x$. 
Then using our equation for $y$, we can write
$$
\begin{align*}
p &= \frac{w_x}{w_y}\frac{k^{1/w_y} x^{-w_x/w_y} }{x}\\
&=  \frac{w_x}{w_y}\frac{k^{1/w_y}}{ x^{1+w_x/w_y}}
\end{align*}
$$
Then we want $p'$ and $x\mapsto x+\delta_x$:
$$
p' = \frac{w_x}{w_y}\frac{k^{1/w_y}}{ (x+\delta_x)^{1+w_x/w_y}}
$$
$$
\implies \boxed{ \delta_x = \left(\frac{p' w_y}{k^{1/w_y}w_x}\right)^{\frac{-1}{1+w_x/w_y}} -x }
$$

##### For Raising Price
Suppose that we need the price to move $p\mapsto p'$ with $p'>p$. 
This means we tender $x$ in the swap so $y\mapsto y+\delta_x$. 
Then using our equation for $x$, we can write
$$
\begin{align*}
p &= \frac{w_x}{w_y}\frac{y}{k^{1/w_x} y^{-w_y/w_x}}\\
&= \frac{w_x}{w_y}\frac{y^{1+w_y/w_x}}{k^{1/w_x}}
\end{align*}
$$
Then we want $p'$ and $y\mapsto y+\delta_y$:
$$
p' = \frac{w_x}{w_y}\frac{(y+\delta_y)^{1+w_y/w_x}}{k^{1/w_x}}
$$
$$
\implies \boxed{ \delta_y = \left(\frac{p' w_yk^{1/w_x}}{w_x}\right)^{\frac{1}{1+w_y/w_x}} -y }
$$
