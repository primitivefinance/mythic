## nasty bugs

Spent a lot of time debugging a stack overflow bug in the app. It was because I was performing a command that was returning a Message::Empty, which had `.into()` called on it. This somehow caused infinite recursion, leading to the stack overflow. Fixing it required me to explicitly type it as view::Message::Empty, then do `into()`.


## performance
- FPS counter to see if we get drops in fps
- Intermittent "pulse check" to see how the ui's running, probably in a trace.