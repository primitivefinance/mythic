pub mod rpc;

use anyhow::anyhow;
use iced::widget::Container;
use iced::Fill;
use iced_aw::Bootstrap;

use super::*;
use crate::{
    app::{RootMessage, RootViewMessage, UserProfileMessage},
    data::user::UserProfile,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Route(Pages),
    Rpc(rpc::Message),
}

impl MessageWrapper for Message {
    type ParentMessage = RootMessage;
}

impl MessageWrapperView for Message {
    type ParentMessage = RootViewMessage;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::View(view::ViewMessage::Settings(message))
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

pub struct SettingsPage {
    pub active: Pages,
    pub rpc: rpc::RpcManagement,
}

impl SettingsPage {
    pub fn new(user: UserProfile) -> Self {
        Self {
            active: Pages::default(),
            rpc: rpc::RpcManagement::new(user.rpcs.clone()),
        }
    }

    pub fn pages(&self) -> Vec<NavigationStep<RootViewMessage>> {
        vec![NavigationStep::new(
            Bootstrap::Lightning,
            "RPC",
            Message::Route(Pages::Rpc).into(),
            self.active == Pages::Rpc,
            false,
        )]
    }

    fn switch_page(&mut self, page: Pages) -> Task<Message> {
        self.active = page;
        Task::none()
    }
}

impl From<SettingsPage> for Page {
    fn from(screen: SettingsPage) -> Self {
        Page::new(Box::new(screen))
    }
}

impl Lifecycle for SettingsPage {
    type AppMessage = RootMessage;
    type ViewMessage = RootViewMessage;

    fn load(&self) -> Task<Self::AppMessage> {
        Task::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Task<Self::AppMessage> {
        if let Self::AppMessage::View(view::ViewMessage::Settings(message)) = message {
            match message {
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
                                Task::perform(async {}, move |_| {
                                    app::UserProfileMessage::RemoveRPC(name.clone())
                                })
                                .map(|x| x.into()),
                            );
                        }

                        commands.push(self.rpc.update(message).map(|x| Message::Rpc(x).into()));
                        Task::batch(commands)
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
                                    Task::perform(async {}, move |_| {
                                        UserProfileMessage::AddRPC(chain.clone())
                                    })
                                    .map(|x| x.into()),
                                );
                                commands
                                    .push(self.rpc.update(message).map(|x| Message::Rpc(x).into()));
                                Task::batch(commands)
                            }
                            Err(e) => {
                                tracing::error!("Failed to submit new RPC packet: {:?}", e);

                                Task::perform(async {}, move |_| {
                                    view::ViewMessage::Settings(settings::Message::Rpc(
                                        settings::rpc::Message::Feedback(
                                            anyhow!("failed to submit new RPC packet".to_string())
                                                .into(),
                                        ),
                                    ))
                                })
                                .map(|x| x.into())
                            }
                        }
                    }
                    _ => self.rpc.update(message).map(|x| Message::Rpc(x).into()),
                },

                Message::Route(page) => self.switch_page(page).map(|x| x.into()),
                _ => Task::none(),
            }
        } else {
            Task::none()
        }
    }

    fn view(&self) -> Element<'_, RootViewMessage> {
        let nav = navigation_steps("All Settings", self.pages()).width(Length::FillPortion(1));
        let mut content = Row::new().push(nav);

        let nav_content = match self.active {
            Pages::Rpc => self.rpc.view().map(move |x| Message::Rpc(x).into()),
            _ => Column::new().into(),
        };

        content = content.push(
            Column::new()
                .push(nav_content)
                .width(Length::FillPortion(3)),
        );

        Container::new(content)
            .center_x(Fill)
            .padding(Sizes::Lg)
            .width(Fill)
            .height(Fill)
            .into()
    }
}
