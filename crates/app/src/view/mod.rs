use iced::widget::{Column, Container, Row};

use self::sidebar::Page;
use super::*;
use crate::{
    components::system::{ExcaliburColor, ExcaliburContainer},
    screens::State,
};

pub mod sidebar;

/// Root message for the Terminal component.
#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    // Exit application
    Exit,
    // Confirm exit application
    ConfirmExit,
    // Route to a new page.
    Route(sidebar::Route),
    // Copy to clipboard.
    CopyToClipboard(String),
    // Production pages.
    Portfolio(portfolio::Message),
    Settings(settings::Message),
    // Developer pages
    Developer(dev::Message),
    Experimental(dev::experimental::Message),
}

impl MessageWrapperView for Message {
    type ParentMessage = Message;
}

impl MessageWrapper for Message {
    type ParentMessage = app::Message;
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::View(message)
    }
}

/// Root layout that is rendered by the root application.
///
/// Renders two components in a row:
/// - Sidebar menu
/// - Content
///
/// Content is wrapped by its own layout.
pub fn app_layout<'a, T: Into<Element<'a, Message>>>(
    menu: &'a sidebar::Sidebar,
    content: T,
) -> Element<'a, Message> {
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

/// For rendering content inside a screen that implements [`State`].
pub fn screen_layout<'a, T: Into<Element<'a, Message>>>(
    _window: &'a Page,
    content: T,
) -> Element<'a, Message> {
    Container::new(content)
        .center_x()
        .center_y()
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Sizes::Xl)
        .style(
            ExcaliburContainer::default()
                .background(ExcaliburColor::Background2)
                .theme(),
        )
        .into()
}
