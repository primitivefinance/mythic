//! Custom stylesheet wrapper for iced::Button.
//! Iced Buttons have multiple states that can be styled, but the default
//! stylesheet only allows you to set one style for all states.
//! This wrapper allows you to set different styles for each state.
//! It introduces a lot of boilerplate, which is why there is an
//! [`ExcaliburButton`] component that uses this wrapper to deliver a few
//! predefined button styles.

use iced::{
    border::Radius,
    widget::button::{Appearance, StyleSheet},
    Background, Border, Color, Shadow,
};

use super::styles::*;

/// Basically wraps the Button style sheet so that we can access and set
/// different appearances for the button.
/// Since each appearance has its own state, make sure to set each state to
/// match the desired appearance.. For example, if border radius is 10, you must
/// set the border radius for each state.
#[derive(Debug, Clone, Copy)]
pub struct CustomButtonStyle {
    pub active: Appearance,
    pub hovered: Appearance,
    pub pressed: Appearance,
    pub disabled: Appearance,
    pub current_state: ButtonState,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum ButtonState {
    #[default]
    Active,
    Hovered,
    Pressed,
    Disabled,
}

impl CustomButtonStyle {
    pub fn new() -> Self {
        let default = Appearance {
            shadow_offset: Default::default(),
            background: None,
            border: Border::default(),
            text_color: Color::WHITE,
            shadow: Shadow::default(),
        };
        Self {
            active: default,
            hovered: default,
            pressed: default,
            disabled: default,
            current_state: Default::default(),
        }
    }

    pub fn primary(theme: &iced::Theme) -> Self {
        Self {
            active: theme.active(&iced::theme::Button::Primary),
            hovered: theme.hovered(&iced::theme::Button::Primary),
            pressed: theme.pressed(&iced::theme::Button::Primary),
            disabled: theme.disabled(&iced::theme::Button::Primary),
            current_state: Default::default(),
        }
    }
    #[allow(dead_code)]
    pub fn secondary(theme: &iced::Theme) -> Self {
        Self {
            active: theme.active(&iced::theme::Button::Secondary),
            hovered: theme.hovered(&iced::theme::Button::Secondary),
            pressed: theme.pressed(&iced::theme::Button::Secondary),
            disabled: theme.disabled(&iced::theme::Button::Secondary),
            current_state: Default::default(),
        }
    }

    pub fn destructive(theme: &iced::Theme) -> Self {
        Self {
            active: theme.active(&iced::theme::Button::Destructive),
            hovered: theme.hovered(&iced::theme::Button::Destructive),
            pressed: theme.pressed(&iced::theme::Button::Destructive),
            disabled: theme.disabled(&iced::theme::Button::Destructive),
            current_state: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn positive(theme: &iced::Theme) -> Self {
        Self {
            active: theme.active(&iced::theme::Button::Positive),
            hovered: theme.hovered(&iced::theme::Button::Positive),
            pressed: theme.pressed(&iced::theme::Button::Positive),
            disabled: theme.disabled(&iced::theme::Button::Positive),
            current_state: Default::default(),
        }
    }

    pub fn text(theme: &iced::Theme) -> Self {
        Self {
            active: theme.active(&iced::theme::Button::Text),
            hovered: theme.hovered(&iced::theme::Button::Text),
            pressed: theme.pressed(&iced::theme::Button::Text),
            disabled: theme.disabled(&iced::theme::Button::Text),
            current_state: Default::default(),
        }
    }

    pub fn active(mut self) -> Self {
        self.current_state = ButtonState::Active;
        self
    }

    pub fn hovered(mut self) -> Self {
        self.current_state = ButtonState::Hovered;
        self
    }

    pub fn pressed(mut self) -> Self {
        self.current_state = ButtonState::Pressed;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.current_state = ButtonState::Disabled;
        self
    }

    pub fn background(mut self, background: Option<Background>) -> Self {
        match self.current_state {
            ButtonState::Active => self.active.background = background,
            ButtonState::Hovered => self.hovered.background = background,
            ButtonState::Pressed => self.pressed.background = background,
            ButtonState::Disabled => self.disabled.background = background,
        }
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        match self.current_state {
            ButtonState::Active => self.active.text_color = color,
            ButtonState::Hovered => self.hovered.text_color = color,
            ButtonState::Pressed => self.pressed.text_color = color,
            ButtonState::Disabled => self.disabled.text_color = color,
        }
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        match self.current_state {
            ButtonState::Active => self.active.background = Some(Background::Color(color)),
            ButtonState::Hovered => self.hovered.background = Some(Background::Color(color)),
            ButtonState::Pressed => self.pressed.background = Some(Background::Color(color)),
            ButtonState::Disabled => self.disabled.background = Some(Background::Color(color)),
        }
        self
    }

    pub fn border_radius(mut self, boarder_radius: Radius) -> Self {
        match self.current_state {
            ButtonState::Active => self.active.border.radius = boarder_radius,
            ButtonState::Hovered => self.hovered.border.radius = boarder_radius,
            ButtonState::Pressed => self.pressed.border.radius = boarder_radius,
            ButtonState::Disabled => self.disabled.border.radius = boarder_radius,
        }
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        match self.current_state {
            ButtonState::Active => self.active.border.width = width,
            ButtonState::Hovered => self.hovered.border.width = width,
            ButtonState::Pressed => self.pressed.border.width = width,
            ButtonState::Disabled => self.disabled.border.width = width,
        }
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        match self.current_state {
            ButtonState::Active => self.active.border.color = color,
            ButtonState::Hovered => self.hovered.border.color = color,
            ButtonState::Pressed => self.pressed.border.color = color,
            ButtonState::Disabled => self.disabled.border.color = color,
        }
        self
    }

    #[allow(dead_code)]
    pub fn shadow_offset(mut self, offset: iced::Vector) -> Self {
        match self.current_state {
            ButtonState::Active => self.active.shadow_offset = offset,
            ButtonState::Hovered => self.hovered.shadow_offset = offset,
            ButtonState::Pressed => self.pressed.shadow_offset = offset,
            ButtonState::Disabled => self.disabled.shadow_offset = offset,
        }
        self
    }

    pub fn as_custom(&self) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(*self))
    }
}

impl StyleSheet for CustomButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        self.active
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        self.hovered
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance {
        self.pressed
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        self.disabled
    }
}

pub fn route_button_style(bg: Color) -> CustomButtonStyle {
    CustomButtonStyle::new()
        .text_color(Color::WHITE)
        .border_radius(3.0.into())
        .background_color(bg)
        .pressed()
        .text_color(Color::WHITE)
        .border_radius(3.0.into())
        .background_color(SELECTED_CONTAINER_COLOR)
        .hovered()
        .text_color(Color::WHITE)
        .border_radius(3.0.into())
        .background_color(HIGHLIGHTED_CONTAINER_COLOR)
        .disabled()
        .text_color(DISABLED_COLOR)
        .border_radius(3.0.into())
}
