// Test view to see how things are working
use iced::{widget::{button, column, row, text_input, text::Text, text_input::TextInput, Space, horizontal_rule}, Alignment, advanced::graphics::core::window::icon, Pixels, alignment::Horizontal};
use iced_aw::{graphics::icons::icon_to_char, ICON_FONT};

use crate::mvp::{
    api::contacts::ContactList,
    components::{button::route_button_style, tables::*, logos::excalibur_logo},
    execution::form::TransactionSteps,
    screens::execution::{form::FormMessage, processing::StorageDiffs},
    units::address_to_string,
};

static PLACEHOLDER: &str = "Search anything...";

use super::*;
pub fn simple_view<'a>() -> Element<'a, Message> {
    // Container::new(Text::new("Hello, world!"))
    //     .width(Length::Fill)
    //     .height(Length::Fill)
    //     .into()

    let title = container(
        column![
            Text::new("Capy")
                .size(75)
                .font(ICON_FONT),
                // .style(iced::Theme::default()),
            Text::new("Ethereum search engine").size(18)
        ]
        .spacing(15)
        .align_items(Alignment::Center),
    );

    const EXCALIBUR_LOGO: &[u8] = include_bytes!("../static/logos/excalibur_logo.png");
    // let historial_container = if self.searches.is_empty() {
    //     empty_message("You didn't searched anything yet...")
    // } else {
    //     show_historial(&self.searches)
    // };
    let test_querry = "0x0dcd2f752394c41875e259e00bb44fd505297caf";
    let input_and_button = container(
        row![
            text_input(PLACEHOLDER, &test_querry)
                .on_input(Message::SearchInputChanged)
                .padding([12, 20]),
            button(icon('\u{F144}', 16))
                .height(30)
                .width(30)
                .padding(6.2)
                .style(iced::theme::Button::Custom(Box::new(components::button::CustomButtonStyle::new())))
                .on_press(Message::OnPressing),
        ]
        .width(595)
        .align_items(Alignment::Center),
    )
    .width(610)
    .center_x()
    .center_y()
    .style(InfoContainer::theme());

    let principal_container = container(
        column![title, input_and_button]
            .align_items(Alignment::Center)
            .spacing(30),
    );

    let principal_box = container(container(
        column![
            principal_container,
            horizontal_rule(1),
            // historial_container
        ]
        .align_items(Alignment::Center)
        .spacing(15),
    ))
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y();

    principal_box.into()
}

fn icon(unicode: char, size: impl Into<Pixels>) -> Text<'static> {
    text(unicode.to_string())
        .font(ICON_FONT)
        .horizontal_alignment(Horizontal::Center)
        .size(size)
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

// fn view(&self) -> Element<Message> {
//     static PLACEHOLDERS: [&str; 3] = [
//         "Search anything...",
//         "Give me your question...",
//         "Let step on that errors...",
//     ];
//     let mut rng = thread_rng();

//     let placeholder = *PLACEHOLDERS.choose(&mut rng).unwrap();

//     let title = container(
//         column![
//             Text::new("Capy")
//                 .size(75)
//                 .font(BOLD_FONT)
//                 .style(ModernColor::Custom(252.0, 187.0, 150.0)),
//             Text::new("Programmer search engine").size(18)
//         ]
//         .spacing(15)
//         .align_items(Alignment::Center),
//     );

//     let tags = row(vec![
//         itag(
//             "images/stack-overflow.png",
//             (252.0, 187.0, 150.0),
//             Message::TagSelected("overflow".into()),
//         )
//         .into(),
//         itag(
//             "images/stack-exchange.png",
//             (175.0, 197.0, 226.0),
//             Message::TagSelected("exchange".into()),
//         )
//         .into(),
//         button(icon('\u{F1D2}', 16))
//             .height(30)
//             .width(30)
//             .padding(6.2)
//             .on_press(Message::OnChangingTheme)
//             .style(ModernButton::Secondary)
//             .into(),
//     ])
//     .spacing(10);

//     let input_and_button = container(
//         row![
//             text_input(placeholder, &self.inputs.query)
//                 .on_input(Message::QueryChange)
//                 .padding([12, 20]),
//             button(icon('\u{F144}', 16))
//                 .height(30)
//                 .width(30)
//                 .padding(6.2)
//                 .style(if self.inputs.query.trim().is_empty() {
//                     ModernButton::Secondary
//                 } else {
//                     ModernButton::Principal
//                 })
//                 .on_press(Message::OnPressing),
//         ]
//         .width(595)
//         .align_items(Alignment::Center),
//     )
//     .width(610)
//     .center_x()
//     .center_y()
//     .style(ModernContainer::Input);

//     let principal_container: container::Container<Message, Renderer> = container(
//         column![title, input_and_button]
//             .align_items(Alignment::Center)
//             .spacing(30),
//     );

//     // let principal_box = container(container(
//     //     column![
//     //         principal_container,
//     //         tags,
//     //         horizontal_rule(1),
//     //         historial_container
//     //     ]
//     //     .align_items(Alignment::Center)
//     //     .spacing(15),
//     // ))
//     // .padding(10)
//     // .width(Length::Fill)
//     // .height(Length::Fill)
//     // .center_x()
//     // .center_y();

//     // principal_box.into()
// }

// fn historial_text(query: &str, id: usize) -> Element<'static, Message> {
//     Row::new()
//         .push(
//             button(
//                 text(query)
//                     .size(18)
//                     .style(ModernColor::Custom(160.0, 160.0, 160.0)),
//             )
//             .style(ModernButton::Text)
//             .on_press(Message::SetSearch(query.to_string())),
//         )
//         .push(horizontal_space(10))
//         .push(
//             button(icon('\u{F62A}', 18).style(ModernColor::Custom(160.0, 160.0, 160.0)))
//                 .on_press(Message::RemoveSearch(id))
//                 .style(ModernButton::Text),
//         )
//         .align_items(Alignment::Center)
//         .into()
// }

// fn show_historial(queries: &[String]) -> modern::modern_widget::Container<'static, Message> {
//     let data: Vec<Element<Message>> = queries
//         .iter()
//         .enumerate()
//         .map(|(id, q)| historial_text(q.trim(), id))
//         .collect();
//     container(
//         scrollable(
//             column(data)
//                 .padding([20, 30])
//                 .align_items(Alignment::Start)
//                 .spacing(5),
//         )
//         .width(580),
//     )
//     .width(610)
//     .height(200)
//     .style(ModernContainer::Historial)
// }

// fn empty_message(msg: &str) -> Container<'_, Message, Renderer> {
//     container(
//         text(msg)
//             .width(Length::Fill)
//             .size(20)
//             .vertical_alignment(Vertical::Center)
//             .horizontal_alignment(Horizontal::Center)
//             .style(ModernColor::Custom(82.0, 81.0, 90.0)),
//     )
//     .width(610)
//     .height(200)
//     .center_x()
//     .center_y()
//     .style(ModernContainer::Historial)
// }