use std::collections::HashMap;

use ethers::abi::Token;
use iced::widget::{Column, Container, Row};

use self::sidebar::Page;
use super::{
    components::{containers::*, *},
    tracer::AppEventLayer,
    *,
};
use crate::{
    components::system::{label, ExcaliburColor},
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
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

/// For rendering content inside a screen that implements [`State`].
pub fn screen_layout<'a, T: Into<Element<'a, Message>>>(
    window: &'a Page,
    content: T,
) -> Element<'a, Message> {
    Container::new(screen_window(window, content))
        .center_x()
        .center_y()
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(Sizes::Xl)
        .style(CustomContainer::theme(Some(
            ExcaliburColor::Background2.color().into(),
        )))
        .into()
}

/// note: the header needs to fill the container. but this pushes the content
/// out to its max width.
/// so we need to cap the window to a max width, which we should improve on in
/// the future.
pub fn screen_window<'a, T: Into<Element<'a, Message>>>(
    window: &'a Page,
    content: T,
) -> Container<'a, Message, iced::Renderer> {
    let name = window.name().clone();
    Container::new(
        Column::new()
            .push(Row::new().push(content))
            .spacing(Sizes::Md as u16),
    )
    .max_height(ByteScale::Xl7 as u16)
}
