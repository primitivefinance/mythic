pub mod button;
pub mod chart;
pub mod containers;
pub mod input;
pub mod logos;
pub mod progress;
pub mod select;
pub mod styles;
pub mod tables;

use std::borrow::Cow;

use button::*;
use iced::{
    widget::{pick_list, Button, Container},
    BorderRadius, Color, Element, Padding, Renderer,
};
use iced_aw::{
    graphics::icons::{self, icon_to_char},
    Icon, ICON_FONT,
};
use input::*;
use styles::*;

use self::{
    containers::{CardContainer, CustomContainer, Indicator, ScreenWindowContainer, WindowHeader},
    select::custom_pick_list,
    tables::{builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, rows::RowBuilder},
};
// These components should return View messages.
use super::{
    view::{sidebar::Page, Message},
    *,
};

/// Renders a gray text label in lowercase.
pub fn label_item<'a>(t: String) -> Text<'a> {
    tertiary_label(t).size(TextSize::Md as u16)
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
pub fn labeled_data<'a, Message>(label: String, data: String) -> Element<'a, Message, Renderer>
where
    Message: 'a,
{
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
pub fn action_button<'a, Message>(label: String) -> iced::widget::Button<'a, Message>
where
    Message: 'static,
{
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
pub fn destructive_button<'a, Message>(label: String) -> iced::widget::Button<'a, Message>
where
    Message: 'a,
{
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
        .background_color(Color::from_rgb8(200, 39, 30))
        .disabled()
        .border_radius(5.0.into())
        .background_color(DISABLED_COLOR)
        .text_color(DISABLED_TEXT_GRAY);
    button(content).style(destructive_button_style.as_custom())
}

/// Renders a tab-like button
#[allow(dead_code)]
pub fn tab_button<'a, Message>(active: bool, label: String) -> iced::widget::Button<'a, Message>
where
    Message: 'a,
{
    let content = text(label)
        .size(16)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .style(Color::WHITE);
    let tab_button_style = CustomButtonStyle::new()
        .border_radius(5.0.into())
        .background_color(Color::TRANSPARENT)
        .hovered()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(38, 36, 45))
        .pressed()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(38, 36, 50))
        .disabled()
        .border_radius(5.0.into())
        .background_color(DISABLED_COLOR)
        .text_color(DISABLED_TEXT_GRAY);
    button(with_lower_indicator(
        active,
        Column::new().push(content).padding(Padding {
            top: 12.0,
            right: 5.0,
            bottom: 12.0,
            left: 5.0,
        }),
    ))
    .style(tab_button_style.as_custom())
}

pub fn with_lower_indicator<'a, Message>(
    toggle: bool,
    elem: impl Into<Element<'a, Message>>,
) -> Column<'a, Message>
where
    Message: 'a,
{
    let indicator = if toggle {
        container(Column::new())
            .style(Indicator::theme())
            .height(Length::Fixed(2.0))
    } else {
        container(Column::new()).height(Length::Fixed(2.0))
    };

    Column::new()
        .push(elem)
        .push(indicator.width(Length::Fixed(100.0)))
        .align_items(alignment::Alignment::Center)
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
pub fn labeled_data_container<'a, Message>(
    _label: String,
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new();
    content = content.push(labeled_data_row(data, max_elements));
    content.into()
}

/// Renders a row of labeled data elements using labeled_data. Specify the
/// maximum amount of elements in the row, if the total amount of elements
/// exceeds the value, it will push a new row to the column.
pub fn labeled_data_row<'a, Message>(
    label_data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message, Renderer>
where
    Message: 'a,
{
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
    text(value).size(FontSizes::TitleLg)
}

#[allow(dead_code)]
pub fn title_medium<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleMd)
}

#[allow(dead_code)]
pub fn title_small<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleSm)
}

#[allow(dead_code)]
pub fn body_text<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Md)
}

#[allow(dead_code)]
pub fn caption<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs)
}

pub fn h1<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::TitleSm)
}

pub fn h2<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Lg)
}

#[allow(dead_code)]
pub fn h3<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Md)
}

#[allow(dead_code)]
pub fn h4<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm)
}

