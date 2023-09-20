# G3M Arbitrage Math

## Starting work
The trading function is defined by:3
$$
\psi(x,y) = \left(\frac{x}{w_x}\right)^{w_x} \left( \frac{y}{w_y} \right)^{w_y} = k
$$
It also must be that $w_x+w_y = 1$.

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

---

## Getting the arbitrage calculation

### For Lowering Price
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
\implies \boxed{ \delta_x = w_x\left(\frac{p'}{k^{1/w_y}}\right)^{1+w_x/w_y}-x}
$$

### For Raising Price
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
\implies \boxed{ \delta_y = w_y\left(p'k^{1/w_x}\right)^{-1-w_y/w_x}-y}
$$