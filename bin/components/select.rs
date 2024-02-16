//! All custom select components and widgets.

use std::{borrow::Cow, rc::Rc};

use iced::{border, widget::pick_list::*, Background, Border};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct CustomSelect {
    pub active: Appearance,
    pub hovered: Appearance,
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
        let default = Appearance {
            text_color: SECONDARY_COLOR,
            placeholder_color: TERTIARY_LABEL_COLOR,
            handle_color: Color::WHITE,
            background: SELECT_BG_COLOR.into(),
            border: Border::with_radius(5.0),
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

    pub fn border_radius(mut self, radius: border::Radius) -> Self {
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

    pub fn as_custom(&self) -> iced::theme::PickList {
        iced::theme::PickList::Custom(Rc::new(*self), Rc::new(CustomMenu::new()))
    }
}

impl StyleSheet for CustomSelect {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        self.active
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        self.hovered
    }
}

pub struct CustomMenu {
    pub appearance: iced::widget::overlay::menu::Appearance,
}

impl CustomMenu {
    pub fn new() -> Self {
        Self {
            appearance: iced::widget::overlay::menu::Appearance {
                text_color: Color::WHITE,
                background: MENU_BG_COLOR.into(),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::BLACK,
                },
                selected_text_color: Color::WHITE,
                selected_background: PRIMARY_COLOR.into(),
            },
        }
    }
}

impl iced::widget::overlay::menu::StyleSheet for CustomMenu {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::overlay::menu::Appearance {
        self.appearance
    }
}
