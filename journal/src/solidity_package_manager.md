# Solidity Package Manager

What we need:
- a package manager that a user can submit packages to
- anyone (including us) can download and install that package into our projects.
- A package should have three assets in it:
  - The contract source (sol files)
  - Bindings for the contract (rs files)
  - Contract ABI (json artifacts)

## Design approach 

Build a simple Rust CLI that has two commands:
- `spm package` - compiles the solidity contract and generates the bindings and artifacts into a "package" folder
- `spm publish` - publishes the package to a cargo registry

I have double checked that you can publish non-rust files as part of libs to a cargo registry, you just have to specify them in the `Cargo.toml` file of the package. 

Step one, figure out the right way to build the package so that it can be published to a cargo registry.
step two, automate the process of building the package with spm package
step three, automate the process of publishing the package with spm publish
step four celebrate

Time to plan this out: 27 minutes