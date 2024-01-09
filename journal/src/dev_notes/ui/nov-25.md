# bugs

Found a bug where if there are duplicate entries in the address book (i.e., same key with different labels), it will break the execution flow when trying to find the artifact, because it uses the label as the path to find the contract's artifact.


# architecture

I need to make a diagram of this, but here's a rough guide:

We start with the base "package" of components that make up every section of the app. This is like the primitive lego brick that the app is constructed with. Each primitive has:
- Message - An enum of variants to communicate to the component with.
- view() - A function to render the components on the screen.
- update() - A function to mutate the state of the component.


This begins at the root `iced: :Application` all the way down to the last child. This happens recursively, so app -> update -> child -> update -> ...nth child.

In each child component, a Parent Message is defined for developers to easily understand its relationship in the context of its package. Then, the child's Message can implement `From<ChildMessage> for ParentMessage`. Implementing this allows the parent to "wrap" the message with its own message type, thereby completing the recursion.


# sidebar

I worked on the sidebar. It started out very basic: an enum that is referenced from the application state. Then the enum variants just need to define their own "button links" with their icons + message to send, and that's it. Pass the reference enum to the app_layout and you have a sidebar!

I made it more complicated. I made an actual Sidebar struct that implements our `State` trait, so it has its own view. It then has its own state of the original enum `Page`, as `page`, which has its own view function that is called, which is now just the original logic that was in before the sidebar. 

I also added a `bookmarks` field to the Sidebar state. This way, we can update the bookmarks in the application state, and then pass the reference of the `Sidebar` to the `app_layout`, which calls `view()` as usual.

This lets us keep some state in the sidebar, which I think make sense. However, I don't know if making this component more complex could have some negative side effects down the road, because its a stateful thingy instead of a ref to an enum variant.

Overall, looks good though!