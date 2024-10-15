//! Navigation is a component for switching between pages like the settings or dashboard.

use iced::widget::{Column, Row, Space};
use iced::{Element, Length};

use iced_aw::{bootstrap::icon_to_char, Bootstrap};

use super::*;
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

pub type NavItem = (Bootstrap, String, NavEvent, bool);

impl Navigation {
    pub fn label(&self) -> String {
        match self {
            Navigation::Empty => "Select".to_string(),
            Navigation::Dashboard => "Dashboard".to_string(),
            Navigation::Settings => "Settings".to_string(),
            Navigation::Exit => "Exit".to_string(),
        }
    }

    pub fn icon(&self) -> Bootstrap {
        match self {
            Navigation::Empty => Bootstrap::TerminalFill,
            Navigation::Dashboard => Bootstrap::House,
            Navigation::Settings => Bootstrap::Gear,
            Navigation::Exit => Bootstrap::X,
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
                true => iced::widget::button::primary,
                false => iced::widget::button::secondary,
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
                .style(style)
                .padding(Sizes::Sm),
            );
        }

        column
            .spacing(Sizes::Xs)
            .align_x(alignment::Alignment::Center)
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
