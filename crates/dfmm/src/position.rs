use futures::stream::Stream;



pub trait Position<E> {
    type EventStream: Stream<Item = E>;
    fn stream_value(&self, event: E) -> f64;
}