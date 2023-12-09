//! Combines components into more complex components.
//! All modules in this directory are either underlying component wrappers or
//! the system module that defines most of the styling system.
//!
//! All the components that combine these parts and the styling system live in
//! here.

pub mod button;
pub mod chart;
pub mod input;
pub mod logos;
pub mod progress;
pub mod select;
pub mod styles;
pub mod system;
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
    select::custom_pick_list,
    system::{label, panel, Card, ExcaliburButton, ExcaliburColor, ExcaliburContainer},
    tables::{builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, rows::RowBuilder},
};
// These components should return View messages.
use super::{view::Message, *};

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
            .style(ExcaliburContainer::indicator().theme())
            .height(Length::Fixed(2.0))
    } else {
        container(Column::new()).height(Length::Fixed(2.0))
    };

    Column::new()
        .push(elem)
        .push(indicator.width(Length::Fixed(100.0)))
        .align_items(alignment::Alignment::Center)
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
    text: String,
    value: Option<String>,
    _placeholder: String,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> Column<'a, Message>
where
    Message: 'static,
{
    let title = label(&text).secondary().build();
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
    let title = label(&title).title3().build();

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
    label(&value).highlight().build()
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

/// Renders an instructions title, ctaription, an action button and feedback
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
    Message: 'a + Clone + Default,
{
    let mut inner: Column<'a, Message> = Column::new()
        .spacing(Sizes::Sm)
        .padding(Sizes::Sm)
        .width(Length::Fill)
        .push(label("Instructions").title2().build());

    for instruction in instructions {
        inner = inner.push(instruction.into());
    }

    let mut submit: Button<'a, Message> = ExcaliburButton::new()
        .primary()
        .build(label(&action.unwrap_or_else(|| "Submit".to_string())).build())
        .padding(Sizes::Md)
        .width(Length::Fill);

    if let Some(on_submit) = on_submit {
        submit = submit.on_press(on_submit)
    }

    let feedback = label(&feedback.unwrap_or_default())
        .highlight()
        .build()
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
    let key = label(&key).secondary().build();
    let value = label(&value).build();
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
    pub cta: String,
    pub on_press: Message,
    pub active: bool,
    pub disabled: bool,
}

impl<Message> NavigationStep<Message>
where
    Message: Clone + Default,
{
    /// Creates a new navigation step.
    pub fn new(icon: Icon, cta: &str, on_press: Message, active: bool, disabled: bool) -> Self {
        Self {
            icon,
            cta: cta.to_string(),
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
            cta: "Default".to_string(),
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
    let mut content = Column::new().push(label(&title).title3().build());

    for NavigationStep {
        icon,
        cta,
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
                    .style(ExcaliburContainer::indicator().theme()),
            );
        }

        row = row
            .push(text(icon_to_char(icon)).font(ICON_FONT))
            .push(label(&cta).title3().build());

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
