use iced::widget::{Column, Container, Row};

use self::sidebar::Page;
use super::*;
use crate::{
    components::system::{ExcaliburColor, ExcaliburContainer},
    pages::State,
};

pub mod sidebar;

#[derive(Debug, Clone, Default)]
pub enum ViewMessage {
    #[default]
    Empty,
    Root(RootMessage),

    Portfolio(portfolio::Message),

    Settings(settings::Message),
}

#[derive(Debug, Clone, Default)]
pub enum RootMessage {
    #[default]
    Empty,
    SaveAndExit,
    ConfirmExit,
    Route(sidebar::Route),
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

pub fn app_layout<'a, T: Into<Element<'a, ViewMessage>>>(
    menu: &'a sidebar::Sidebar,
    content: T,
) -> Element<'a, ViewMessage> {
    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(menu.view())
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(screen_layout(&menu.page, content))
                    .width(Length::FillPortion(5)),
            ),
    )
    .style(
        ExcaliburContainer::default()
            .background(ExcaliburColor::Background1)
            .theme(),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

pub fn screen_layout<'a, T: Into<Element<'a, ViewMessage>>>(
    _window: &'a Page,
    content: T,
) -> Element<'a, ViewMessage> {
    Container::new(content)
        .center_x()
        .center_y()
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(ExcaliburContainer::default().middle_bottom().theme())
        .into()
}
