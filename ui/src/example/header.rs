//! Simple header component that fills the top of the screen and renders a
//! "file" button a title in the center.

use iced::{
    alignment,
    widget::{button, column, container, row, text},
    Element, Font, Length, Renderer,
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
            .style(iced::theme::Text::Color(iced::Color::BLACK));

        let button = button(
            text("Home")
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .padding(10)
        .on_press(Message::ButtonPressed);

        let row = row![button]
            .push(column![title].padding(8))
            .spacing(4)
            .align_items(alignment::Alignment::Center)
            .height(Length::Fill);

        container(row)
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(45.0))
            .style(super::styles::background::BackgroundContainer::theme())
            .into()
    }
}
