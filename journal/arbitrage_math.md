# G3M Arbitrage Math

## Starting work
The trading function is defined by:
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
\nabla_x \psi &= x^{w_x-1}\left(\frac{y}{w_y}\right)^{w_y} = x^{-w_y}\left(\frac{y}{w_y}\right)^{w_y}\\
\nabla_y \psi &= \left(\frac{x}{w_x}\right)^{w_x}y^{w_y-1} = \left(\frac{x}{w_x}\right)^{w_x}y^{-w_x}
\end{align*}
$$
therefore:
$$
p = \frac{y^{w_x}}{x^{w_y}}\left(\frac{x}{w_x}\right)^{-w_x}\left(\frac{y}{w_y}\right)^{w_y} = \frac{w_x^{w_x}}{w_y^{w_y}}\frac{y}{x}
$$

---

## Getting the arbitrage calculation

### For Lowering Price
Suppose that we need the price to move $p\mapsto p'$ with $p'<p$. This means we tender $x$ in the swap so $x\mapsto x+\delta_x$. Then using our equation for $y$, we can write
$$
\begin{align*}
p &= \frac{w_x^{w_x}}{w_y^{w_y}}\frac{w_y k^{1/w_y} \left(\frac{x}{w_x}\right)^{-w_x/w_y}}{x}\\
&= \frac{w_x^{w_x}}{w_y^{w_y}}w_yw_x^{w_x/w_y}k^{1/w_y}x^{-(1+w_x/w_y)}\\
&= w_x^{w_x+w_x/w_y}w_y^{w_x}k^{1/w_y}x^{-(1+w_x/w_y)}
\end{align*}
$$
Then we want $p'$ and $x\mapsto x+\delta_x$:
$$
p' = w_x^{w_x+w_x/w_y}w_y^{w_x}k^{1/w_y}(x+\delta_x)^{-(1+w_x/w_y)}
$$
$$
\implies \boxed{ \delta_x =\left(\frac{p'}{w_x^{w_x+w_x/w_y}w_y^{w_x}k^{1/w_y}}\right)^{-\frac{1}{1+w_x/w_y}}-x }
$$

### For Raising Price
Suppose that we need the price to move $p\mapsto p'$ with $p'>p$. This means we tender $x$ in the swap so $y\mapsto y+\delta_x$. Then using our equation for $x$, we can write
$$
\begin{align*}
p &= \frac{w_x^{w_x}}{w_y^{w_y}}\frac{y}{w_x k^{1/w_x} \left(\frac{y}{w_y}\right)^{-w_y/w_x}}\\
&= w_x^{-w_y} w_y^{w_y/w_x - w_y}k^{1/w_x}y^{1-w_y/w_x}
\end{align*}
$$
Then we want $p'$ and $y\mapsto y+\delta_y$:
$$
p' = w_x^{-w_y} w_y^{w_y/w_x - w_y}k^{1/w_x}(y+\delta_y)^{1-w_y/w_x}
$$
$$
\implies \boxed{ \delta_y = \left(\frac{p'}{w_x^{-w_y} w_y^{w_y/w_x - w_y}k^{1/w_x}}\right)^{-\frac{1}{1-w_y/w_x}}-y }
$$