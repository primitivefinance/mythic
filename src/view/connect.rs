//! For connecting the clients to an RPC.

use anyhow::Result;
use iced::widget::{button, text_input, Column, Row};
use iced::{Center, Element, Fill, Task};
use std::sync::Arc;

use super::blockchain::AlloyClient;
use super::{AppMessage, ViewMessage};
use crate::components::styles::Sizes;
use crate::components::system::{label, ExcaliburContainer};
use crate::pages::MessageWrapper;
use crate::view::RootMessage;

#[derive(Default, Clone, Debug)]
pub enum Message {
    #[default]
    Empty,

    ChangeRpcUrl(Option<String>),

    Connect,

    UpdateClient(AlloyClient),
}

#[derive(Clone)]
pub struct Connect {
    rpc_url: Option<String>,
    pub client: Arc<AlloyClient>,
}

impl Default for Connect {
    fn default() -> Self {
        Self {
            rpc_url: None,
            client: Arc::new(AlloyClient::default()),
        }
    }
}

pub async fn refresh_client_connection(rpc_url: String) -> Result<AlloyClient, anyhow::Error> {
    AlloyClient::default().connect(&rpc_url).await
}

impl Connect {
    pub fn new(client: Arc<AlloyClient>) -> Self {
        Self {
            rpc_url: None,
            client: client.clone(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<AppMessage> {
        match message {
            Message::ChangeRpcUrl(url) => {
                self.rpc_url = url;

                Task::none()
            }
            Message::Connect => {
                tracing::debug!("Switching from RPC: {:?}", self.rpc_url);

                let rpc_url = match self.rpc_url.clone() {
                    Some(url) => url.clone(),
                    None => return Task::none(),
                };

                tracing::debug!("Connecting to RPC: {}", rpc_url);

                Task::perform(
                    refresh_client_connection(rpc_url.clone()),
                    move |r| match r {
                        Ok(client) => Message::UpdateClient(client),
                        Err(e) => {
                            tracing::error!("Failed to connect to RPC: {}", e);

                            Message::Empty.into()
                        }
                    },
                )
                .map(|x| x.into())
            }
            Message::UpdateClient(r) => {
                tracing::debug!("Updated client: {:?}", r.provider.is_some());
                self.client = Arc::new(r);

                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, ViewMessage> {
        let row_container = Row::new()
            .push(
                Column::new()
                    .width(iced::Length::FillPortion(5))
                    .align_x(iced::Alignment::End)
                    .push(
                        Row::new()
                            .push(Row::new().width(iced::Length::FillPortion(3)))
                            .push(
                                Row::new()
                                    .width(iced::Length::FillPortion(1))
                                    .push(
                                        text_input(
                                            &"rpc",
                                            self.rpc_url
                                                .as_deref()
                                                .unwrap_or("ws://localhost:8545"),
                                        )
                                        .on_input(|url| Message::ChangeRpcUrl(Some(url)).into()),
                                    )
                                    .push(
                                        button(label("Connect"))
                                            .style(button::primary)
                                            .on_press(Message::Connect.into()),
                                    )
                                    .align_y(Center)
                                    .spacing(Sizes::Sm),
                            )
                            .align_y(Center)
                            .spacing(Sizes::Md),
                    ),
            )
            .align_y(Center);

        ExcaliburContainer::default()
            .middle_top()
            .sharp()
            .build(row_container)
            .padding(iced::Padding {
                top: Sizes::Sm.into(),
                right: Sizes::Md.into(),
                bottom: Sizes::Sm.into(),
                left: Sizes::Md.into(),
            })
            .height(super::header::TITLE_BAR_HEIGHT)
            .width(Fill)
            .into()
    }
}

impl MessageWrapper for Message {
    type ParentMessage = ViewMessage;
}

impl From<Message> for ViewMessage {
    fn from(message: Message) -> Self {
        ViewMessage::Root(RootMessage::Connect(message))
    }
}

impl From<Message> for AppMessage {
    fn from(message: Message) -> Self {
        AppMessage::View(ViewMessage::Root(RootMessage::Connect(message)))
    }
}
