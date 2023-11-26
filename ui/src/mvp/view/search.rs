// Test view to see how things are working
use iced::widget::text::Text;
use iced::widget::{button, text_input::TextInput, Space};

use super::*;
pub fn simple_view<'a>() -> Element<'a, Message> {
    Container::new(Text::new("Hello, world!"))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// pub fn search_view<'a>(query: &'a mut String, search_button: &'a mut
// button::State) -> Element<'a, Message> {     let search_bar:
// TextInput<Message> = TextInput::new(query, "Search...")         .padding(5)
//         .size(16)
//         .width(Length::Fill)
//         .style(BackgroundContainerTheme::theme())
//         .on_submit(Message::SearchInputChanged);

//     let search_button: Button<Message> = Button::new(search_button,
// Text::new("Search"))         .on_press(Message::SearchButtonPressed)
//         .style(BackgroundContainerTheme::theme());

//     let content = Row::new()
//         .push(search_bar)
//         .push(search_button)
//         .spacing(20)
//         .align_items(Align::Center);

//     Container::new(content)
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .center_x()
//         .center_y()
//         .style(BackgroundContainerTheme::theme())
//         .into()
// }
