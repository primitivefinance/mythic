# Overview

1. Tracer: This is a struct that holds a sender and receiver for a channel. This channel is used to send and receive AppEventLog instances, which are structured representations of tracing events.

2. LayerWithChannel: This is a custom tracing layer that sends AppEventLog instances over the channel in the Tracer. It does this by implementing the on_event method, which is called whenever a tracing event occurs.

3. AppEventLog: This is a struct that represents a structured version of a tracing event. It implements the Visit trait, which allows it to extract fields from a tracing event.

4. TraceConfigBuilder: This is a builder for creating a tracing layer. It allows you to specify a log target (stdout, stderr, or a file) and any additional layers you want to add to the tracing subscriber.

5. setup and setup_with_channel: These are helper functions for setting up tracing. setup sets up basic tracing to stdout, while setup_with_channel sets up tracing with a Tracer, allowing you to send and receive AppEventLog instances over a channel.

6. Tests: There are several tests that verify the functionality of the tracer. These tests create tracing events and verify that they can be received and processed correctly.

The overall architecture is designed to allow structured logging of tracing events. When a tracing event occurs, it's structured into an AppEventLog and sent over a channel. This allows you to process tracing events in a structured way, rather than dealing with raw log messages.


## How it gets structured

1. Span Creation: When a new span is created (using tracing::info_span!, for example), the on_new_span method of the LayerWithChannel is called. This method receives the attributes of the new span, which include its fields.

2. Storing Span Fields: In the on_new_span method, the fields of the span are visited by an instance of AppEventLog. This visitor extracts the fields and stores them in a thread-local storage (CURRENT_SPAN_FIELDS). This allows the fields to be associated with the current thread and accessible throughout the lifespan of the span.

3. Event Handling: When a tracing event occurs within a span, the on_event method of the LayerWithChannel is called. This method receives the event and the context, which includes the current span.

4. Event Structuring: In the on_event method, an AppEventLog is created to visit the fields of the event. This visitor combines the fields of the event with the fields of the current span (retrieved from the thread-local storage) to create a structured representation of the event. This structured representation includes both the event-specific fields and the span-specific fields.

5. Sending Structured Data: The structured representation of the event (an AppEventLog instance) is then sent over the channel in the Tracer. This allows the structured data to be received and processed elsewhere in the application.

This architecture allows span-specific fields to be associated with all events that occur within a span, even if those fields are not explicitly included in each event. This is useful for adding context to events without having to repeat the same information in each event.