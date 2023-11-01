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

## Structure
Files and directories:
- lib.rs - Exposes the `run` function to start running an application.
- sdk/ - Abstractions over arbiter + contracts that are used by iced, does not use iced.
Applications, yes, we can have multiple applications. Check the [example app](./src/app/example.rs).
- example/ - The example application including its `Screen`s and `Component`s.
- analyzer/ - The analyzer application including its `Screen`s and `Component`s.


## How to make a screen

A screen is a dedicated view into some part of the application. It can encapsulate an entire screen/page/view or just be a section of the overall layout.

It's architected to communicate directly with the root `Application`. The `Application` trait in iced is capable of processing async functions via `Command::perform()`. This forces our hand with how to execute api calls with arbiter.

Individual screens can be made that forward messages to the `Application`, which is processed in `Application::update()`, which calls `Screen::update()`. Logic is handled in the Application update, with the Screen update simply reacting to the Application's updates. 

## How to make a Component

A component is specifically implemented as an iced Element. An iced Element can be easily rendered in the application's view function.

Check out the [example component](./src/components/example.rs) for reference.

A component needs to:
- Implement the possible Messages that can be sent, this should be a generic type.
- Implement the iced Renderer.
- Implement the `from` function for an iced Element, so the component can be casted to the Element type.

Components can pass messages to their parent, making it easier for their parent to react to component changes. This is why components should have a generic message type that is implemented.

## How to add the component to the application UI

Components can be rendered by adding the converted iced Element into the container that is rendered in the main application's `view` function.


## Notes and todo

Components are like mini applications, they are designed just like the main application that is running just with a smaller amount of overhead.

Next tasks are to spec out the rough "model" of the application and its children components, then implement them.

Another idea is to host an api in the background that can be `curl` into from the application, i.e. a backend. This might be more work than just integrating direct calls into arbiter with iced components.