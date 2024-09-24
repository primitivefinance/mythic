//! Navigation is a component for switching between pages like the settings or dashboard.

use iced::widget::{Column, Row, Space};
use iced::{Color, Element, Length};
use iced_aw::{graphics::icons::icon_to_char, Icon};

use super::*;
use crate::components::button::*;
use crate::components::system::label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum Navigation {
    Empty,

    #[default]
    Dashboard,

    Settings,

    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum NavEvent {
    #[default]
    Empty,

    Navigate(Navigation),
}

pub type NavItem = (Icon, String, NavEvent, bool);

impl Navigation {
    pub fn label(&self) -> String {
        match self {
            Navigation::Empty => "Select".to_string(),
            Navigation::Dashboard => "Dashboard".to_string(),
            Navigation::Settings => "Settings".to_string(),
            Navigation::Exit => "Exit".to_string(),
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Navigation::Empty => Icon::TerminalFill,
            Navigation::Dashboard => Icon::House,
            Navigation::Settings => Icon::Gear,
            Navigation::Exit => Icon::X,
        }
    }

    pub fn item(&self, active: &Navigation) -> NavItem {
        let name = self.label();
        let icon = self.icon();

        (icon, name, NavEvent::Navigate(*self), *self == *active)
    }

    pub fn items(active: &Navigation) -> Vec<NavItem> {
        vec![
            Navigation::Dashboard.item(active),
            Navigation::Settings.item(active),
            Navigation::Exit.item(active),
        ]
    }

    pub fn view<'a>(&self) -> Element<'a, NavEvent> {
        let mut column = Column::new();
        for (icon, name, msg, selected) in Self::items(self) {
            let style = match selected {
                true => route_button_style(Color::TRANSPARENT)
                    .hovered()
                    .background(Some(Color::TRANSPARENT.into())),
                false => route_button_style(Color::TRANSPARENT),
            };

            let mut app_name = label(name);

            if !selected {
                app_name = app_name.secondary();
            }

            column = column.push(
                button(
                    Row::new()
                        .push(Space::with_width(Length::Fixed(Sizes::Xs.into())))
                        .push(label(icon_to_char(icon)).icon().build())
                        .push(app_name.build())
                        .spacing(Sizes::Md),
                )
                .width(Length::Fill)
                .on_press(msg)
                .style(style.as_custom())
                .padding(Sizes::Sm),
            );
        }

        column
            .spacing(Sizes::Xs)
            .align_items(alignment::Alignment::Center)
            .into()
    }
}

impl MessageWrapper for NavEvent {
    type ParentMessage = app::AppMessage;
}

impl MessageWrapperView for NavEvent {
    type ParentMessage = view::ViewMessage;
}

impl From<NavEvent> for <NavEvent as MessageWrapper>::ParentMessage {
    fn from(msg: NavEvent) -> Self {
        app::AppMessage::View(view::ViewMessage::Root(view::RootMessage::Route(msg)))
    }
}

impl From<NavEvent> for <NavEvent as MessageWrapperView>::ParentMessage {
    fn from(msg: NavEvent) -> Self {
        view::ViewMessage::Root(view::RootMessage::Route(msg))
    }
}
