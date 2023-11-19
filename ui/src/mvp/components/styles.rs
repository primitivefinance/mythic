#![allow(dead_code)]

use iced::{Color, Font};

pub const GRAY_100: Color = Color::from_rgb(
    0x11 as f32 / 255.0,
    0x11 as f32 / 255.0,
    0x11 as f32 / 255.0,
);

pub const GRAY_200: Color = Color::from_rgb(
    0x16 as f32 / 255.0,
    0x16 as f32 / 255.0,
    0x16 as f32 / 255.0,
);

pub const GRAY_300: Color = Color::from_rgb(
    0x1c as f32 / 255.0,
    0x1c as f32 / 255.0,
    0x1c as f32 / 255.0,
);

pub const GRAY_400: Color = Color::from_rgb(
    0x22 as f32 / 255.0,
    0x22 as f32 / 255.0,
    0x22 as f32 / 255.0,
);

pub const GRAY_500: Color = Color::from_rgb(
    0x27 as f32 / 255.0,
    0x27 as f32 / 255.0,
    0x27 as f32 / 255.0,
);

pub const GRAY_600: Color = Color::from_rgb(
    0x2d as f32 / 255.0,
    0x2d as f32 / 255.0,
    0x2d as f32 / 255.0,
);

pub const GRAY_700: Color = Color::from_rgb(
    0x32 as f32 / 255.0,
    0x32 as f32 / 255.0,
    0x32 as f32 / 255.0,
);

pub const GRAY_800: Color = Color::from_rgb(
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
);

pub const GRAY_900: Color = Color::from_rgb(
    0x4c as f32 / 255.0,
    0x4c as f32 / 255.0,
    0x4c as f32 / 255.0,
);

pub const GRAY_1000: Color = Color::from_rgb(
    0x60 as f32 / 255.0,
    0x60 as f32 / 255.0,
    0x60 as f32 / 255.0,
);

pub const BLUE_400: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x6b as f32 / 255.0,
    0xe6 as f32 / 255.0,
);

pub const BLUE_500: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x77 as f32 / 255.0,
    0xff as f32 / 255.0,
);

pub const BLUE_600: Color = Color::from_rgb(
    0x1a as f32 / 255.0,
    0x85 as f32 / 255.0,
    0xff as f32 / 255.0,
);

pub const CYAN_500: Color = Color::from_rgb(
    0x68 as f32 / 255.0,
    0xf4 as f32 / 255.0,
    0xfd as f32 / 255.0,
);

pub const DISABLED_COLOR: Color = Color::from_rgba(
    0x41 as f32 / 255.0,
    0x51 as f32 / 255.0,
    0xE4 as f32 / 255.0,
    0.25,
);

pub const PRIMARY_COLOR: Color = BLUE_500;
pub const SECONDARY_COLOR: Color = CYAN_500;
pub const PRIMARY_LABEL_COLOR: Color = Color::WHITE;
pub const SECONDARY_LABEL_COLOR: Color = GRAY_1000;
pub const TERTIARY_LABEL_COLOR: Color = GRAY_800;
pub const QUATERNARY_LABEL_COLOR: Color = GRAY_600;
pub const DISABLED_TEXT_GRAY: Color = QUATERNARY_LABEL_COLOR;
pub const BACKGROUND: Color = GRAY_100;
pub const PANEL: Color = GRAY_300;
pub const MODAL: Color = GRAY_500;
pub const BORDER_COLOR: Color = GRAY_800;
pub const CARD_BG_COLOR: Color = GRAY_800;
pub const INFO_BG_CONTAINER: Color = GRAY_700;
pub const TABLE_COLUMN_BG_COLOR: Color = GRAY_900;
pub const STROKE_COLOR: Color = GRAY_1000;

// pretty sure this breaks if they don't have daggersquare installed?
pub const FONT_DAGGERSQUARE: Font = Font::with_name("DAGGERSQUARE");
pub const FONT_BOLD: Font = Font {
    family: iced::font::Family::Name("Arial"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    monospaced: false,
};

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
    TitleSm = 20,
    TitleMd = 24,
    TitleLg = 28,
    TitleXl = 34,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitleSize {
    Sm = 20,
    Md = 24,
    Lg = 28,
    Xl = 34,
}

pub enum TextSize {
    Xs = 12,
    Sm = 14,
    Md = 16,
    Lg = 18,
}

impl From<FontSizes> for iced::Pixels {
    fn from(font_size: FontSizes) -> Self {
        match font_size {
            FontSizes::Xs => 12.0.into(),
            FontSizes::Sm => 14.0.into(),
            FontSizes::Md => 16.0.into(),
            FontSizes::Lg => 18.0.into(),
            FontSizes::TitleSm => 20.0.into(),
            FontSizes::TitleMd => 24.0.into(),
            FontSizes::TitleLg => 28.0.into(),
            FontSizes::TitleXl => 34.0.into(),
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
