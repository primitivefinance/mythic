# Arbitrageur

## Arbitrageur Agent

The main idea for designing the arbitrageur agent is to give them a set of exchanges to monitor and a set of actions to take. 
In the simplest case, we will have two exchanges.
The agent will then monitor the exchanges and take actions when the price of a exchange deviates sufficiently from the price of the other exchange.

### Necessary Attributes

The arbitrageur will need to be a structure that contains the following bits of data:
- The attributes of the exchanges.
- A client connection to each of the exchanges.

We will also give the arbitrageur a `atomic_arbitrage` contract so that they can use this to execute atomic arbitrages.

### Necessary Methods

The arbitrageur will need to have the following methods:
- A method to watch/update the prices of the exchanges.
- A method to check for arbitrage opportunities.
- A method to execute arbitrage opportunities.
- Methods to compute the trade sizes to take.}

## Other Notes

The arbitrageur could be ran asynchronously, but if there are not many agents in the simulation, it can be possible to iterate through the agents and have them take turns executing their actions.

## Arbitrageur Agent

The main idea for designing the arbitrageur agent is to give them a set of exchanges to monitor and a set of actions to take. 
In the simplest case, we will have two exchanges.
The agent will then monitor the exchanges and take actions when the price of a exchange deviates sufficiently from the price of the other exchange.

### Necessary Attributes

The arbitrageur will need to be a structure that contains the following bits of data:
- The attributes of the exchanges.
- A client connection to each of the exchanges.

We will also give the arbitrageur a `atomic_arbitrage` contract so that they can use this to execute atomic arbitrages.

### Necessary Methods

The arbitrageur will need to have the following methods:
- A method to watch/update the prices of the exchanges.
- A method to check for arbitrage opportunities.
- A method to execute arbitrage opportunities.
- Methods to compute the trade sizes to take.}

## Other Notes

The arbitrageur could be ran asynchronously, but if there are not many agents in the simulation, it can be possible to iterate through the agents and have them take turns executing their actions.