#[allow(dead_code)]
pub fn h5<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs)
}

#[allow(dead_code)]
pub fn paragraph<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm)
}

#[allow(dead_code)]
pub fn primary_label<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm).style(PRIMARY_LABEL_COLOR)
}

#[allow(dead_code)]
pub fn secondary_label<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm).style(SECONDARY_LABEL_COLOR)
}

#[allow(dead_code)]
pub fn tertiary_label<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Sm).style(TERTIARY_LABEL_COLOR)
}

#[allow(dead_code)]
pub fn quaternary_label<'a>(value: String) -> Text<'a> {
    text(value)
        .size(FontSizes::Sm)
        .style(QUATERNARY_LABEL_COLOR)
}

/// todo: remove label item
#[allow(dead_code)]
pub fn highlight_label<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs).style(MINT_500)
}

#[allow(dead_code)]
pub fn highlight_secondary_label<'a>(value: String) -> Text<'a> {
    text(value).size(FontSizes::Xs).style(BLUE_400)
}

pub fn with_font(value: Text) -> Text {
    value.font(FONT_DAGGERSQUARE)
}

/// Card is just a container with a background color and some border radius.
pub struct Card {
    background: Option<iced::Background>,
}

impl Card {
    // todo: refactor this to use builder pattern.
    pub fn new<'a, Message, T: Into<Element<'a, Message>>>(content: T) -> Container<'a, Message>
    where
        Message: 'a,
    {
        let content = content.into();
        Container::new(content).style(CardContainer::theme())
    }

    /// Returns a fresh instance of this Card.
    pub fn template() -> Self {
        Self {
            background: Some(iced::Background::Color(BACKGROUND)),
        }
    }

    /// Modifies the background.
    pub fn background(mut self, background: Option<iced::Background>) -> Self {
        self.background = background;
        self
    }

    pub fn build<'a, Message, T: Into<Element<'a, Message>>>(
        self,
        content: T,
        border_radius: BorderRadius,
    ) -> Container<'a, Message>
    where
        Message: 'static,
    {
        let content = content.into();
        Container::new(content).style(CustomContainer::theme_with_border_radius(
            self.background,
            Some(border_radius),
        ))
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
            .push(h2(name))
            .push(Row::new().push(content))
            .spacing(Sizes::Md as u16),
    )
    .max_height(ByteScale::Xl7 as u16)
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
pub fn select_group<'a, Message>(
    title: String,
    options: Vec<String>,
    selected: Option<String>,
    on_selected: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: 'a,
{
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

/// Renders a label and text input inside a column.
pub fn labeled_input<'a, Message>(
    label: String,
    value: Option<String>,
    _placeholder: String,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> Column<'a, Message>
where
    Message: 'static,
{
    let title = label_item(label.to_string());
    // todo: use placeholder
    let input = create_input_component(value, on_change);

    Column::new().push(title).push(input).spacing(Sizes::Md)
}

/// Column with a label and pick list field.
pub fn labeled_select<'a, Message, T>(
    title: String,
    options: impl Into<Cow<'a, [T]>>,
    selected: Option<T>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: 'a,
    T: ToString + Eq + 'static + Clone,
    [T]: ToOwned<Owned = Vec<T>>,
{
    let title = h3(title.to_string());

    Column::new()
        .push(title)
        .push(
            custom_pick_list(options, selected, on_selected, None)
                .padding(Sizes::Md as u16)
                .width(Length::Fill),
        )
        .spacing(Sizes::Md as u16)
        .into()
}

/// For use in the instructions container.
pub fn instruction_text<'a>(value: String) -> Text<'a> {
    highlight_label(value).size(TextSize::Sm as u16)
}

pub fn instructions_inner<'a, Message, T: Into<Element<'a, Message>>>(
    instructions: Vec<T>,
) -> Column<'a, Message>
where
    Message: 'static + Clone,
{
    let mut inner: Column<'a, Message> = Column::new()
        .spacing(Sizes::Sm)
        .padding(Sizes::Sm)
        .width(Length::Fill);

    for instruction in instructions {
        inner = inner.push(instruction);
    }

    inner
}

/// Renders an instructions title, description, an action button and feedback
/// in a card.
/// note: Message must be `Clone` for the submit button to be converted to an
/// Element.
pub fn instructions<'a, Message, T: Into<Element<'a, Message>>>(
    instructions: Vec<T>,
    action: Option<String>,
    feedback: Option<String>,
    on_submit: Option<Message>,
) -> Container<'a, Message>
where
    Message: 'static + Clone,
{
    let mut inner: Column<'a, Message> = Column::new()
        .spacing(Sizes::Sm)
        .padding(Sizes::Sm)
        .width(Length::Fill)
        .push(h2("Instructions".to_string()));

    for instruction in instructions {
        inner = inner.push(instruction.into());
    }

    let mut submit: Button<'a, Message> =
        action_button(action.unwrap_or_else(|| "Submit".to_string()).to_string())
            .padding(Sizes::Md)
            .width(Length::Fill);

    if let Some(on_submit) = on_submit {
        submit = submit.on_press(on_submit)
    }

    let feedback = highlight_label(feedback.unwrap_or_default().to_string())
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);

    Card::new(
        Column::new()
            .push(inner)
            .push(submit)
            .push(feedback)
            .spacing(Sizes::Md)
            .padding(Sizes::Md),
    )
}

/// Renders a table with static data.
/// Message needs to implement Default.
pub fn static_table<'a, Message>(
    title: String,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
) -> Column<'a, Message>
where
    Message: 'static + Default,
{
    Column::new()
        .spacing(Sizes::Md)
        .push(label_item(title))
        .push(
            TableBuilder::new()
                .column(
                    ColumnBuilder::new().headers(headers).rows(
                        data.into_iter()
                            .map(|row| {
                                RowBuilder::new()
                                    .style(|| {
                                        CustomContainer::theme(Some(iced::Background::Color(
                                            GRAY_600,
                                        )))
                                    })
                                    .cells(
                                        row.into_iter()
                                            .map(|cell| {
                                                CellBuilder::new().child(primary_label(cell))
                                            })
                                            .collect(),
                                    )
                            })
                            .collect(),
                    ),
                )
                .padding_cell(Sizes::Xs)
                .build(),
        )
}

