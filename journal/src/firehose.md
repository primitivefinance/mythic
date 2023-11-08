# Building the firehose

The first problem is what is going into the firehose?

We want to use our logs from the tracing crate. To do this, we need to get a channel that can receive these logs.

I first attempted to use `with_default(subscriber, || {})`. This will allow a custom subscriber (i.e. with a receiving channel that we created in our app) to pickup all the traces inside that closure. However, it does not pick up traces that are made in separate threads OR tokio tasks. This is a problem because all of the sim traces are in that task.

So, I moved on to the default global subscriber. This was being set in src/main.rs, but I moved it out and instead force the specific application (run sim, run ui) to handle the global tracer.

In the ui/lib.rs, I can instantiate the global tracer and add a "layer". A Layer is an abstraction for an additional "Subscriber", which is a trait that can be implemented to track the events. We make our own channel for sending and receiving, make a subscriber, and add it as a layer before we create our "global" tracer. The global tracer works cross thread! Then we just need to pipe the receiver channel into our app and into the component that needs it: Firehose.