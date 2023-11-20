pub mod button;
pub mod containers;
pub mod exit;
pub mod input;
pub mod logos;
pub mod styles;
pub mod tables;

use button::*;
use iced::{
    widget::{pick_list, Container},
    Color, Element, Renderer,
};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};
use input::*;
use styles::*;

use self::containers::{CardContainer, MenuContainerTheme, ScreenWindowContainer, WindowHeader};
// These components should return View messages.
use super::{
    view::{Message, Page},
    *,
};

/// Renders a gray text label in lowercase.
pub fn label_item<'a>(t: String) -> Text<'a> {
    let content = t.to_lowercase();
    text(content).size(16).style(Color::from_rgb(0.5, 0.5, 0.5))
}

/// Renders white text in the DAGGERSQUARE font.
pub fn data_item<'a>(t: String) -> Text<'a> {
    text(t).font(FONT_DAGGERSQUARE).style(Color::WHITE)
}

/// Renders a column with a label and an element.
pub fn labeled<'a, T: Into<Element<'a, Message>>>(
    label: String,
    element: T,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label)).push(element.into());
    content = content.spacing(8);
    content.into()
}

/// Renders a row of labeled controls, where each control has a label.
pub fn labeled_controls<'a, T: Into<Element<'a, Message>>>(
    controls: Vec<(String, T)>,
) -> Element<'a, Message> {
    let mut content = Row::new();
    for (label, control) in controls {
        content = content.push(labeled(label, control));
    }
    content.spacing(4).into()
}

/// Renders a column with a label and a piece of data with the DAGGERSQUARE
/// font.
pub fn labeled_data<'a>(label: String, data: String) -> Element<'a, Message, Renderer> {
    let mut content = Column::new()
        .push(label_item(label))
        .push(data_item(data).size(20));
    content = content.spacing(8);
    content.into()
}

/// Renders a nice blue button.
pub fn action_button<'a>(label: String) -> iced::widget::Button<'a, Message> {
    let content = text(label)
        .size(16)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .style(Color::WHITE);
    let action_button_style = CustomButtonStyle::new()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(35, 88, 226))
        .hovered()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(88, 135, 255))
        .pressed()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(11, 63, 197))
        .disabled()
        .border_radius(5.0.into())
        .background_color(DISABLED_COLOR)
        .text_color(DISABLED_TEXT_GRAY);
    button(content).style(action_button_style.as_custom())
}

/// Renders a nice red button.
pub fn destructive_button<'a>(label: String) -> iced::widget::Button<'a, Message> {
    let content = text(label)
        .size(16)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .style(Color::WHITE);
    let destructive_button_style = CustomButtonStyle::new()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(228, 75, 65))
        .hovered()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(189, 39, 29))
        .pressed()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(200, 39, 30));
    button(content).style(destructive_button_style.as_custom())
}

/// Container that groups actions or settings with a label and a row of
/// controls.
pub fn controls_container<'a, T: Into<Element<'a, Message>>>(
    label: String,
    actions: Vec<T>,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label));
    let mut row = Row::new().spacing(4);
    for action in actions {
        row = row.push(action.into());
    }
    content = content.push(row);
    content.spacing(8).into()
}

/// Containers that groups multiple labeled data pieces under a label
pub fn labeled_data_container<'a>(
    label: String,
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label));
    content = content.push(labeled_data_row(data, max_elements));
    content.into()
}

/// Renders a row of labeled data elements using labeled_data. Specify the
/// maximum amount of elements in the row, if the total amount of elements
/// exceeds the value, it will push a new row to the column.
pub fn labeled_data_row<'a>(
    label_data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message, Renderer> {
    let mut content = Column::new();
    let mut row = Row::new().spacing(4);
    let mut i = 0;
    for (label, data) in label_data {
        row = row.push(labeled_data(label, data));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(4);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(8).into()
}

/// A container that spaces the elements in a row out so they fill the space.
pub fn space_between_row<'a, T: Into<Element<'a, Message, Renderer>>>(
    elements: Vec<T>,
) -> Element<'a, Message> {
    let mut content = Row::new()
        .spacing(8)
        .width(Length::Shrink)
        .align_items(alignment::Alignment::Center);

    // The first element should be wrapped in a column that has align_items with
    // Start The last element should have the same but with End alignment.
    // All other elements should have center alignment.
    let len = elements.len();
    for (i, element) in elements.into_iter().enumerate() {
        content = content.push(
            Column::new()
                .align_items(match i {
                    0 => alignment::Alignment::Start,
                    _ if i == len - 1 => alignment::Alignment::End,
                    _ => alignment::Alignment::Center,
                })
                .push(element.into())
                .width(Length::FillPortion(1)),
        );
    }
    content.into()
}

