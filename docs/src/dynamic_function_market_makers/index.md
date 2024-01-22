# Dynamic Function Market Makers
Automated Market Makers (AMMs) have been extensively studied in DeFi and usually appear in the form of Constant Function Market Makers (CFMMs).
CFMMs are a special case of a more general class of AMMs called Dynamic Function Market Makers (DFMMs).
The main difference between CFMMs and DFMMs is that CFMMs maintain a constant **invariant** or **bonding function** while DFMMs can allow for parameters of the bonding function to change over time.

## Examples
We will provide two examples of CFMMs and their DFMM counterparts. 
First is the [Geometric Mean Market Maker (GMMM)](./geometric_mean.md) and the second is the [Log Normal Market Maker (LNMM)](./log_normal.md).
We will assume that pools have reserves $r_i$ for $i=1,\dots,n$ and that the trading function is $\varphi(\boldsymbol{r})$.

### Geometric Mean Market Maker
The GMMM is a CFMM that maintains the following invariant:
$$
\begin{equation}
\varphi(\boldsymbol{r}) = \prod_{i=1}^n r_i^{w_i} - L
\end{equation}
$$
where $w_i$ are the **weights**, i.e., that $w_i$ is the weight of token $i$ and $L$ is the liquidity parameter.
We also require that $w_i \in [0,1]$ and $\sum_{i=1}^n w_i = 1$.

We consider the state of the CFMM valid if and only if:
$$
\begin{equation}
\varphi(\boldsymbol{r}) = 0.
\end{equation}
$$
Dimensional analysis tells us that $L$ has dimensions of *tokens*.

The DFMM counterpart allows for the weights be arbitrary functions of time and even pool state.
We will denote the varying weights by $w_i(t, \boldsymbol{q})$ where $\boldsymbol{q}$ is a choice of pool state, i.e.,
$$
\begin{equation}
\boldsymbol{q} = \left(r_1, \dots, r_n, w_1, \dots, w_n, L\right).
\end{equation}
$$
For simplicity, consider a pool with two tokens $X$ and $Y$ with weights $w(t)$ and $1-w(t)$ respectively. 
Then the DFMM invariant is:
$$
\begin{equation}
\varphi(x,y,t) = r_x^{w(t)} r_y^{1-w(t)} - L.
\end{equation}
$$
For sake of concreteness, we can let $t \in [0,1]$ and take $w(t) = t$ so that the weights are linearly interpolated between $X$ and $Y$.
Specifically, the pool will start out with $0$ weight on $X$ and $1$ weight on $Y$ and end with $1$ weight on $X$ and $0$ weight on $Y$.
This can be thought of as a means of **dollar cost averaging** from $Y$ into $X$ over time $t$.

### Log Normal Market Maker
The LNMM is a CFMM that maintains the following invariant:
$$
\begin{equation}
\varphi(\boldsymbol{r}) = \sum_{i=1}^n \Phi^{-1}\left( \frac{r_i}{p_i L} \right) + \sigma \sqrt{\tau} - L
\end{equation}
$$
where $\Phi^{-1}$ is the inverse of the cumulative distribution function of the standard normal distribution, $p_i$ is the **relative strike price** of token $i$, $\sigma$ is the volatility parameter, and $\tau$ is the time parameter.