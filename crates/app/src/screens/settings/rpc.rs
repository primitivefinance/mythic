//! Rpcs can be selected and used for different underlying blockchain calls.
//! These rpcs are used within the app.

use std::collections::HashMap;

use super::*;
use crate::{
    components::{
        system::{label, Card, ExcaliburButton},
        tables::{builder::TableBuilder, cells, columns::ColumnBuilder, rows::RowBuilder},
    },
    user::networks::{ChainPacket, RPCList},
};

#[derive(Debug, Clone)]
pub enum Feedback {
    Success(String),
    Error(String),
}

impl From<anyhow::Error> for Feedback {
    fn from(error: anyhow::Error) -> Self {
        Self::Error(error.to_string())
    }
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    AddRpc,
    ChangeName(Option<String>),
    ChangeChainId(Option<String>),
    ChangeUrl(Option<String>),
    SelectedRPC(bool, Option<String>),
    Sync(RPCList),
    Feedback(Feedback),
    Delete,
    Submit,
    Reset,
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Rpc(message)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    pub name: Option<String>,
    pub chain_id: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RpcManagement {
    pub storage: RPCList,
    pub chain_packet: Option<Form>,
    pub selected_rpcs: HashMap<String, bool>,
    pub form_feedback: Option<Feedback>,
}

impl RpcManagement {
    pub fn new(storage: RPCList) -> Self {
        Self {
            storage,
            chain_packet: None,
            selected_rpcs: HashMap::new(),
            form_feedback: None,
        }
    }

    pub fn reset(&mut self) {
        self.chain_packet = None;
        self.selected_rpcs.clear();
        self.form_feedback = None;
    }

    /// Converts the form input into a chain packet, if the form is valid.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn get_chain_packet(&self) -> anyhow::Result<ChainPacket, anyhow::Error> {
        if let Some(chain_packet) = &self.chain_packet {
            let name = chain_packet.name.clone();
            let chain_id = chain_packet.chain_id.clone();
            let url = chain_packet.url.clone();

            if let (Some(name), Some(chain_id), Some(url)) = (name, chain_id, url) {
                let chain_id = chain_id.parse::<u64>().map_err(|_| {
                    anyhow::anyhow!("Chain ID must be a number!").context("Chain ID error")
                })?;
                let chain_packet = ChainPacket {
                    name,
                    chain_id,
                    url,
                };
                return Ok(chain_packet);
            }
        }

        Err(anyhow::anyhow!("No form fields updated!").into())
    }

    pub fn view_rpcs(&self) -> Element<'_, Message> {
        let mut content = Column::new();

        // List all the rpcs from the RPC storage
        for chain_packet in self.storage.list() {
            let mut row = Row::new().spacing(Sizes::Md);
            row = row.push(label(&chain_packet.name.clone()).secondary().build());
            row = row.push(
                label(&chain_packet.chain_id.to_string())
                    .secondary()
                    .build(),
            );
            row = row.push(label(&chain_packet.url.clone()).secondary().build());
            content = content.push(row);
        }

        content.into()
    }

    pub fn rpc_table(&self) -> TableBuilder<Message> {
        let rpcs = self
            .storage
            .list()
            .into_iter()
            .cloned()
            .collect::<Vec<ChainPacket>>();
        let selected_rpcs = self.selected_rpcs.clone();

        let table = TableBuilder::new().padding_cell(Sizes::Md).column(
            ColumnBuilder::new()
                .headers(vec![
                    "Name".to_string(),
                    "Chain ID".to_string(),
                    "URL".to_string(),
                    "Select".to_string(),
                ])
                .rows(
                    rpcs.into_iter()
                        .map(|chain_packet| {
                            RowBuilder::new()
                                .cell(
                                    cells::CellBuilder::new()
                                        .value(Some(chain_packet.name.clone())),
                                )
                                .cell(
                                    cells::CellBuilder::new()
                                        .value(Some(chain_packet.chain_id.to_string())),
                                )
                                .cell(
                                    cells::CellBuilder::new().value(Some(chain_packet.url.clone())),
                                )
                                .cell(
                                    cells::CellBuilder::new()
                                        .checked(
                                            selected_rpcs.get(&chain_packet.name.clone()).cloned(),
                                        )
                                        .on_checkbox(move |x| {
                                            Message::SelectedRPC(x, Some(chain_packet.name.clone()))
                                        }),
                                )
                        })
                        .collect::<Vec<RowBuilder<Message>>>(),
                ),
        );

        table
    }
}

