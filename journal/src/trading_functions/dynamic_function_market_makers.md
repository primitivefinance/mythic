# Dynamic Function Market Makers

We want to introduce the notion of Dynamic Function Market Makers (DFMMs). 
These are generalizations of the constant function market makers (CFMMs) we have seen. 
The idea is that the trading function of the CFMM is not constant but rather a function of some variable. 
For example, what if a trading function had additional parameters that vary according to internal or external components? 
You can think of this mathematically by introducing new variables to the trading function. 
For example, if we have a CFMM with trading function $\varphi(x,y)$, we can introduce a new variable $w$ and define a new trading function $\varphi(x,y,w)$. This is a DFMM in that another property dynamically changes the trading function. 

> For example, if the [**geometric mean**](geometric_mean.md) is the trading function, what would the behavior of this trading function be if the weights changed over time rather than stayed static? What if we wanted the weights of only one asset to be changed? Then we can introduce a new variable $w$ for consequences and define a new trading function $\varphi(x,y,w) = x^w y^z$. Here, we are varying over the variable $x$ but not $y$ if we consider $z$ to be constant. 

>Another example of the trading function we have been considering is the replicating market maker that replicates the payoff of a covered call. This trading function is $\varphi(x,y) = x - k\Phi(\Phi^{-1}(1-y) - \sigma \sqrt{\tau})$. Here the strike price is normally considered constant for the duration of the pool, but the function varies to maturity $\tau$. In this context, the trading function changes over time.