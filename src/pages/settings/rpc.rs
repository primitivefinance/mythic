use std::collections::HashMap;

use iced::{widget::text_input, Fill, Padding};

use self::system::ExcaliburContainer;
use super::*;
use crate::{
    components::system::{label, ExcaliburButton},
    data::rpcs::{RPCList, RPCValue},
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
    pub fn new(mut storage: RPCList) -> Self {
        if storage.chains.get("flashbot").is_none() {
            storage
                .chains
                .insert("flashbot".to_owned(), RPCValue::default());
        }
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

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn get_chain_packet(&self) -> anyhow::Result<RPCValue, anyhow::Error> {
        if let Some(chain_packet) = &self.chain_packet {
            let name = chain_packet.name.clone();
            let chain_id = chain_packet.chain_id.clone();
            let url = chain_packet.url.clone();

            if let (Some(name), Some(chain_id), Some(url)) = (name, chain_id, url) {
                let chain_id = chain_id.parse::<u64>().map_err(|_| {
                    anyhow::anyhow!("Chain ID must be a number!").context("Chain ID error")
                })?;
                let chain_packet = RPCValue {
                    name,
                    chain_id,
                    url,
                };
                return Ok(chain_packet);
            }
        }

        Err(anyhow::anyhow!("No form fields updated!"))
    }

    #[allow(dead_code)]
    pub fn view_rpcs(&self) -> Element<'_, Message> {
        let mut content = Column::new();
        tracing::debug!("Chain packet: {:?}", self.storage.list());
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

    pub fn form_item<'a, Message>(
        title: impl ToString,
        content: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default,
    {
        ExcaliburContainer::default().transparent().build(
            Column::new()
                .spacing(Sizes::Md)
                .push(label(title).secondary().build())
                .push(
                    ExcaliburContainer::default()
                        .round(Sizes::Sm)
                        .middle_top()
                        .light_border()
                        .build(content),
                ),
        )
    }
}

impl Lifecycle for RpcManagement {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Task<Self::AppMessage> {
        Task::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Task<Self::AppMessage> {
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
                let mut storage = self.storage.clone();
                if let Ok(chain_packet) = self.get_chain_packet() {
                    let chain_packet_name = chain_packet.name.clone();
                    storage.chains.insert(chain_packet_name, chain_packet);
                }
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

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Lg).padding(Sizes::Lg);

        // If any rpcs are selected then the rpc delete message will be Delete, else
        // Empty.

        let mut delete_button = ExcaliburButton::new()
            .build(label("Delete RPCs").build())
            .padding(Sizes::Sm);
        if !self.selected_rpcs.is_empty() {
            delete_button = delete_button.on_press(Message::Delete);
        }

        let actions = Row::new()
            .spacing(Sizes::Md)
            .push(
                ExcaliburButton::new()
                    .build(label("Add RPC").build())
                    .padding(Sizes::Sm)
                    .on_press(Message::AddRpc),
            )
            .push(delete_button);

        let upper_half = Column::new()
            .spacing(Sizes::Md)
            .push(label("Manage RPC Settings").title2().build())
            .push(actions)
            .push(
                ExcaliburContainer::default()
                    .light_border()
                    .build(Column::new().push(iced::widget::text("todo".to_string()))),
            );

        let mut lower_half = Column::new().spacing(Sizes::Md);

        if let Some(chain_packet) = &self.chain_packet {
            let labeled_name_input = RpcManagement::form_item(
                "Name",
                Column::new().push(text_input(
                    "Choose a name",
                    &chain_packet.name.clone().unwrap_or_default(),
                )),
            );
            let labeled_chain_id_input = RpcManagement::form_item(
                "Chain ID",
                Column::new().push(text_input(
                    "Choose a chain id",
                    &chain_packet.chain_id.clone().unwrap_or_default(),
                )),
            );

            let labeled_url_input = RpcManagement::form_item(
                "URL",
                Column::new().push(text_input(
                    "Choose a url",
                    &chain_packet.url.clone().unwrap_or_default(),
                )),
            );

            let submit_button = ExcaliburButton::new()
                .build(label("Add RPC to list").build())
                .on_press(Message::Submit)
                .padding(Sizes::Sm);

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
                        .push(label("Submit").secondary().build())
                        .push(submit_button)
                        .width(Length::FillPortion(2)),
                );

            let form = Column::new().push(row_1).push(row_2).spacing(Sizes::Md);

            lower_half = lower_half.push(form);
        }

        // if form error, push it as text.
        if let Some(feedback) = &self.form_feedback {
            let label = match feedback {
                Feedback::Success(message) => label(message.clone()).style(GREEN_400).build(),
                Feedback::Error(message) => label(message.clone()).style(RED_400).build(),
            };

            lower_half = lower_half.push(label);
        }

        content = content.push(upper_half);
        content = content.push(lower_half);

        Container::new(content)
            .center_x(Fill)
            .width(Fill)
            .height(Fill)
            .into()
    }
}
