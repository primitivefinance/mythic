#![allow(dead_code)]

use iced::{Color, Font, Padding};

pub const GRAY_100: Color = Color::from_rgb(
    0x15 as f32 / 255.0,
    0x15 as f32 / 255.0,
    0x15 as f32 / 255.0,
);

pub const GRAY_200: Color = Color::from_rgb(
    0x19 as f32 / 255.0,
    0x1A as f32 / 255.0,
    0x1D as f32 / 255.0,
);

pub const GRAY_300: Color = Color::from_rgb(
    0x1c as f32 / 255.0,
    0x1d as f32 / 255.0,
    0x20 as f32 / 255.0,
);

pub const GRAY_400: Color = Color::from_rgb(
    0x25 as f32 / 255.0,
    0x27 as f32 / 255.0,
    0x2D as f32 / 255.0,
);

pub const GRAY_500: Color = Color::from_rgb(
    0x2A as f32 / 255.0,
    0x2E as f32 / 255.0,
    0x3A as f32 / 255.0,
);

pub const GRAY_600: Color = Color::from_rgb(
    0x3B as f32 / 255.0,
    0x41 as f32 / 255.0,
    0x51 as f32 / 255.0,
);

pub const GRAY_700: Color = Color::from_rgb(
    0x54 as f32 / 255.0,
    0x5c as f32 / 255.0,
    0x74 as f32 / 255.0,
);

pub const GRAY_800: Color = Color::from_rgb(
    0x6d as f32 / 255.0,
    0x77 as f32 / 255.0,
    0x96 as f32 / 255.0,
);

pub const GRAY_900: Color = Color::from_rgb(
    0x84 as f32 / 255.0,
    0x90 as f32 / 255.0,
    0xb5 as f32 / 255.0,
);

pub const GRAY_1000: Color = Color::from_rgb(
    0x9D as f32 / 255.0,
    0xAB as f32 / 255.0,
    0xD4 as f32 / 255.0,
);

pub const BLUE_400: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x6b as f32 / 255.0,
    0xe6 as f32 / 255.0,
);

pub const BLUE_500: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x4E as f32 / 255.0,
    0xCA as f32 / 255.0,
);

pub const BLUE_600: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x4E as f32 / 255.0,
    0xCA as f32 / 255.0,
);

pub const CYAN_500: Color = Color::from_rgb(
    0x68 as f32 / 255.0,
    0xf4 as f32 / 255.0,
    0xfd as f32 / 255.0,
);

pub const GREEN_400: Color = Color::from_rgb(
    0x08 as f32 / 255.0,
    0xA0 as f32 / 255.0,
    0x45 as f32 / 255.0,
);

pub const RED_400: Color = Color::from_rgb(
    0xE4 as f32 / 255.0,
    0x4B as f32 / 255.0,
    0x41 as f32 / 255.0,
);

pub const MINT_500: Color = Color::from_rgb(
    0x5A as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xc4 as f32 / 255.0,
);

pub const PRIMARY_COLOR: Color = BLUE_500;
pub const SECONDARY_COLOR: Color = CYAN_500;
pub const PRIMARY_LABEL_COLOR: Color = Color::WHITE;
pub const SECONDARY_LABEL_COLOR: Color = GRAY_1000;
pub const TERTIARY_LABEL_COLOR: Color = GRAY_800;
pub const QUATERNARY_LABEL_COLOR: Color = GRAY_600;
pub const DISABLED_TEXT_GRAY: Color = GRAY_900;
pub const DISABLED_COLOR: Color = QUATERNARY_LABEL_COLOR;

// Main components
pub const BACKGROUND: Color = GRAY_100;
pub const SIDEBAR_BG_COLOR: Color = GRAY_200;
pub const SELECT_BG_COLOR: Color = GRAY_300;
pub const MENU_BG_COLOR: Color = SIDEBAR_BG_COLOR;
pub const CARD_BG_COLOR: Color = GRAY_400;
pub const WINDOW_HEADER_COLOR: Color = GRAY_500;
pub const INFO_BG_CONTAINER: Color = GRAY_500;
pub const TABLE_HEADER_BG: Color = GRAY_300;
pub const TABLE_ROW_1: Color = GRAY_500;
pub const TABLE_ROW_2: Color = GRAY_400;

// Element bg colors
pub const BORDER_COLOR: Color = GRAY_800;
pub const TABLE_COLUMN_BG_COLOR: Color = GRAY_900;
pub const STROKE_COLOR: Color = GRAY_1000;
pub const SELECTED_CONTAINER_COLOR: Color = GRAY_500;
pub const HIGHLIGHTED_CONTAINER_COLOR: Color = GRAY_600;

