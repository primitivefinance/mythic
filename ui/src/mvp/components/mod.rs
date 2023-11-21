pub mod button;
pub mod containers;
pub mod input;
pub mod logos;
pub mod select;
pub mod styles;
pub mod tables;

use button::*;
use iced::{
    widget::{pick_list, Container},
    Color, Element, Renderer,
};
use iced_aw::Icon;
use input::*;
use styles::*;

use self::{
    containers::{CardContainer, ScreenWindowContainer, WindowHeader},
    select::custom_pick_list,
};
// These components should return View messages.
use super::{
    view::{control::custom_icon_button, Message, Page},
    *,
};

/// Renders a gray text label in lowercase.
pub fn label_item<'a>(t: String) -> Text<'a> {
    secondary_label(t).size(TextSize::Md as u16)
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
    let mut content = Column::new()
        .push(label_item(label))
        .push(container(element).center_y());
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
    content.spacing(Sizes::Md as u16).into()
}

/// Renders a column with a label and a piece of data with the DAGGERSQUARE
/// font.
pub fn labeled_data<'a>(label: String, data: String) -> Element<'a, Message, Renderer> {
    // If data is a value above > 1000, replace the last three zeros with an
    // uppercase "K". Same with > 1_000_000 "M", etc.
    let data = match data.parse::<f64>() {
        Ok(value) => {
            if value > 1_000_000.0 {
                format!("{:.2}M", value / 1_000_000.0)
            } else if value > 1000.0 {
                format!("{:.2}K", value / 1000.0)
            } else {
                data
            }
        }
        Err(_) => data,
    };

    let mut content = Column::new()
        .push(secondary_label(label).size(TextSize::Lg as u16))
        .push(highlight_label(data).size(TitleSize::Md as u16));
    content = content.spacing(Sizes::Sm as u16);
    content.into()
}

/// Renders a nice blue button.
pub fn action_button<'a>(label: String) -> iced::widget::Button<'a, Message> {
    // todo: need to specify almost every style because the default button style
    // doesn't work. we can just update the custom button style struct we made
    // to use our own default style.
    let action_button_style = CustomButtonStyle::new()
        .text_color(Color::WHITE)
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(35, 88, 226))
        .hovered()
        .text_color(Color::WHITE)
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(88, 135, 255))
        .pressed()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(11, 63, 197))
        .disabled()
        .border_radius(5.0.into())
        .background_color(DISABLED_COLOR)
        .text_color(DISABLED_TEXT_GRAY);
    button(
        text(label)
            .horizontal_alignment(alignment::Horizontal::Center)
            .width(Length::Fill),
    )
    .style(action_button_style.as_custom())
}

/// Renders a nice red button.
#[allow(dead_code)]
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
    let mut row = Row::new()
        .spacing(4)
        .align_items(iced::alignment::Alignment::Center);
    for action in actions {
        row = row.push(action.into());
    }
    content = content.push(row);
    content.spacing(8).into()
}

/// Containers that groups multiple labeled data pieces under a label
pub fn labeled_data_container<'a>(
    _label: String,
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message> {
    let mut content = Column::new();
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
    let mut row = Row::new().spacing(Sizes::Lg as u16);
    let mut i = 0;
    for (label, data) in label_data {
        row = row.push(labeled_data(label, data));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(Sizes::Lg as u16);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(Sizes::Lg as u16).into()
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

#[allow(dead_code)]
pub fn title_large<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleLg).into()
}

#[allow(dead_code)]
pub fn title_medium<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleMd).into()
}

#[allow(dead_code)]
pub fn title_small<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleSm).into()
}

#[allow(dead_code)]
pub fn body_text<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Md).into()
}

#[allow(dead_code)]
pub fn caption<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs).into()
}

pub fn h1<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleSm).into()
}

pub fn h2<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Lg).into()
}

#[allow(dead_code)]
pub fn h3<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Md).into()
}

#[allow(dead_code)]
pub fn h4<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm).into()
}

#[allow(dead_code)]
pub fn h5<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs).into()
}

#[allow(dead_code)]
pub fn paragraph<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm).into()
}

#[allow(dead_code)]
pub fn primary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(PRIMARY_LABEL_COLOR)
        .into()
}

#[allow(dead_code)]
pub fn secondary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(SECONDARY_LABEL_COLOR)
        .into()
}

#[allow(dead_code)]
pub fn tertiary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(TERTIARY_LABEL_COLOR)
        .into()
}

#[allow(dead_code)]
pub fn quaternary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(QUATERNARY_LABEL_COLOR)
        .into()
}

/// todo: remove label item
#[allow(dead_code)]
pub fn highlight_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Xs)
        .style(SECONDARY_COLOR)
        .into()
}

#[allow(dead_code)]
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
                                .push(
                                    custom_icon_button(Icon::X, Sizes::Md as u16)
                                        .on_press(view::Message::Page(view::Page::Empty)),
                                )
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
    _placeholder: String,
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
    selected: Option<String>,
    on_selected: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let title = h3(title.to_string());
    let input = custom_pick_list(options, selected.clone(), on_selected, None)
        .padding(Sizes::Md as u16)
        .width(Length::Fill);

    // let input_container =
    // Container::new(input).style(MenuContainerTheme::theme());

    Column::new()
        .push(title)
        .push(input)
        .spacing(Sizes::Md as u16)
        .into()
}

pub fn copyable_text<'a, E: Into<Element<'a, view::Message>>>(
    label: E,
    value: String,
) -> iced::widget::Button<'a, view::Message> {
    let copy_button = button(label)
        .style(
            CustomButtonStyle::text(&iced::Theme::Dark)
                .hovered()
                .border_radius(5.0.into())
                .background(Some(SEMI_TRANSPARENT_HIGHLIGHT_CONTAINER.into()))
                .pressed()
                .border_radius(5.0.into())
                .background(Some(MENU_BG_COLOR.into()))
                .as_custom(),
        )
        .padding(0)
        .on_press(view::Message::CopyToClipboard(value));
    copy_button
}
