# Portfolio in a box

Construct, implement, and model a portfolio strategy.

## Installation

```bash
./build.sh

cargo run
```

## Run tests

```bash
cargo test --workspace
```

## For book
You may have to install `cargo-make` and `mdbook`, but they're in our workspace's Cargo file so I think it will be okay. Anyway, to compile the book run:
```bash
cargo make journal
```
then you can run
```bash
cargo make journal-serve
```
to be able to view the book in your browser at `http://localhost:3000`

## Crates
- Core - Abstractions for portfolio management in rust
- Simulation - Simulation loop, data logging, and agents
- src/main.rs - Binary entrypoint for this repo
- contracts/ - Smart contracts for portfolio management