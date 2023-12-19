use iced::widget::{Column, Container, Row};
use iced_aw::{floating_element::Anchor, helpers::floating_element};

use self::sidebar::Page;
use super::*;
use crate::{
    app::AppClock,
    components::system::{label, ExcaliburColor, ExcaliburContainer},
    controller::State,
};

pub mod portfolio_view;
pub mod sidebar;
// TODO: execution is not complete
// pub mod execute;

/// All controllers emit View messages. These get drilled down to the original
/// controller that emitted them.
///
/// This enables controllers to communicate with controllers above them because
/// the view messages start at the root application controller.
#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    // Root controller messages are "caught" in flight in the application's
    // update function. Controllers can indirectly communicate with the root application
    // this way.
    Root(RootMessage),

    // Children controllers emit their own messages that they expect to get back and process on
    // their own.
    Portfolio(portfolio::Message),
    Settings(settings::Message),
    Developer(dev::Message),
    Experimental(dev::experimental::Message),
}

#[derive(Debug, Clone, Default)]
pub enum RootMessage {
    #[default]
    Empty,
    // Exit the application safely, saving all persistent data before exiting.
    SaveAndExit,
    // Confirm exit application
    ConfirmExit,
    // Route to a new page.
    Route(sidebar::Route),
    // Copy to clipboard.
    CopyToClipboard(String),
    // Updates the model.
    ModelSyncRequest,
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
    app_clock: &'a AppClock,
    menu: &'a sidebar::Sidebar,
    content: T,
) -> Element<'a, Message> {
    let floating_clock = floating_element(
        Container::new(Column::new())
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0))
            .center_x()
            .center_y(),
        Column::new()
            .push(label("Performance").tertiary().caption2().build())
            .push(app_clock.view_max())
            .push(app_clock.view_min())
            .push(app_clock.view_average())
            .push(app_clock.view_tbu())
            .push(app_clock.view_frequency()),
    )
    .anchor(Anchor::SouthWest)
    .offset(20.0)
    .hide(std::env::var("DEV_MODE").is_err());

    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(menu.view())
                    .push(floating_clock)
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
        .style(ExcaliburContainer::default().middle_bottom().theme())
        .into()
}
