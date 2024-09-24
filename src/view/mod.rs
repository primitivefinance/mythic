use iced::widget::{Column, Container, Row};

use super::*;
use crate::components::system::ExcaliburContainer;

pub mod document;
pub mod header;
pub mod navigation;

#[derive(Debug, Clone, Default)]
pub enum ViewMessage {
    #[default]
    Empty,
    Root(RootMessage),

    Dashboard(dashboard::Message),

    Settings(settings::Message),
}

#[derive(Debug, Clone, Default)]
pub enum RootMessage {
    #[default]
    Empty,
    SaveAndExit,
    ConfirmExit,
    Route(navigation::NavEvent),
    CopyToClipboard(String),
    ModelSyncRequest,
}

impl MessageWrapperView for ViewMessage {
    type ParentMessage = ViewMessage;
}

impl MessageWrapper for ViewMessage {
    type ParentMessage = app::AppMessage;
}

impl From<ViewMessage> for app::AppMessage {
    fn from(message: ViewMessage) -> Self {
        app::AppMessage::View(message)
    }
}
