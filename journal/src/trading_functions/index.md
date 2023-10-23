# Trading Functions

Trading functions are a specific way of doing automated market making.
Given a set of reserves and a single invariant, we can know how to compute a swap between any pair of assets.
Trading functions are completely deterministic e.g. in their price discovery properties.

Quantities we can compute:
- Price
- Price impact
    - Volume to volatility at a given price
- Liquidity

## Notation
- The **_tokens_** are given by: $$X \equiv \mathtt{token\_x}$$ $$Y \equiv \mathtt{token\_y}$$
- The **_reserves_** are: $$ x \equiv \mathtt{reserve\_x} $$ $$ y \equiv \mathtt{reserve\_y} $$
- The **_trading function_** is: $$ \varphi(x,y) \equiv \mathtt{trading\_function} $$
- The **_fee parameter_** is: $$ \gamma \equiv \mathtt{fee} $$
- The **_amount changed_** will be written as $\delta$ with subscripts and will be given by: $$ \delta \equiv \mathtt{amount\_deposit} $$ $$ \delta \equiv \mathtt{amount\_withdrawn} $$ 
we can use a sign on $\delta$ to denote whether it is a deposit or withdrawal.
    - For liquidity deposits: $$ \delta_x \equiv \mathtt{amount\_deposit\_x} $$ $$ \delta_y \equiv \mathtt{amount\_deposit\_y} $$
    - For liquidity withdraws: $$ \delta_x \equiv \mathtt{amount\_withdrawn\_x} $$ $$ \delta_y \equiv \mathtt{amount\_withdrawn\_y} $$
- The **_amount swapped_** is written as $\Delta$
- The **_pool price_** is: $$ p \equiv \mathtt{pool\_price} $$
- The **price** is: $$ S \equiv \mathtt{price} $$ This can show up for arbitrage math for example

## Computations

### Pool Price
The pool price $p$ can be computed by:
$$
p = \frac{\nabla_x \psi}{\nabla_y \psi}
$$
This assumes that $Y$ is the numeraire token, so that this is a price of $X$ in terms of $Y$.

### Liquidity Changes
When finding a valid change in liquidity, we must assert that the pool price $p$ remain invariant.
That is: $$p(x,y) = p(x+\delta_x, y+\delta_y)$$

### Swaps
When finding a valid (fee-less) swap, we must assert that the trading function remain invariant. 
That is $$\varphi(x,y) = \varphi(x+\Delta_x, y+\Delta_y)$$
This is, assuming there is no fee. 

#### Including a Reinvested Fee
If we include a fee, then we must assert that the trading function remain invariant after the fee is taken and reinvested into the pool.
The logic works this way, but when it comes to defining the equations, we can do so without having to take multiple steps.
In essence:
- The user inputs a $\Delta$ and $(1-\gamma)\Delta$ is taken as a liquidity change. 
This increases the liquidity first, and then the swap is done by taking the remaining $\gamma\Delta$ and using the equation of a swap without a fee.

So, we have, for a swap taking in $X$:
$$
\varphi(x+\Delta_x, y+\Delta_y) = \varphi(x,y)\\
\implies \Delta_y.
$$
Then we must deposit $\delta_x = (1-\gamma) \Delta_x$ and from which it must be that:
$$
p(x+\delta_x, x+\delta_y) = p(x,y)\\
\implies \delta_y.
$$
The user then receives back:
$$
\widetilde{\Delta_x} = \Delta_y - \delta_y.
$$
The new invariant value for the trading function is then:
$$
\varphi(x+\delta_x, y+\delta_y) .
$$

### Price Impact
TODO

## Interface

```rust
// Initialization
fn new(uint256 price, uint256 initial_x, uint256 initial_y, Parameters parameters) -> Pool;

// Read/getters
fn get_parameters() -> uint256;
fn get_price() -> uint256;
fn reserve_x() -> uint256;
fn reserve_y() -> uint256;

// Write/state changers
fn swap(bool input_token, uint256 amount_in) -> bool;
fn changeLiquidity(bool token, bool deposit, uint256 amount) -> bool;

// Structures
struct Parameters {
    fee: uint256,
    // ~ additional parameters
}

// High level events
event Swap(bool input_token, uint256 amount_in, uint256 amount_out);
event AddLiquidity(uint256 amount_x_in, uint256 amount_y_in);

// Logging events, used for debugging and analysis
event LogPrice(uint256 price);
```