pub const SEMI_TRANSPARENT_HIGHLIGHT_CONTAINER: Color = Color {
    a: 0.25,
    ..HIGHLIGHTED_CONTAINER_COLOR
};

// Unused
pub const MODAL: Color = GRAY_400;
pub const PANEL: Color = GRAY_300;

// pretty sure this breaks if they don't have daggersquare installed?
pub const FONT_DAGGERSQUARE: Font = Font::with_name("DAGGERSQUARE");
pub const FONT_BOLD: Font = Font {
    family: iced::font::Family::Name("Arial"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    monospaced: false,
};

pub const FONT_SEMIBOLD: Font = Font {
    family: iced::font::Family::Name("Arial"),
    weight: iced::font::Weight::Semibold,
    stretch: iced::font::Stretch::Normal,
    monospaced: false,
};

/// Sizes for spacing, padding, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sizes {
    #[default]
    Zero = 0,
    Xs = 4,
    Sm = 8,
    Md = 16,
    Lg = 24,
    Xl = 32,
    Xl2 = 56,
    Xl3 = 64,
    Xl4 = 72,
    Xl5 = 96,
    Xl6 = 128,
}

impl From<Sizes> for f32 {
    fn from(item: Sizes) -> Self {
        match item {
            Sizes::Zero => 0.0,
            Sizes::Xs => 4.0,
            Sizes::Sm => 8.0,
            Sizes::Md => 16.0,
            Sizes::Lg => 24.0,
            Sizes::Xl => 32.0,
            Sizes::Xl2 => 56.0,
            Sizes::Xl3 => 64.0,
            Sizes::Xl4 => 72.0,
            Sizes::Xl5 => 96.0,
            Sizes::Xl6 => 128.0,
        }
    }
}

impl From<Sizes> for iced::Pixels {
    fn from(item: Sizes) -> Self {
        match item {
            Sizes::Zero => 0.0.into(),
            Sizes::Xs => 4.0.into(),
            Sizes::Sm => 8.0.into(),
            Sizes::Md => 16.0.into(),
            Sizes::Lg => 24.0.into(),
            Sizes::Xl => 32.0.into(),
            Sizes::Xl2 => 56.0.into(),
            Sizes::Xl3 => 64.0.into(),
            Sizes::Xl4 => 72.0.into(),
            Sizes::Xl5 => 96.0.into(),
            Sizes::Xl6 => 128.0.into(),
        }
    }
}

impl From<Sizes> for Padding {
    fn from(item: Sizes) -> Self {
        match item {
            Sizes::Zero => Padding::new(0.0),
            Sizes::Xs => Padding::new(4.0),
            Sizes::Sm => Padding::new(8.0),
            Sizes::Md => Padding::new(16.0),
            Sizes::Lg => Padding::new(24.0),
            Sizes::Xl => Padding::new(32.0),
            Sizes::Xl2 => Padding::new(56.0),
            Sizes::Xl3 => Padding::new(64.0),
            Sizes::Xl4 => Padding::new(72.0),
            Sizes::Xl5 => Padding::new(96.0),
            Sizes::Xl6 => Padding::new(128.0),
        }
    }
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

impl From<TitleSize> for iced::Pixels {
    fn from(item: TitleSize) -> Self {
        match item {
            TitleSize::Sm => 20.0.into(),
            TitleSize::Md => 24.0.into(),
            TitleSize::Lg => 28.0.into(),
            TitleSize::Xl => 34.0.into(),
        }
    }
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

impl From<ByteScale> for iced::Pixels {
    fn from(item: ByteScale) -> Self {
        match item {
            ByteScale::Xs => 1.0.into(),
            ByteScale::Sm => 2.0.into(),
            ByteScale::Md => 4.0.into(),
            ByteScale::Lg => 8.0.into(),
            ByteScale::Xl => 16.0.into(),
            ByteScale::Xl2 => 32.0.into(),
            ByteScale::Xl3 => 64.0.into(),
            ByteScale::Xl4 => 128.0.into(),
            ByteScale::Xl5 => 256.0.into(),
            ByteScale::Xl6 => 512.0.into(),
            ByteScale::Xl7 => 1024.0.into(),
            ByteScale::Xl8 => 2048.0.into(),
            ByteScale::Xl9 => 4096.0.into(),
            ByteScale::Xl10 => 8192.0.into(),
            ByteScale::Xl11 => 16384.0.into(),
            ByteScale::Xl12 => 32768.0.into(),
        }
    }
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
