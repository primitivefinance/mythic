# dejanking

- Faster anvil loading or load in background.
- User profile migration. maybe just version them?
- Initial states should be loaded until the real state is loaded and overwrites it.
- Responsiveness between user action -> result is not clear / fast. Main example is pending transaction indicator + receiving the tx in the history table.
- Positions are still not properly separated/combined. Not clear that my x/y tokens in allocated are a "combo".
- Failure cases and testing is not documented or potentially caught, especially important in update model.
- Components are separated, i.e. protocol / dev clients.
- Not clear where the client ends and the protocol starts and vice versa
- nightly rust features
- logging is bad + doesnt save to file.
- dedicated bindings?
- propagating model changes to child components.