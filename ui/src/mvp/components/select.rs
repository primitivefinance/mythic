//! All custom select components and widgets.

use std::{borrow::Cow, rc::Rc};

use iced::widget::pick_list::{self, *};

use super::*;

pub fn custom_pick_list<'a, Message, T>(
    options: impl Into<Cow<'a, [T]>>,
    selected: Option<T>,
    on_selected: impl Fn(T) -> Message + 'a,
    placeholder: Option<&'a str>,
) -> PickList<'a, T, Message>
where
    T: ToString + Eq + 'static,
    [T]: ToOwned<Owned = Vec<T>>,
{
    pick_list(options, selected, on_selected)
        .style(CustomSelect::new().as_custom())
        .placeholder(placeholder.unwrap_or("Select an option"))
}

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

impl CustomSelect {
    pub fn new() -> Self {
        let default = Appearance {
            text_color: SECONDARY_COLOR,
            placeholder_color: TERTIARY_LABEL_COLOR,
            handle_color: Color::WHITE,
            background: MENU_BG_COLOR.into(),
            border_radius: 5.0.into(),
            border_width: 0.0,
            border_color: Default::default(),
        };
        Self {
            active: default,
            hovered: default,
            current_state: Default::default(),
        }
    }

    pub fn active(mut self) -> Self {
        self.current_state = SelectState::Active;
        self
    }

    pub fn hovered(mut self) -> Self {
        self.current_state = SelectState::Hovered;
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
                border_width: 0.0,
                border_radius: 5.0.into(),
                border_color: Default::default(),
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
