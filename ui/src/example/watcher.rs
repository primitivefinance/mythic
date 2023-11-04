//! Widget for watching an event and updating the content with the data.
//! Subscribes to an event using Websocket protocol from an ethers-rs provider.
//! It's possible to define event filters to watch using abigen! macro.

use std::sync::Arc;

use ethers::{
    contract::{abigen, Contract},
    prelude::*,
};
use iced::{
    alignment::{self},
    widget::{button, column, text},
    Alignment, Element, Length,
};
use tokio_util::sync::CancellationToken;
use tracing::info;

abigen!(
    Counter,
    r#"[
    event Update(uint value)
    ]
    "#,
);

/// Need to deploy it to anvil first...
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
        // In this code, the tokio::select! block will either wait for the cancellation
        // token to be triggered or for the event stream to complete.
        // If the cancellation token is triggered,
        // it will log a message and then return,
        // effectively stopping the task.
        // If the event stream completes,
        // it will log a message and continue running.
        let _handle = tokio::spawn(async move {
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

#[derive(Debug, Clone)]
pub enum WatcherToAppMessage {
    Toggle(bool),
}

#[derive(Debug, Clone)]
pub enum AppToWatcherMessage {
    ToggleWatcher(bool),
    SetWatcher(Option<CancellationToken>),
    AbortWatcher,
}

#[derive(Debug, Clone)]
pub struct WatcherComponent {
    pub status: bool,
    pub handle: Option<CancellationToken>,
}

/// Implements the UI component for the watcher.
impl WatcherComponent {
    pub fn new() -> Self {
        Self {
            status: false,
            handle: None,
        }
    }

    pub fn update(&mut self, message: AppToWatcherMessage) -> Option<WatcherToAppMessage> {
        match message {
            AppToWatcherMessage::ToggleWatcher(state) => {
                info!("Got watcher message: {:?}", state);
                self.status = state;

                // Tell the application to handle the resulting toggle state.
                Some(WatcherToAppMessage::Toggle(state))
            }
            AppToWatcherMessage::SetWatcher(handle) => {
                info!("Got watcher update handle");

                // Set the handler
                self.handle = handle;

                None
            }
            AppToWatcherMessage::AbortWatcher => {
                info!("Got watcher abort");
                match self.handle {
                    Some(ref handle) => {
                        info!("Cancelling watcher using token");
                        handle.cancel();
                    }
                    None => {
                        info!("No watcher online to abort.");
                    }
                }

                None
            }
        }
    }

    /// Returns AppToWatcherMessage to the application, which passes it back
    /// this Watcher component.
    pub fn view<'a>(&self) -> Element<'a, AppToWatcherMessage> {
        let mut content = column![];

        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .height(40)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .on_press(on_press)
        };

        // Render the watcher buttons
        content = content.push(button(
            "Turn watcher ON",
            AppToWatcherMessage::ToggleWatcher(true),
        ));
        content = content.push(button(
            "Turn watcher OFF",
            AppToWatcherMessage::ToggleWatcher(false),
        ));

        content.spacing(10).align_items(Alignment::Center).into()
    }
}
