//! Settings has all user settings and preferences, including rpc, signer, and
//! contacts management.

pub mod contacts;
pub mod rpc;
pub mod signers;

use anyhow::anyhow;
use iced::widget::Container;
use iced_aw::Icon;

use super::*;
use crate::{
    app::{RootMessage, RootViewMessage, UserProfileMessage},
    model::user::UserProfile,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Route(Pages),
    Rpc(rpc::Message),
    Signers(signers::Message),
    Contacts(contacts::Message),
}

impl MessageWrapper for Message {
    type ParentMessage = RootMessage;
}

impl MessageWrapperView for Message {
    type ParentMessage = RootViewMessage;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(_message: Message) -> Self {
        Self::Empty
    }
}

impl From<Message> for <Message as MessageWrapperView>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Settings(message)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Pages {
    Empty,
    #[default]
    Rpc,
    Signers,
    Contacts,
}

pub struct SettingsScreen {
    pub active: Pages,
    pub rpc: rpc::RpcManagement,
    pub signers: signers::SignerManagement,
    pub contacts: contacts::ContactsManagement,
}

impl SettingsScreen {
    pub fn new(user: UserProfile) -> Self {
        Self {
            active: Pages::default(),
            rpc: rpc::RpcManagement::new(user.rpcs.clone()),
            signers: signers::SignerManagement::new(),
            contacts: contacts::ContactsManagement::new(),
        }
    }

    pub fn pages(&self) -> Vec<NavigationStep<RootViewMessage>> {
        vec![
            NavigationStep::new(
                Icon::Clock,
                "rpc",
                Message::Route(Pages::Rpc).into(),
                self.active == Pages::Rpc,
                false,
            ),
            NavigationStep::new(
                Icon::Clock,
                "signers",
                Message::Route(Pages::Signers).into(),
                self.active == Pages::Signers,
                false,
            ),
            NavigationStep::new(
                Icon::Clock,
                "contacts",
                Message::Route(Pages::Contacts).into(),
                self.active == Pages::Contacts,
                false,
            ),
        ]
    }

    fn switch_page(&mut self, page: Pages) -> Command<Message> {
        self.active = page;
        Command::none()
    }
}

impl From<SettingsScreen> for Screen {
    fn from(screen: SettingsScreen) -> Self {
        Screen::new(Box::new(screen))
    }
}

impl State for SettingsScreen {
    type AppMessage = RootMessage;
    type ViewMessage = RootViewMessage;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::Message::Settings(message)) => match message {
                Message::Rpc(message) => match message {
                    rpc::Message::Delete => {
                        tracing::debug!("Deleting RPCs from profile.");
                        let marked_list = self.rpc.selected_rpcs.clone();
                        let mut commands = vec![];
                        for (name, selected) in marked_list {
                            if !selected {
                                continue;
                            }

                            commands.push(
                                Command::perform(async {}, move |_| {
                                    app::UserProfileMessage::RemoveRPC(name)
                                })
                                .map(|x| x.into()),
                            );
                        }

                        commands.push(self.rpc.update(message).map(|x| Message::Rpc(x).into()));
                        return Command::batch(commands);
                    }
                    rpc::Message::Submit => {
                        let chain = self.rpc.get_chain_packet();

                        match chain {
                            Ok(chain) => {
                                tracing::debug!(
                                    "Submitted new RPC packet successfully: {:?}",
                                    chain.clone()
                                );

                                let mut commands = vec![];
                                commands.push(
                                    Command::perform(async {}, |_| {
                                        UserProfileMessage::AddRPC(chain)
                                    })
                                    .map(|x| x.into()),
                                );
                                commands
                                    .push(self.rpc.update(message).map(|x| Message::Rpc(x).into()));
                                return Command::batch(commands);
                            }
                            Err(e) => {
                                tracing::error!("Failed to submit new RPC packet: {:?}", e);

                                return Command::perform(async {}, move |_| {
                                    view::Message::Settings(settings::Message::Rpc(
                                        settings::rpc::Message::Feedback(anyhow!(e).into()),
                                    ))
                                })
                                .map(|x| x.into());
                            }
                        }
                    }
                    _ => return self.rpc.update(message).map(|x| Message::Rpc(x).into()),
                },
                Message::Signers(message) => {
                    return self
                        .signers
                        .update(message)
                        .map(|x| Message::Signers(x).into())
                }
                Message::Contacts(message) => {
                    return self
                        .contacts
                        .update(message)
                        .map(|x| Message::Contacts(x).into())
                }
                Message::Route(page) => return self.switch_page(page).map(|x| x.into()),
                _ => {}
            },
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, RootViewMessage> {
        let nav = navigation_steps("All", self.pages()).width(Length::FillPortion(1));
        let mut content = Row::new().push(nav);

        let nav_content = match self.active {
            Pages::Rpc => self.rpc.view().map(move |x| Message::Rpc(x).into()),
            Pages::Signers => self.signers.view().map(move |x| Message::Signers(x).into()),
            Pages::Contacts => self
                .contacts
                .view()
                .map(move |x| Message::Contacts(x).into()),
            _ => Column::new().into(),
        };

        content = content.push(
            Column::new()
                .push(nav_content)
                .width(Length::FillPortion(3)),
        );

        Container::new(content)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
