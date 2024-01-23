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
- These parameters must satisfy 
$$
w_x, w_y \geq 0 \\
w_x+w_y=1 
$$

Next, we define the trading function to be:
$$
\varphi(x,y) = x^{w_x} y^{w_y} = L
$$
where $L$ is the invariant of the pool. 
We can put:
$$
L \equiv \mathtt{liquidity}
$$
Note that $L$ is in units of Token by virtue of the geometric mean.

## Price
If we compute the derivatives and simplify the expression, we get that the pool price is:
$$
\boxed{P = \frac{w_x}{w_y}\frac{y}{x}}
$$
We can determine a price in terms of just $x$ or just $y$ if need be.

## Initializing Pool
We need to initalize a pool from a given price $p$ and an amount of a token. We can also do it by specifying liquidity too.

### Given x and price

Noting that 
$$
y= \frac{w_y}{w_x}p x
$$
we can get
$$
\begin{equation}
\boxed{L_X(x,S) = x\left(\frac{w_y}{w_x}S\right)^{w_y}}
\end{equation}
$$
This is a linear function in $x$:
$$
L_X(x+a\delta_x) = L_X(x) + aL_X(\delta_X)
$$
We can get now the amount of $Y$ needed from $L$ and $x$ using the trading function and note:
$$
\boxed{y(x,L;w_x) = \left(\frac{L}{x^{w_x}}\right)^{1/w_y}}
$$

### Given y and price
Noting that
$$
x = \frac{w_x}{w_y}\frac{1}{p}y
$$
we can get
$$
\begin{equation}
\boxed{L_Y(y,S) = y\left(\frac{w_x}{w_y}\frac{1}{S}\right)^{w_x}}
\end{equation}
$$
We can get now the amount of $X$ needed from $L$ and $y$ using the trading function and note:
$$
\boxed{x(y,L;w_y) = \left(\frac{L}{y^{w_y}}\right)^{1/w_x}}
$$


## Swap 

We require that the trading function remain invariant when a swap is applied, that is:
$$
L(x,y) = (x+\Delta_x)^{w_x}(y+\Delta_y)^{w_y}
$$
while also taking fees as a liquidity deposit (which will increase the liquidity $L$).

### Trade in $\Delta_X$ for $\Delta_Y$
Suppose that we want to trade in $\Delta_X$ for $\Delta_Y$. 
Then we have that we are really inputting $\gamma\Delta_X$ while raising $L\mapsto L+\delta_L$.
From Equation (1) we get that:
$$
x = \frac{L}{\left(\frac{w_y}{w_x}S\right))^{w_y}}
$$
and note that $L_X(x,p)$ is linear in $x$.
Then we have that:
$$
L_X(x+\delta_x) = L_X(x) + \delta_L \\= L_X(x) + \delta_X(\frac{w_y}{w_x}p)^{w_y}
$$
so 
$$
\boxed{\delta_{L_X} = \delta_X\left(\frac{w_y}{w_x}p\right)^{w_y}}
$$
TODO: CAN REWRITE THIS WITHOUT PRICE

Hence we have for a swap with fees that (note $\Delta$ are what users input and receive):
$$
L+\delta_L = (x+\gamma \Delta_X)^{w_x}(y+\Delta_y)^{w_y}
$$
Then:
$$
\boxed{\Delta_Y(\Delta_X) = \left(\frac{L+\delta_{L_X}}{(x+\Delta_X)^{w_x}}\right)^{1/w_y}-y}
$$

### Trade in $\Delta_Y$ for $\Delta_X$
We can get the
$$
x = \frac{y}{p}\frac{w_x}{w_y}
$$
We have the linear function:
$$
\boxed{L_Y(y,S) = y\left(\frac{w_x}{w_y}\frac{1}{S}\right)^{w_x}}
$$
so that:
$$
\boxed{\delta_{L_Y} = \delta_Y\left(\frac{w_x}{w_y}\frac{1}{p}\right)^{w_x}}
$$

Then
$$
\boxed{\Delta_X(\Delta_Y) = \left(\frac{L+\delta_{L_Y}}{(y+\Delta_Y)^{w_y}}\right)^{1/w_x}-x}
$$


