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
pub struct SidebarContainer;

impl StyleSheet for SidebarContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(SIDEBAR_BG_COLOR)),
            border_width: 1.0,
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
            border_width: 1.0,
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
            border_width: 1.0,
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
    #[allow(dead_code)]
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

pub struct CustomContainer {
    pub background: Option<iced::Background>,
    pub border_radius: BorderRadius,
    pub border_width: f32,
    pub border_color: Color,
}

impl Default for CustomContainer {
    fn default() -> Self {
        Self {
            background: None,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl StyleSheet for CustomContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as StyleSheet>::Style) -> Appearance {
        Appearance {
            background: self.background,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
            ..Default::default()
        }
    }
}

impl CustomContainer {
    pub fn theme(background: Option<iced::Background>) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(CustomContainer {
            background,
            ..Default::default()
        }))
    }
    pub fn theme_with_border_radius(
        background: Option<iced::Background>,
        border_radius: Option<BorderRadius>,
    ) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(CustomContainer {
            background,
            border_radius: border_radius.unwrap_or(0.0.into()),
            ..Default::default()
        }))
    }
}
