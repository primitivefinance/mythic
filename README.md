# Excalibur

Construct, implement, and model a portfolio strategy.

## Installation
To install from source you can run

```bash
cargo install --path .
```
Then you can see the help menu with
```bash
excalibur --help
```

To use the binary without installation you can run
```bash
cargo run -- --help
```
and then 
```bash
cargo run -- <args>
```

## Run tests

```bash
cargo test --workspace
```

## For book
You may have to install `cargo-make`, `mdbook`, and `mdbook-katex`, but they're in our workspace's Cargo file so it may be okay. Anyway, to compile the book run:
```bash
cargo make journal
```
then you can run
```bash
cargo make journal-view
```
to be able to view the book in your browser at `http://localhost:3000`

## Crates
- Core - Abstractions for portfolio management in rust
- Simulation - Simulation loop, data logging, and agents
- src/main.rs - Binary entrypoint for this repo
- contracts/ - Smart contracts for portfolio management