pub struct DualColumn<'a, Message>
where
    Message: 'a,
{
    pub column_1: Vec<Element<'a, Message>>,
    pub column_2: Vec<Element<'a, Message>>,
    pub spacing: Option<Sizes>,
    pub padding: Option<Padding>,
    pub column_1_alignment: Option<alignment::Alignment>,
    pub column_2_alignment: Option<alignment::Alignment>,
}

impl<'a, Message> Default for DualColumn<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> DualColumn<'a, Message> {
    pub fn new() -> Self {
        Self {
            column_1: vec![],
            column_2: vec![],
            spacing: None,
            padding: None,
            column_1_alignment: None,
            column_2_alignment: None,
        }
    }

    pub fn column_1_alignment(mut self, alignment: alignment::Alignment) -> Self {
        self.column_1_alignment = Some(alignment);
        self
    }

    pub fn column_2_alignment(mut self, alignment: alignment::Alignment) -> Self {
        self.column_2_alignment = Some(alignment);
        self
    }

    pub fn column_1(mut self, column_1: Vec<Element<'a, Message>>) -> Self {
        self.column_1 = column_1;
        self
    }

    pub fn column_2(mut self, column_2: Vec<Element<'a, Message>>) -> Self {
        self.column_2 = column_2;
        self
    }

    pub fn columns(
        mut self,
        column_1: Vec<Element<'a, Message>>,
        column_2: Vec<Element<'a, Message>>,
    ) -> Self {
        self.column_1 = column_1;
        self.column_2 = column_2;
        self
    }

    pub fn spacing(mut self, spacing: Sizes) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn build(self) -> Row<'a, Message> {
        let mut row = Row::new();

        let mut first_column = Column::with_children(self.column_1.into_iter().collect())
            .width(Length::FillPortion(2))
            .align_items(
                self.column_1_alignment
                    .unwrap_or(alignment::Alignment::Start),
            );

