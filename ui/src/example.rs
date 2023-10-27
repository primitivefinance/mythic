#![allow(unused_variables)]
use std::sync::Arc;

use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};
use iced::{
    executor,
    widget::{button, column, container, text},
    Application, Command, Element, Length, Theme,
};

use crate::counter_component::*;
use crate::vault::*;

#[allow(clippy::large_enum_variant)]
pub enum Example {
    BuildingEnvironment,
    Running {
        environment: Environment,
        client: Arc<RevmMiddleware>,
        deployment: DeploymentState,
    },
}

pub enum DeploymentState {
    NotDeployed,
    Deploying,
    Deployed(Vault),
    DeploymentFailed(Error),
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Deploy,
    Deployed(Result<Vault, Error>),
    CounterMessage(Option<u32>),
}

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
    ProviderConnectionError,
    BlockSubscriptionError,
}

impl Application for Example {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            {
                let env = EnvironmentBuilder::new().build();
                let client = RevmMiddleware::new(&env, Some("client")).unwrap();
                Self::Running {
                    environment: env,
                    client,
                    deployment: DeploymentState::NotDeployed,
                }
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Arbiter UI")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Deploy => {
                if let Self::Running {
                    ref client,
                    deployment,
                    ..
                } = self
                {
                    *deployment = DeploymentState::Deploying;
                    let client_clone = client.clone();
                    return Command::perform(Vault::deploy(client_clone), |_| {
                        Message::Deployed(Ok(Vault { valid: true }))
                    });
                }
            }
            Message::Deployed(result) => {
                if let Self::Running { deployment, .. } = self {
                    match result {
                        Ok(vault) => {
                            *deployment = DeploymentState::Deployed(vault);
                        }
                        Err(error) => {
                            *deployment = DeploymentState::DeploymentFailed(error);
                        }
                    }
                }
            }
            Message::CounterMessage(msg) => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content: Element<_> = match self {
            Example::BuildingEnvironment => text("Building Environment...").size(40).into(),
            Example::Running {
                client, deployment, ..
            } => {
                // Base container for the Running state
                let mut content = column![];

                // Add the Deploy button
                content = content.push(button("Deploy").on_press(Message::Deploy));

                // Reflect the deployment state in the UI
                content = match deployment {
                    DeploymentState::NotDeployed => content,
                    DeploymentState::Deploying => content.push(text("Deploying...")),
                    DeploymentState::Deployed(_entity) => {
                        content.push(text("Successfully Deployed"))
                    }
                    DeploymentState::DeploymentFailed(error) => {
                        content.push(text(format!("Deployment failed: {:?}", error)))
                    }
                };

                content = content.push(counter_state(Some(1), Message::CounterMessage));

                content.into()
            }
        };

        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

// yo!!
impl From<ethers::providers::ProviderError> for Error {
    fn from(_error: ethers::providers::ProviderError) -> Self {
        match _error {
            ethers::providers::ProviderError::UnsupportedRPC => Error::APIError,
            ethers::providers::ProviderError::JsonRpcClientError(_error) => {
                println!("Error: {:#?}", _error);

                Error::ProviderConnectionError
            }
            _ => Error::BlockSubscriptionError,
        }
    }
}
