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
    app_clock: &'a AppClock,
    menu: &'a sidebar::Sidebar,
    content: T,
) -> Element<'a, ViewMessage> {
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
