//! Exposes a builder for constructing cells for tables.
//!
//! Set the `on_change` field to render an input.
//! Set the `on_submit` field to render an input that submits on enter.
//! Set the `selected` field to render a checkbox.
//! Set the `child` field to render a custom component.
//! Set none of the above to render a static text cell that styles it as a
//! label.
//!
//! # Example
//!
//! ```
//! use components::tables::cells::CellBuilder;
//! use iced::{button, Button, Column, Element, Text};
//!
//! let cell = CellBuilder::new()
//!     .value(Some("Value".to_string()))
//!     .on_change(|value| Message::Empty)
//!     .on_submit(|value| Message::Empty)
//!     .placeholder(Some("Placeholder".to_string()))
//!     .internal_padding(Some(Sizes::Lg))
//!     .selected(Some(true))
//!     .build();
//! ```

use iced::{
    widget::{component, text_input, Checkbox, Component, Container},
    Element, Renderer,
};

use super::components::*;
use crate::components::containers::BorderedContainer;

pub mod input;
pub mod select;
pub mod toggle;

pub fn cell_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message>
where
    Message: 'a,
{
    Container::new(content.into())
        .width(Length::Fill)
        .center_x()
        .center_y()
}

/// Constructs a cell for a table.
pub struct CellBuilder<Message>
where
    Message: Default,
{
    value: Option<String>,
    child: Option<Element<'static, Message>>,
    on_change: Option<Box<dyn Fn(Option<String>) -> Message>>,
    on_submit: Option<Box<dyn Fn(Option<String>) -> Message>>,
    on_select: Option<Box<dyn Fn(String) -> Message>>,
    on_checkbox: Option<Box<dyn Fn(bool) -> Message>>,
    placeholder: Option<String>,
    external_padding: Option<Sizes>,
    internal_padding: Option<Sizes>,
    selected: Option<String>,
    checked: Option<bool>,
    options: Option<Vec<String>>,
    // todo: this might really impact performance.
    containerize: Box<dyn Fn(Element<'static, Message>) -> Container<'static, Message>>,
}

impl<Message> Default for CellBuilder<Message>
where
    Message: 'static + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> CellBuilder<Message>
where
    Message: 'static + Default,
{
    pub fn new() -> Self {
        Self {
            value: None,
            child: None,
            on_change: None,
            on_submit: None,
            on_checkbox: None,
            on_select: None,
            placeholder: None,
            external_padding: None,
            internal_padding: None,
            checked: None,
            selected: None,
            options: None,
            containerize: Box::new(cell_container),
        }
    }

    pub fn update_value(&mut self, value: Option<String>) {
        self.value = value;
    }

    pub fn child(mut self, child: impl Into<Element<'static, Message>>) -> Self {
        self.child = Some(child.into());
        self
    }

    pub fn value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    pub fn options(mut self, options: Option<Vec<String>>) -> Self {
        self.options = options;
        self
    }

    pub fn on_change(mut self, on_change: impl Fn(Option<String>) -> Message + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    pub fn on_submit(mut self, on_submit: impl Fn(Option<String>) -> Message + 'static) -> Self {
        self.on_submit = Some(Box::new(on_submit));
        self
    }

    pub fn on_checkbox(mut self, on_checkbox: impl Fn(bool) -> Message + 'static) -> Self {
        self.on_checkbox = Some(Box::new(on_checkbox));
        self
    }

    pub fn on_select(mut self, on_select: impl Fn(String) -> Message + 'static) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    pub fn checked(mut self, checked: Option<bool>) -> Self {
        self.checked = checked;
        self
    }

    pub fn placeholder(mut self, placeholder: Option<String>) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn external_padding(mut self, external_padding: Option<Sizes>) -> Self {
        self.external_padding = external_padding;
        self.containerize =
            Box::new(move |e| (self.containerize)(e).padding(external_padding.unwrap_or_default()));
        self
    }

    pub fn internal_padding(mut self, internal_padding: Option<Sizes>) -> Self {
        self.internal_padding = internal_padding;
        self
    }

    pub fn selected(mut self, selected: Option<String>) -> Self {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: impl Fn() -> iced::theme::Container + 'static) -> Self {
        self.containerize = Box::new(move |e| cell_container(e).style(style()));
        self
    }

    pub fn build<'a>(self) -> impl Into<Container<'a, Message>>
    where
        Message: 'a + Default,
    {
        let value = self.value.as_ref().cloned();

        // If on_checkbox is Some, then we need to render a checkbox.
        if self.on_checkbox.is_some() {
            // how do we handle the case where checked is not ever set?
            let checkbox = Checkbox::new(
                self.value.unwrap_or_default(),
                self.checked.unwrap_or_default(),
                self.on_checkbox
                    .unwrap_or_else(|| Box::new(|_| Message::default())),
            );

            return (self.containerize)(checkbox.into());
        }

        let internal_padding: Padding = self.internal_padding.unwrap_or_default().into();
        let cell_content = match self.child {
            Some(child) => Column::new().push(child).padding(internal_padding).into(),
            None => Column::new()
                .push(primary_label(value.clone().unwrap_or_default()))
                .padding(internal_padding)
                .into(),
        };

        // If options is Some, then we need to render a select.
        if let Some(options) = self.options {
            let select = select::cell_select(
                options,
                self.selected,
                self.on_select
                    .unwrap_or_else(|| Box::new(|_| Message::default())),
                self.placeholder,
            );

            return (self.containerize)(select.into());
        }

        // If on_change is Some, then we need to render an input.
        if let Some(on_change) = self.on_change {
            let input = input::cell_input(
                self.value,
                on_change,
                self.on_submit
                    .unwrap_or_else(|| Box::new(|_| Message::default())),
                self.placeholder,
                self.internal_padding,
            );

            return (self.containerize)(input.into());
        }

        // Else, render a static cell.
        (self.containerize)(cell_content)
    }
}

impl<'a, Message> From<CellBuilder<Message>> for Container<'a, Message, Renderer>
where
    Message: 'static + Default,
{
    fn from(cell_builder: CellBuilder<Message>) -> Self {
        cell_builder.build().into()
    }
}

#[allow(dead_code)]
pub fn dev_cell<'a, Message>(
    value: Option<String>,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> Container<'a, Message>
where
    Message: 'static + Default,
{
    CellBuilder::new()
        .value(value)
        .on_change(on_change)
        .style(BorderedContainer::theme)
        .into()
}
