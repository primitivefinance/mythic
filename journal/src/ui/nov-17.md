## Arbiter debugging

Verify transaction payloads before executing! I was passing empty data to a send transaction and arbiter unfortunately silently exited its main run loop without telling me about missing data.