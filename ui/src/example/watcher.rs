//! Widget for watching an event and updating the content with the data.
//! Subscribes to an event using Websocket protocol from an ethers-rs provider.
//! It's possible to define event filters to watch using abigen! macro.

use iced::{
    widget::{column, container, image, Container, Text},
    Alignment, Length,
};

use ethers::contract::{abigen, Contract};
use ethers::prelude::*;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use tracing::info;

abigen!(
    Counter,
    r#"[
    event Update(uint value)
    ]
    "#,
);

const COUNTER_ADDY: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

#[derive(Debug)]
pub struct Watcher {
    pub handle: CancellationToken,
}

impl Watcher {
    pub async fn new(client: Arc<Provider<Ws>>) -> eyre::Result<Self> {
        let cancel_token = CancellationToken::new();
        let cloned_token = cancel_token.clone();

        let event = Contract::event_of_type::<UpdateFilter>(client.clone())
            .from_block(1)
            .address(ValueOrArray::Array(vec![COUNTER_ADDY.parse()?]));

        info!("Starting event stream watcher");

        // Note that `log` has type AnswerUpdatedFilter
        // In this code, the tokio::select! block will either wait for the cancellation token to be triggered or for the event stream to complete.
        // If the cancellation token is triggered,
        // it will log a message and then return,
        // effectively stopping the task.
        // If the event stream completes,
        // it will log a message and continue running.
        let handle = tokio::spawn(async move {
            tokio::select! {
                _ = cloned_token.cancelled() => {
                    info!("Cancellation token triggered");
                    return;
                }
                _ = async {
                    let mut set = tokio::task::JoinSet::new();
                    set.spawn(async move {
                        let mut stream = event.subscribe_with_meta().await.unwrap();
                        while let Some(Ok((log, meta))) = stream.next().await {
                            info!("{log:?}");
                            info!("{meta:?}")
                        }
                    });

                    while let Some(res) = set.join_next().await {
                        info!("stream watcher completed: {:?}", res);
                    }
                } => {}
            }
        });

        info!("Exited event stream watcher");

        Ok(Self {
            handle: cancel_token.clone(),
        })
    }
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
