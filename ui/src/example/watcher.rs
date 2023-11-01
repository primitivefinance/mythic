//! Widget for watching an event and updating the content with the data.
//! Subscribes to an event using Websocket protocol from an ethers-rs provider.
//! It's possible to define event filters to watch using abigen! macro.

use iced::{
    widget::{column, container, image, Container, Text},
    Alignment, Length,
};

use ethers::contract::{abigen, Contract};
use ethers::prelude::*;
use ethers::types::Address;
use std::sync::Arc;

use tracing::info;

abigen!(
    Counter,
    r#"[
    event Update(uint value)
    ]
    "#,
);

const COUNTER_ADDY: &str = "0x5fc8d32690cc91d4c39d9d3abcbd16989f875707";

pub struct Watcher<B, M, D> {
    event_filter: Event<B, M, D>,
}

impl Watcher<Arc<Provider<Ws>>, Provider<Ws>, UpdateFilter> {
    pub async fn new(client: Arc<Provider<Ws>>) -> eyre::Result<Self> {
        let event = Contract::event_of_type::<UpdateFilter>(client.clone())
            .from_block(1)
            .address(ValueOrArray::Array(vec![COUNTER_ADDY.parse()?]));

        let mut stream = event.subscribe_with_meta().await?.take(2);

        info!("Starting event stream watcher");

        // Note that `log` has type AnswerUpdatedFilter
        while let Some(Ok((log, meta))) = stream.next().await {
            info!("{log:?}");
            info!("{meta:?}")
        }

        info!("exited event stream watcher");

        drop(stream);

        Ok(Self {
            event_filter: event,
        })
    }

    /* pub async fn sub(&self) -> eyre::Result<()> {
        let mut stream = self.event_filter.subscribe().await?.take(2);

        while let Some(Ok((log, meta))) = stream.next().await {
            println!("{log:?}");
            println!("{meta:?}")
        }

        Ok(())
    } */
}

pub fn watcher<'a, Message: 'a>() -> Container<'a, Message> {
    // Renders the value of the watched event.
    let value = Text::new("Value")
        .size(30)
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center);

    // Renders the event name.
    let event_name = Text::new("Event Name")
        .size(30)
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center);

    let mut column = column![];

    column = column.push(event_name);
    column = column.push(value);

    container(column.spacing(10).align_items(Alignment::Center))
        .width(Length::Fill)
        .center_x()
        .into()
}
