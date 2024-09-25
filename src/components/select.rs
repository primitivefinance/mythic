//! All custom select components and widgets.

use std::{
    borrow::{Borrow, Cow},
    rc::Rc,
};

use iced::{widget::pick_list::*, Background};

use super::*;

pub fn custom_pick_list<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
    placeholder: Option<String>,
) -> PickList<'a, T, L, V, Message>
where
    T: ToString + Eq + 'static + Clone,
    [T]: ToOwned<Owned = Vec<T>>,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
{
    pick_list(options, selected, on_selected)
        .style(|theme, status| {
            CustomSelect::new()
                .hovered()
                .background(HIGHLIGHTED_CONTAINER_COLOR.into())
                .style_fn(theme, status)
        })
        .placeholder(placeholder.unwrap_or("Select an option".to_string()))
}
#[allow(dead_code)]
pub fn excalibur_select<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
    placeholder: impl ToString,
    border_radius: Option<iced::border::Radius>,
) -> PickList<'a, T, L, V, Message>
where
    T: ToString + Eq + 'static + Clone,
    [T]: ToOwned<Owned = Vec<T>>,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
{
    pick_list(options, selected, on_selected)
        .style(move |theme, status| {
            CustomSelect::new()
                .border_color(ExcaliburColor::Custom(GRAY_600).into())
                .border_width(1.0)
                .border_radius(border_radius.unwrap_or(5.0.into()))
                .text_color(ExcaliburColor::Label(system::LabelColors::Highlight).into())
                .background(ExcaliburColor::Background3.into())
                .placeholder_color(ExcaliburColor::Label(system::LabelColors::Tertiary).into())
                .hovered()
                .border_color(ExcaliburColor::Custom(GRAY_600).into())
                .border_width(1.0)
                .border_radius(border_radius.unwrap_or(5.0.into()))
                .text_color(ExcaliburColor::Label(system::LabelColors::Highlight).into())
                .placeholder_color(ExcaliburColor::Label(system::LabelColors::Tertiary).into())
                .background(ExcaliburColor::Background4.into())
                .style_fn(theme, status)
        })
        .placeholder(placeholder.to_string())
}

#[derive(Debug, Clone, Copy)]
pub struct CustomSelect {
    pub active: iced::widget::pick_list::Style,
    pub hovered: iced::widget::pick_list::Style,
    pub current_state: SelectState,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum SelectState {
    #[default]
    Active,
    Hovered,
}

#[allow(dead_code)]
impl CustomSelect {
    pub fn new() -> Self {
        let default = iced::widget::pick_list::Style {
            text_color: SECONDARY_COLOR,
            placeholder_color: TERTIARY_LABEL_COLOR,
            handle_color: Color::WHITE,
            background: SELECT_BG_COLOR.into(),
            border: iced::Border {
                radius: 5.0.into(),
                width: 0.0,
                color: Default::default(),
                ..iced::Border::default()
            },
        };
        Self {
            active: default,
            hovered: default,
            current_state: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn active(mut self) -> Self {
        self.current_state = SelectState::Active;
        self
    }

    #[allow(dead_code)]
    pub fn hovered(mut self) -> Self {
        self.current_state = SelectState::Hovered;
        self
    }
    #[allow(dead_code)]
    pub fn text_color(mut self, color: Color) -> Self {
        match self.current_state {
            SelectState::Active => self.active.text_color = color,
            SelectState::Hovered => self.hovered.text_color = color,
        }

        self
    }
    #[allow(dead_code)]
    pub fn placeholder_color(mut self, color: Color) -> Self {
        match self.current_state {
            SelectState::Active => self.active.placeholder_color = color,
            SelectState::Hovered => self.hovered.placeholder_color = color,
        }

        self
    }

    pub fn handle_color(mut self, color: Color) -> Self {
        match self.current_state {
            SelectState::Active => self.active.handle_color = color,
            SelectState::Hovered => self.hovered.handle_color = color,
        }

        self
    }

    pub fn background(mut self, background: Background) -> Self {
        match self.current_state {
            SelectState::Active => self.active.background = background,
            SelectState::Hovered => self.hovered.background = background,
        }

        self
    }

    pub fn border_radius(mut self, radius: iced::border::Radius) -> Self {
        match self.current_state {
            SelectState::Active => self.active.border.radius = radius,
            SelectState::Hovered => self.hovered.border.radius = radius,
        }

        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        match self.current_state {
            SelectState::Active => self.active.border.width = width,
            SelectState::Hovered => self.hovered.border.width = width,
        }

        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        match self.current_state {
            SelectState::Active => self.active.border.color = color,
            SelectState::Hovered => self.hovered.border.color = color,
        }

        self
    }

    pub fn style_fn(
        &self,
        _theme: &iced::Theme,
        _status: iced::widget::pick_list::Status,
    ) -> iced::widget::pick_list::Style {
        match self.current_state {
            SelectState::Active => self.active,
            SelectState::Hovered => self.hovered,
        }
    }
}

pub struct CustomMenu {
    pub appearance: iced::widget::overlay::menu::Style,
}

impl CustomMenu {
    pub fn new() -> Self {
        Self {
            appearance: iced::widget::overlay::menu::Style {
                text_color: Color::WHITE,
                background: MENU_BG_COLOR.into(),
                border: iced::Border {
                    width: 1.0,
                    color: Color::BLACK,
                    radius: 5.0.into(),
                    ..iced::Border::default()
                },
                selected_text_color: Color::WHITE,
                selected_background: PRIMARY_COLOR.into(),
            },
        }
    }

    fn appearance(&self) -> iced::widget::overlay::menu::Style {
        self.appearance
    }
}
