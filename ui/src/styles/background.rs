use iced::{widget::container, Color};

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
            background: Some(iced::Background::Color(BG_CONTAINER)),
            ..Default::default()
        }
    }
}

impl BackgroundContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(BackgroundContainer))
    }
}

pub struct Layer1Container;

impl iced::widget::container::StyleSheet for Layer1Container {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::WHITE)),
            border_radius: [10.0, 0.0, 0.0, 0.0].into(),
            ..Default::default()
        }
    }
}

impl Layer1Container {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Layer1Container))
    }
}