impl State for RpcManagement {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Sync(storage) => {
                tracing::debug!("Syncing RPCs in rpc settings: {:?}", storage.clone());
                self.storage = storage;
            }
            Message::ChangeName(name) => {
                if let Some(chain_packet) = &mut self.chain_packet {
                    chain_packet.name = name;
                }
            }
            Message::ChangeChainId(chain_id) => {
                if let Some(chain_packet) = &mut self.chain_packet {
                    chain_packet.chain_id = chain_id;
                }
            }
            Message::ChangeUrl(url) => {
                if let Some(chain_packet) = &mut self.chain_packet {
                    chain_packet.url = url;
                }
            }
            Message::SelectedRPC(selected, name) => {
                tracing::debug!("Selected RPC: {:?} {:?}", selected, name);
                // Add to map if selected, else remove it.
                if let Some(name) = name {
                    if selected {
                        self.selected_rpcs.insert(name, selected);
                    } else {
                        self.selected_rpcs.remove(&name);
                    }
                }
            }
            Message::AddRpc => {
                tracing::debug!("Adding RPC");
                self.chain_packet = Some(Form::default());
                self.form_feedback = None;
            }
            Message::Submit => {
                tracing::debug!("Submitting RPC");
                self.reset();
            }
            Message::Delete => {
                tracing::debug!("Deleting RPCs");
                // Clear the storage of the deleted items.
                let marked_list = self.selected_rpcs.clone();
                for (name, selected) in marked_list {
                    if !selected {
                        continue;
                    }

                    self.storage.remove(&name);
                }

                // Clear the selected list
                self.selected_rpcs.clear();

                self.form_feedback = Some(Feedback::Success("Deleted RPCs!".to_string()));
            }
            Message::Feedback(feedback) => {
                tracing::debug!("Got form feedback: {:?}", feedback);
                self.form_feedback = Some(feedback);
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Lg).padding(Sizes::Lg);

        // If any rpcs are selected then the rpc delete message will be Delete, else
        // Empty.

        let mut delete_button = ExcaliburButton::new()
            .danger()
            .build(label(&"Delete RPCs").build())
            .padding(Sizes::Sm);
        if !self.selected_rpcs.is_empty() {
            delete_button = delete_button.on_press(Message::Delete);
        }

        let actions = Row::new()
            .spacing(Sizes::Md)
            .push(
                ExcaliburButton::new()
                    .primary()
                    .build(label(&"Add RPC").build())
                    .padding(Sizes::Sm)
                    .on_press(Message::AddRpc),
            )
            .push(delete_button);

        let upper_half = Column::new()
            .spacing(Sizes::Md)
            .push(label(&"Manage RPC Settings").title2().build())
            .push(actions)
            .push(Card::new(self.rpc_table().build()).padding(Sizes::Sm));

        let mut lower_half = Column::new().spacing(Sizes::Md);

        if let Some(chain_packet) = &self.chain_packet {
            let labeled_name_input = labeled_input(
                "Name".to_string(),
                chain_packet.name.clone(),
                "Choose a label".to_string(),
                |x| Message::ChangeName(x),
            );

            let labeled_chain_id_input = labeled_input(
                "Chain ID".to_string(),
                chain_packet.chain_id.clone(),
                "Choose a chain id".to_string(),
                |x| Message::ChangeChainId(x),
            );

            let labeled_url_input = labeled_input(
                "URL".to_string(),
                chain_packet.url.clone(),
                "Choose a url".to_string(),
                |x| Message::ChangeUrl(x),
            );

            let submit_button = ExcaliburButton::new()
                .primary()
                .build(label(&"Add RPC to list").build())
                .on_press(Message::Submit)
                .width(Length::Fill)
                .padding(Sizes::Md);

            let row_1 = Row::new()
                .spacing(Sizes::Sm)
                .push(labeled_name_input.width(Length::FillPortion(2)))
                .push(labeled_chain_id_input.width(Length::FillPortion(2)));

            let row_2 = Row::new()
                .spacing(Sizes::Sm)
                .push(labeled_url_input.width(Length::FillPortion(2)))
                .push(
                    Column::new()
                        .spacing(Sizes::Md)
                        .push(label(&"Instructions").build())
                        .push(submit_button)
                        .width(Length::FillPortion(2)),
                );

            let form = Column::new().push(row_1).push(row_2).spacing(Sizes::Md);

            lower_half = lower_half.push(form);
        }

        // if form error, push it as text.
        if let Some(feedback) = &self.form_feedback {
            let label = match feedback {
                Feedback::Success(message) => label(&message.clone()).style(GREEN_400).build(),
                Feedback::Error(message) => label(&message.clone()).style(RED_400).build(),
            };

            lower_half = lower_half.push(label);
        }

        content = content.push(upper_half);
        content = content.push(lower_half);

        Container::new(content)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