        let mut second_column = Column::with_children(self.column_2.into_iter().collect())
            .width(Length::FillPortion(2))
            .align_items(self.column_2_alignment.unwrap_or(alignment::Alignment::End));

        if let Some(spacing) = self.spacing {
            first_column = first_column.spacing(spacing);
            second_column = second_column.spacing(spacing);
        }

        if let Some(padding) = self.padding {
            first_column = first_column.padding(padding);
            second_column = second_column.padding(padding);
        }

        row = row.push(first_column);
        row = row.push(second_column);
        row
    }
}

impl<'a, Message> From<DualColumn<'a, Message>> for Row<'a, Message> {
    fn from(dual_column: DualColumn<'a, Message>) -> Self {
        dual_column.build()
    }
}

pub fn key_value_row<'a, Message>(key: String, value: String) -> Row<'a, Message>
where
    Message: 'a,
{
    let key = label_item(key);
    let value = primary_label(value);
    let mut row = Row::new()
        .push(
            Column::new()
                .width(Length::FillPortion(2))
                .align_items(alignment::Alignment::Start)
                .push(key),
        )
        .push(
            Column::new()
                .width(Length::FillPortion(2))
                .align_items(alignment::Alignment::End)
                .push(value),
        );
    row = row
        .spacing(Sizes::Md as u16)
        .align_items(alignment::Alignment::Center);
    row
}

pub fn custom_icon_button<'a>(
    icon: icons::Icon,
    font_size: u16,
) -> iced::widget::Button<'a, Message> {
    let content = text(icon_to_char(icon))
        .font(ICON_FONT)
        .size(font_size)
        .style(Color::WHITE);
    let control_button_style = CustomButtonStyle::new()
        .background_color(Color::TRANSPARENT)
        .hovered()
        .background_color(PRIMARY_COLOR)
        .border_radius(5.0.into());
    button(content).style(control_button_style.as_custom())
}

/// An individual navigation step that can be rendered in a list of steps.
#[derive(Debug, Clone)]
pub struct NavigationStep<Message>
where
    Message: Clone + Default,
{
    pub icon: Icon,
    pub label: String,
    pub on_press: Message,
    pub active: bool,
    pub disabled: bool,
}

impl<Message> NavigationStep<Message>
where
    Message: Clone + Default,
{
    /// Creates a new navigation step.
    pub fn new(icon: Icon, label: &str, on_press: Message, active: bool, disabled: bool) -> Self {
        Self {
            icon,
            label: label.to_string(),
            on_press,
            active,
            disabled,
        }
    }
}

impl Default for NavigationStep<Message> {
    fn default() -> Self {
        Self {
            icon: Icon::Check,
            label: "Default".to_string(),
            on_press: Message::default(),
            active: false,
            disabled: false,
        }
    }
}

/// Renders a list of navigation steps with a custom icon, label, active flag,
/// and message to emit.
pub fn navigation_steps<'a, Message>(
    title: &str,
    steps: Vec<NavigationStep<Message>>,
) -> Column<'a, Message>
where
    Message: 'a + Clone + Default,
{
    let mut content = Column::new().push(h3(title.to_string()));

    for NavigationStep {
        icon,
        label,
        on_press,
        active,
        disabled,
    } in steps.into_iter()
    {
        let mut row = Row::new()
            .spacing(Sizes::Sm)
            .align_items(alignment::Alignment::Center);
        if active {
            row = row.push(
                container(Column::new())
                    .width(Length::Fixed(Sizes::Xs.into()))
                    .height(Length::Fixed(Sizes::Xl.into()))
                    .style(Indicator::theme()),
            );
        }

        row = row
            .push(text(icon_to_char(icon)).font(ICON_FONT))
            .push(h3(label));

        let bg_color = match active {
            true => SELECTED_CONTAINER_COLOR,
            false => Color::TRANSPARENT,
        };

        let mut row = button(row)
            .padding(Sizes::Sm)
            .style(route_button_style(bg_color).as_custom())
            .width(Length::Fill);

        // Disable the button if it has an empty message.
        if !disabled {
            row = row.on_press(on_press);
        }

        content = content.push(row);
    }

    content.spacing(Sizes::Sm)
}
