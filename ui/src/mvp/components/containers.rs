use iced::{widget::container::*, BorderRadius};

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

/// For rendering anything placed on the background container.
pub struct SidebarContainer;

impl StyleSheet for SidebarContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(SIDEBAR_BG_COLOR)),
            border_width: 1.0.into(),
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}

impl SidebarContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(SidebarContainer))
    }
}

/// For rendering anything placed on the background container.
pub struct ScreenWindowContainer;

impl StyleSheet for ScreenWindowContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(PANEL)),
            border_radius: 9.0.into(),
            border_width: 1.0.into(),
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}

impl ScreenWindowContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(ScreenWindowContainer))
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

pub struct WindowHeader;

impl StyleSheet for WindowHeader {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(WINDOW_HEADER_COLOR)),
            border_radius: [9.0, 9.0, 0.0, 0.0].into(),
            border_width: 1.0.into(),
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}

impl WindowHeader {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(WindowHeader))
    }
}

pub struct InfoContainer;

impl StyleSheet for InfoContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(INFO_BG_CONTAINER)),
            border_radius: 9.0.into(),
            ..Default::default()
        }
    }
}

impl InfoContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(InfoContainer))
    }
}

pub struct BorderedContainer {
    pub border_radius: BorderRadius,
}

impl BorderedContainer {
    pub fn new(border_radius: BorderRadius) -> Self {
        Self { border_radius }
    }
}

impl Default for BorderedContainer {
    fn default() -> Self {
        Self {
            border_radius: 5.0.into(),
        }
    }
}

impl StyleSheet for BorderedContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            border_radius: self.border_radius,
            border_width: 1.0,
            border_color: STROKE_COLOR,
            ..Default::default()
        }
    }
}

impl BorderedContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(BorderedContainer::default()))
    }

    pub fn theme_with_border_radius(border_radius: BorderRadius) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Self::new(border_radius)))
    }
}

pub struct TableColumnContainer {
    pub border_radius: BorderRadius,
}

impl TableColumnContainer {
    pub fn new(border_radius: BorderRadius) -> Self {
        Self { border_radius }
    }
}

impl Default for TableColumnContainer {
    fn default() -> Self {
        Self {
            border_radius: 5.0.into(),
        }
    }
}

impl StyleSheet for TableColumnContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(HIGHLIGHTED_CONTAINER_COLOR)),
            border_radius: self.border_radius,
            border_width: 1.0,
            border_color: STROKE_COLOR,
            ..Default::default()
        }
    }
}

impl TableColumnContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(TableColumnContainer::default()))
    }

    pub fn theme_with_border_radius(border_radius: BorderRadius) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Self::new(border_radius)))
    }
}

pub struct Indicator;

impl iced::widget::container::StyleSheet for Indicator {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(PRIMARY_COLOR)),
            ..Default::default()
        }
    }
}

impl Indicator {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Indicator))
    }
}

pub struct ContainerBlackBg;

impl iced::widget::container::StyleSheet for ContainerBlackBg {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::BLACK)),
            ..Default::default()
        }
    }
}

impl ContainerBlackBg {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(ContainerBlackBg))
    }
}
