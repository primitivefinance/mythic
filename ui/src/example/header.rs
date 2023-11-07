//! Simple header component that fills the top of the screen and renders a
//! "file" button a title in the center.

use iced::{
    alignment,
    widget::{button, column, container, row, text},
    Color, Element, Font, Length, Renderer,
};

pub struct Header {
    pub title: String,
    pub font: Font,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
}

const FONT_DAGGERSQUARE: Font = Font::with_name("DAGGERSQUARE");

impl Header {
    pub fn new(title: String) -> Self {
        let font = FONT_DAGGERSQUARE;
        Self { title, font }
    }

    pub fn view<'a>(&self) -> Element<'a, Message, Renderer> {
        let title = text(&self.title)
            .size(18)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right)
            .vertical_alignment(alignment::Vertical::Center)
            .font(self.font)
            .style(iced::theme::Text::Color(iced::Color::WHITE));

        let button = button(
            text("Home")
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .padding(10)
        .on_press(Message::ButtonPressed);

        let row = row![button]
            .push(column![title].padding(4))
            .spacing(4)
            .align_items(alignment::Alignment::Center)
            .height(Length::Fill);

        container(row)
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(45.0))
            .style(HeaderTheme::theme())
            .into()
    }
}

pub struct HeaderTheme;

pub const HEADER_COLOR: Color = Color::from_rgb(
    0x2C as f32 / 255.0,
    0x2C as f32 / 255.0,
    0x2C as f32 / 255.0,
);

impl iced::widget::container::StyleSheet for HeaderTheme {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(HEADER_COLOR)),
            ..Default::default()
        }
    }
}

impl HeaderTheme {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(HeaderTheme))
    }
}
