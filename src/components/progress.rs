//! Custom progress bar.

use iced::widget::progress_bar;

use super::*;

pub const PROGRESS_BAR_BACKGROUND: Color = Color::from_rgb(
    0x1d as f32 / 255.0,
    0x1d as f32 / 255.0,
    0x1d as f32 / 255.0,
);

pub const PROGRESS_BAR_FOREGROUND: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);

pub const PROGRESS_BAR_BORDER_RADIUS: f32 = 5.0;
pub struct CustomProgressBar {
    background: iced::Background,
    bar: iced::Background,
    border_radius: iced::border::Radius,
}

impl Default for CustomProgressBar {
    fn default() -> Self {
        Self {
            background: (PROGRESS_BAR_BACKGROUND.into()),
            bar: (PROGRESS_BAR_FOREGROUND.into()),
            border_radius: PROGRESS_BAR_BORDER_RADIUS.into(),
        }
    }
}

impl CustomProgressBar {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn background(mut self, background: iced::Background) -> Self {
        self.background = background;
        self
    }

    #[allow(dead_code)]
    pub fn bar(mut self, bar: iced::Background) -> Self {
        self.bar = bar;
        self
    }
    #[allow(dead_code)]
    pub fn border_radius(mut self, border_radius: iced::border::Radius) -> Self {
        self.border_radius = border_radius;
        self
    }

    pub fn style(&self) -> progress_bar::Style {
        progress_bar::Style {
            background: self.background,
            bar: self.bar,
            border: iced::Border {
                radius: self.border_radius,
                ..iced::Border::default()
            },
        }
    }
}
