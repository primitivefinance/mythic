#![allow(dead_code)]

use iced::{Color, Font};

pub const BLACK: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

pub const OFF_WHITE: Color = Color::from_rgb(
    0xfc as f32 / 255.0,
    0xfc as f32 / 255.0,
    0xfc as f32 / 255.0,
);

pub const SECONDARY: Color = Color::from_rgb(
    0xf8 as f32 / 255.0,
    0xf9 as f32 / 255.0,
    0xf9 as f32 / 255.0,
);

pub const LEVEL_1_COLOR: Color = Color::from_rgb(
    0xF8 as f32 / 255.0,
    0xF8 as f32 / 255.0,
    0xF8 as f32 / 255.0,
);

pub const LEVEL_2_COLOR: Color = Color::from_rgb(
    0xF4 as f32 / 255.0,
    0xF4 as f32 / 255.0,
    0xF5 as f32 / 255.0,
);

pub const LEVEL_3_COLOR: Color = Color::from_rgb(
    0xEA as f32 / 255.0,
    0xEA as f32 / 255.0,
    0xEB as f32 / 255.0,
);

pub const LEVEL_4_COLOR: Color = Color::from_rgb(
    0xD4 as f32 / 255.0,
    0xD4 as f32 / 255.0,
    0xD6 as f32 / 255.0,
);

pub const BACKGROUND: Color = Color::from_rgb(
    0x12 as f32 / 255.0,
    0x12 as f32 / 255.0,
    0x12 as f32 / 255.0,
);

pub const PANEL: Color = Color::from_rgb(
    0x18 as f32 / 255.0,
    0x18 as f32 / 255.0,
    0x18 as f32 / 255.0,
);

pub const MODAL: Color = Color::from_rgb(
    0x28 as f32 / 255.0,
    0x28 as f32 / 255.0,
    0x28 as f32 / 255.0,
);

pub const BORDER_COLOR: Color = Color::from_rgb(
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
);

// pretty sure this breaks if they don't have daggersquare installed?
pub const FONT_DAGGERSQUARE: Font = Font::with_name("DAGGERSQUARE");
