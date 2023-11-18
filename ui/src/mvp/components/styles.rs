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

/// todo: fix the color scheme, which was named poorly.
pub const CARD_BG_COLOR: Color = Color::from_rgb(
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
);

// pretty sure this breaks if they don't have daggersquare installed?
pub const FONT_DAGGERSQUARE: Font = Font::with_name("DAGGERSQUARE");

/// Sizes for spacing, padding, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sizes {
    Xs = 4,
    Sm = 8,
    Md = 16,
    Lg = 24,
    Xl = 32,
}

/// Sizes for fonts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSizes {
    Xs = 12,
    Sm = 14,
    Md = 16,
    Lg = 18,
    Xl = 28,
    Xl2 = 38,
}

impl From<FontSizes> for iced::Pixels {
    fn from(font_size: FontSizes) -> Self {
        match font_size {
            FontSizes::Xs => 12.0.into(),
            FontSizes::Sm => 14.0.into(),
            FontSizes::Md => 16.0.into(),
            FontSizes::Lg => 18.0.into(),
            FontSizes::Xl => 28.0.into(),
            FontSizes::Xl2 => 38.0.into(),
        }
    }
}

/// Scale from golden ratio.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoldenRatioScale {
    Xs = 1,
    Sm = 2,
    Md = 3,
    Lg = 5,
    Xl = 8,
    Xl2 = 13,
    Xl3 = 21,
    Xl4 = 34,
    Xl5 = 55,
    Xl6 = 89,
    Xl7 = 144,
    Xl8 = 233,
    Xl9 = 377,
    Xl10 = 610,
    Xl11 = 987,
    Xl12 = 1597,
}

/// Scale from byte sizes.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteScale {
    Xs = 1,
    Sm = 2,
    Md = 4,
    Lg = 8,
    Xl = 16,
    Xl2 = 32,
    Xl3 = 64,
    Xl4 = 128,
    Xl5 = 256,
    Xl6 = 512,
    Xl7 = 1024,
    Xl8 = 2048,
    Xl9 = 4096,
    Xl10 = 8192,
    Xl11 = 16384,
    Xl12 = 32768,
}

impl ByteScale {
    pub fn between(&self, other: &ByteScale) -> f32 {
        let self_f32: f32 = (*self).into();
        let other_f32: f32 = (*other).into();
        (self_f32 + other_f32) / 2.0
    }
}

impl From<ByteScale> for f32 {
    fn from(item: ByteScale) -> Self {
        match item {
            ByteScale::Xs => 1.0,
            ByteScale::Sm => 2.0,
            ByteScale::Md => 4.0,
            ByteScale::Lg => 8.0,
            ByteScale::Xl => 16.0,
            ByteScale::Xl2 => 32.0,
            ByteScale::Xl3 => 64.0,
            ByteScale::Xl4 => 128.0,
            ByteScale::Xl5 => 256.0,
            ByteScale::Xl6 => 512.0,
            ByteScale::Xl7 => 1024.0,
            ByteScale::Xl8 => 2048.0,
            ByteScale::Xl9 => 4096.0,
            ByteScale::Xl10 => 8192.0,
            ByteScale::Xl11 => 16384.0,
            ByteScale::Xl12 => 32768.0,
        }
    }
}
