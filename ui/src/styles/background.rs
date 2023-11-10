use iced::{widget::container, Color};

pub const LEVEL_1_COLOR: Color = Color::from_rgb(
    0xF8 as f32 / 255.0,
    0xF8 as f32 / 255.0,
    0xF8 as f32 / 255.0,
);

pub const LEVEL_2_COLOR: Color = Color::from_rgb(
    0xF4 as f32 / 255.0,
    0xF4 as f32 / 255.0,
    0xF5 as f32 / 255.0,
);

pub const LEVEL_3_COLOR: Color = Color::from_rgb(
    0xEA as f32 / 255.0,
    0xEA as f32 / 255.0,
    0xEB as f32 / 255.0,
);

pub const LEVEL_4_COLOR: Color = Color::from_rgb(
    0xD4 as f32 / 255.0,
    0xD4 as f32 / 255.0,
    0xD6 as f32 / 255.0,
);

pub const BG_CONTAINER: Color = Color::from_rgb(
    0xF3 as f32 / 255.0,
    0xF3 as f32 / 255.0,
    0xF3 as f32 / 255.0,
);

pub struct BackgroundContainer;

impl iced::widget::container::StyleSheet for BackgroundContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(LEVEL_3_COLOR)),
            ..Default::default()
        }
    }
}

impl BackgroundContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(BackgroundContainer))
    }
}

pub struct WhiteContainer;

const BORDER_COLOR: Color = Color::from_rgb(
    0xEE as f32 / 255.0,
    0xEE as f32 / 255.0,
    0xEE as f32 / 255.0,
);

impl iced::widget::container::StyleSheet for WhiteContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::WHITE)),
            border_radius: [10.0, 0.0, 0.0, 0.0].into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl WhiteContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(WhiteContainer))
    }
}

pub struct Layer1Container;

impl iced::widget::container::StyleSheet for Layer1Container {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(LEVEL_1_COLOR)),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl Layer1Container {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Layer1Container))
    }
}

pub struct Layer2Container;

impl iced::widget::container::StyleSheet for Layer2Container {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(LEVEL_2_COLOR)),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl Layer2Container {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Layer2Container))
    }
}

pub struct BorderedContainer;

impl iced::widget::container::StyleSheet for BorderedContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 2.0.into(),
            border_width: 2.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl BorderedContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(BorderedContainer))
    }
}

pub struct FirehoseContainer;

impl iced::widget::container::StyleSheet for FirehoseContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(LEVEL_3_COLOR)),
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
