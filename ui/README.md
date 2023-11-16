# Excalibur Ui

Excalibur uses [iced](https://github.com/iced-rs/iced) to deliver a rust GUI over the core excalibur software.

Get familiar with [Elm Architecture](https://guide.elm-lang.org/architecture/) and start buildings [components](./src/components/).

## Running a UI

```bash
cargo run ui
cargo run ui example
cargo run ui analyzer
cargo run ui <application name>
```

## Testing

```bash
cargo test --package ui

# for verbose testing
RUST_TRACING_LEVEL="trace" cargo test --package ui
```

## MVP Architecture
- [./src/](./src/)
    - [lib.rs](./src/lib.rs) - Exports the "apps" inside the ui directory.
        -  [mvp/](./src/mvp/mod.rs) - Main application. Implements `iced::Application` with different application states for installing, loading, and running the app.
            - [api/](./src/mvp/api/) - Handles logic to talk to external crates or services (i.e. simulation, transacting, etc.)
            - [components/](./src/mvp/components/) - Pure UI legoes.
            - [view/](./src/mvp/view/) - Handles UI rendering for different screens.
            - [screens/](./src/mvp/screens/) - Core logic of the application's screens/pages.
            - [loader.rs](./src/mvp/loader.rs) - Transient application that runs before the main application runtime begins.
            - [app.rs](./src/mvp/app.rs) - Main application runtime, after loading app stage completes.
            - [tracer.rs](./src/mvp/tracer.rs) - Adds stack traces to application that can be exported to files for debugging.
