use iced::widget::container::*;

use super::*;

/// For rendering the background color.
pub struct BackgroundContainerTheme;

impl StyleSheet for BackgroundContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(BACKGROUND)),
            ..Default::default()
        }
    }
}

impl BackgroundContainerTheme {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(BackgroundContainerTheme))
    }
}

/// For rendering anything placed on the background container.
pub struct MenuContainerTheme;

impl StyleSheet for MenuContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(PANEL)),
            border_radius: 9.0.into(),
            ..Default::default()
        }
    }
}

impl MenuContainerTheme {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(MenuContainerTheme))
    }
}

/// For anything placed on a menu container.
pub struct MenuItemContainerTheme;

impl StyleSheet for MenuItemContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(MODAL)),
            border_radius: 11.0.into(),
            ..Default::default()
        }
    }
}

impl MenuItemContainerTheme {
    #[allow(dead_code)]
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(MenuItemContainerTheme))
    }
}

pub struct FirehoseContainer;

impl StyleSheet for FirehoseContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(PANEL)),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl FirehoseContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(FirehoseContainer))
    }
}

pub struct FirehoseTrace;

impl StyleSheet for FirehoseTrace {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(MODAL)),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl FirehoseTrace {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(FirehoseTrace))
    }
}

pub struct CardContainer;

impl StyleSheet for CardContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(CARD_BG_COLOR)),
            border_radius: 9.0.into(),
            ..Default::default()
        }
    }
}

impl CardContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(CardContainer))
    }
}