/// Creates a row of two 50% width columns with the given elements.
/// todo: replace proper spacing and padding sizes.
pub fn dual_column<'a, T: Into<Element<'a, Message>>>(
    first_column: Vec<T>,
    second_column: Vec<T>,
) -> Row<'a, Message> {
    let first_column = Column::with_children(first_column.into_iter().map(|e| e.into()).collect())
        .width(Length::FillPortion(2))
        .spacing(Sizes::Md as u16)
        .align_items(alignment::Alignment::Start);

    let second_column =
        Column::with_children(second_column.into_iter().map(|e| e.into()).collect())
            .width(Length::FillPortion(2))
            .spacing(Sizes::Md as u16)
            .align_items(alignment::Alignment::End);

    Row::new()
        .spacing(Sizes::Md as u16)
        .push(first_column)
        .push(second_column)
}

pub fn title_large<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleLg).into()
}

pub fn title_medium<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleMd).into()
}

pub fn title_small<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleSm).into()
}

pub fn body_text<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Md).into()
}

pub fn caption<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs).into()
}

pub fn h1<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleSm).into()
}

pub fn h2<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Lg).into()
}

pub fn h3<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Md).into()
}

pub fn h4<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm).into()
}

pub fn h5<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs).into()
}

pub fn paragraph<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm).into()
}

pub fn primary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(PRIMARY_LABEL_COLOR)
        .into()
}

pub fn secondary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(SECONDARY_LABEL_COLOR)
        .into()
}

pub fn tertiary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(TERTIARY_LABEL_COLOR)
        .into()
}

pub fn quaternary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(QUATERNARY_LABEL_COLOR)
        .into()
}

/// todo: remove label item
pub fn highlight_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Xs)
        .style(SECONDARY_COLOR)
        .into()
}

pub fn highlight_secondary_label<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs).style(BLUE_400).into()
}

pub fn with_font<'a>(value: Text<'a>) -> Text<'a> {
    value.font(FONT_DAGGERSQUARE)
}

/// Card is just a container with a background color and some border radius.
pub struct Card;

impl Card {
    pub fn new<'a, T: Into<Element<'a, Message>>>(content: T) -> Container<'a, Message> {
        let content = content.into();
        Container::new(content).style(CardContainer::theme())
    }
}

/// note: the header needs to fill the container. but this pushes the content
/// out to its max width.
/// so we need to cap the window to a max width, which we should improve on in
/// the future.
pub fn screen_window<'a, T: Into<Element<'a, Message>>>(
    window: &'a Page,
    content: T,
) -> Container<'a, Message, Renderer> {
    let name = window.name().clone();
    Container::new(
        Column::new()
            .push(
                Container::new(
                    Row::new()
                        .align_items(alignment::Alignment::Center)
                        .push(
                            Column::new()
                                .push(with_font(h1(name)))
                                .width(Length::FillPortion(2)),
                        )
                        .push(
                            Column::new()
                                .push(text(icon_to_char(Icon::BookmarkFill)).font(ICON_FONT))
                                .align_items(alignment::Alignment::End)
                                .width(Length::FillPortion(2)),
                        )
                        .padding(Sizes::Lg as u16),
                )
                .style(WindowHeader::theme()),
            )
            .push(Row::new().push(content))
            .spacing(Sizes::Md as u16),
    )
    .max_height(ByteScale::Xl7 as u16)
    .max_width(ByteScale::Xl7 as u16)
    .style(ScreenWindowContainer::theme())
    .into()
}

/// Column with a label and text input field.
pub fn input_group<'a>(
    title: String,
    value: Option<String>,
    placeholder: String,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> Column<'a, Message> {
    let title = h3(title.to_string());
    // todo: change this so padding is modifiable.
    let input = create_input_component(value, on_change);

    Column::new()
        .push(title)
        .push(input)
        .width(Length::Shrink)
        .spacing(Sizes::Md as u16)
}

/// Column with a label and pick list field.
pub fn select_group<'a>(
    title: String,
    options: Vec<String>,
    selected: String,
    on_selected: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let title = h3(title.to_string());
    let input = pick_list(options, Some(selected.clone()), on_selected).padding(Sizes::Md as u16);

    let input_container = Container::new(input).style(MenuContainerTheme::theme());

    Column::new()
        .push(title)
        .push(input_container)
        .width(Length::Fill)
        .spacing(Sizes::Md as u16)
        .into()
}