## Liquidity Provision
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

## Arbitrage Math
We can solve for each variable in terms of the other and the invariant $k$:
$$
x^{w_x}y^{w_y} = k
$$

First, $x$:
$$
\implies \boxed{x = \left(\frac{L}{y^{w_y}}\right)^{1/w_x} }
$$

The work is analogous for $y$:
$$
\implies \boxed{y = \left(\frac{L}{x^{w_x}}\right)^{1/w_y}}
$$

### Lowering Price
Suppose that we need the price to move $p\mapsto p'$ with $p'<p$. 
This means we tender $x$ in the swap so $x\mapsto x+\delta_x$. 
Then we want $p'$ and $x\mapsto x+\delta_x$:
$$
p(x+\Delta_X,y+\Delta_Y) = \frac{w_x}{w_y}\frac{y+\Delta_Y}{x+\Delta_X}
$$
Now we want to do this all for a given $p'$ and only with $X$.
Note that
$$
\Delta_Y(\Delta_X) = \left(\frac{L+\delta_L}{(x+\Delta_X)^{w_x}}\right)^{1/w_y}-y
$$
Then using this:
$$
x = \frac{L}{(\frac{w_y}{w_x}p)^{w_y}}
$$
we can do
$$
p' = \frac{w_x}{w_y}\frac{\left(\frac{L+\delta_L}{(x+\gamma \Delta_X)^{w_x}}\right)^{1/w_y}}{x+\gamma\Delta_X}\\
(x+\gamma\Delta_X)^{1+w_x/w_y}=\frac{w_x}{p'w_y}(L+(1-\gamma)\Delta_X\left(\frac{w_y}{w_x}p\right)^{w_y})^{w_x}\\
= \frac{1}{p'}\frac{w_x}{w_y}\left(\frac{w_y}{w_x}p\right)^{w_y}(x+(1-\gamma)\Delta_X)^{w_x}\\
\implies (x+\Delta_x)^{1+w_x/w_y-w_x} = \frac{1}{p'}\frac{w_x}{w_y}\left(\frac{w_y}{w_x}p\right)^{w_y}\\
\boxed{\Delta_x = \frac{1}{\gamma}\left(\left(L\frac{w_x}{w_y}\frac{1}{p'x}\right)^{\frac{1}{1+w_x/w_y-w_x}}-x\right)}
$$

TRY AGAIN:
$$
\Delta_x = \frac{1}{\gamma}\left(L \left( \frac{w_x}{pw_y}\right)^{w_y}+(1-\gamma) \Delta_x  \right)\\
\Delta_x + \frac{\gamma-1}{\gamma}\Delta_x = \frac{1}{\gamma}L \left( \frac{w_x}{pw_y}\right)^{w_y}\\
\implies \boxed{\Delta_x = \frac{1}{\gamma}\left(L \left( \frac{w_x}{pw_y}\right)^{w_y}-x\right)}
$$

### Raising Price
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

## Value Function via $L$ and $S$
Given that we treat $Y$ as the numeraire, we know that the portfolio value of a pool when $X$ is at price $S$ is:
$$
V(x,y,S) = x S + y
$$
We can find the relationship to portfolio value from $V(L,S)$. 
This will be helpful when tokenizing pool LP positions. 

Since we have $L_X(x, S)$ and $L_Y(y, S)$, we can get the following:
$$
x = \frac{L}{(\frac{w_y}{w_x}S)^{w_y}}\\
y = \frac{\left(\frac{w_x}{w_y}\frac{1}{S}\right)^{w_x}}{L}
$$
Therefore:
$$
V(L,S) = \frac{LS}{\left(\frac{w_y}{w_x}S\right)^{w_y}} + \frac{L}{\left(\frac{w_x}{w_y}\frac{1}{S}\right)^{w_x}}\\
\boxed{V(L,S)=LS^{w_x}\left(\left( \frac{w_x}{w_y}\right)^{w_y}+\left( \frac{w_y}{w_x}\right)^{w_x}\right)}
$$
Note that $V$ is linear in $L$ and so we can use this to tokenize. 
