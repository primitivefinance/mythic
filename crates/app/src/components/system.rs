//! Entire Excalibur component system.

use iced::Font;

use super::*;

const BG1: Color = Color::from_rgb(
    0x04 as f32 / 255.0,
    0x04 as f32 / 255.0,
    0x04 as f32 / 255.0,
);

const BG2: Color = Color::from_rgb(
    0x0D as f32 / 255.0,
    0x0D as f32 / 255.0,
    0x0D as f32 / 255.0,
);

/// Quantitative colors are for different sizes of values.
/// - Hundreds < 1,000
/// - Thousands < 1,000,000
/// - Millions < 1,000,000,000
/// - Billions > 1,000,000,000
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantitativeColors {
    Hundreds,
    Thousands,
    Millions,
    Billions,
}

/// Label colors are for different text shades.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelColors {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Placeholder,
    Highlight,
}

const MINT: Color = Color::from_rgb(
    0x5A as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

const GREEN: Color = Color::from_rgb(
    0x30 as f32 / 255.0,
    0xFF as f32 / 255.0,
    0x83 as f32 / 255.0,
);

const RED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x42 as f32 / 255.0,
    0x42 as f32 / 255.0,
);

const HIGHLIGHT: Color = Color::from_rgb(
    0x70 as f32 / 255.0,
    0xD7 as f32 / 255.0,
    0xFF as f32 / 255.0,
);

const AMBER: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xD0 as f32 / 255.0,
    0x58 as f32 / 255.0,
);

const PRIMARY_LABEL: Color = Color::WHITE;
const SECONDARY_LABEL: Color = Color::from_rgb(
    0x92 as f32 / 255.0,
    0x92 as f32 / 255.0,
    0x92 as f32 / 255.0,
);
const TERTIARY_LABEL: Color = Color::from_rgb(
    0x4B as f32 / 255.0,
    0x4B as f32 / 255.0,
    0x4E as f32 / 255.0,
);
const QUATERNARY_LABEL: Color = Color::from_rgb(
    0x2A as f32 / 255.0,
    0x2A as f32 / 255.0,
    0x2D as f32 / 255.0,
);
const PLACEHOLDER_LABEL: Color = Color::from_rgb(
    0x9A as f32 / 255.0,
    0x9A as f32 / 255.0,
    0x9A as f32 / 255.0,
);

/// Available colors in Excalibur
/// - Background1 - Primary background color
/// - Background2 - Secondary background color
/// - Primary - Primary color, e.g. button bg fill.
/// - Success - Success color, e.g. successful transaction.
/// - Danger - Danger color, e.g. destructive action button.
/// - Label - Label color, e.g. text color.
/// - Quantitative - Quantitative color, e.g. quantitative text value color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExcaliburColor {
    Background1,
    Background2,
    #[default]
    Primary,
    Success,
    Danger,
    Label(LabelColors),
    Quantitative(QuantitativeColors),
}

impl ExcaliburColor {
    pub fn color(&self) -> Color {
        match self {
            ExcaliburColor::Background1 => BG1,
            ExcaliburColor::Background2 => BG2,
            ExcaliburColor::Primary => MINT,
            ExcaliburColor::Success => GREEN,
            ExcaliburColor::Danger => RED,
            ExcaliburColor::Label(label_color) => match label_color {
                LabelColors::Primary => PRIMARY_LABEL,
                LabelColors::Secondary => SECONDARY_LABEL,
                LabelColors::Tertiary => TERTIARY_LABEL,
                LabelColors::Quaternary => QUATERNARY_LABEL,
                LabelColors::Placeholder => PLACEHOLDER_LABEL,
                LabelColors::Highlight => HIGHLIGHT,
            },
            ExcaliburColor::Quantitative(quantitative_color) => match quantitative_color {
                QuantitativeColors::Hundreds => Color::WHITE,
                QuantitativeColors::Thousands => AMBER,
                QuantitativeColors::Millions => GREEN,
                QuantitativeColors::Billions => HIGHLIGHT,
            },

            _ => Color::WHITE,
        }
    }
}

impl From<ExcaliburColor> for Color {
    fn from(color: ExcaliburColor) -> Self {
        color.color()
    }
}

/// Packages the colors into a palette that can be used by Iced.
#[derive(Debug, Clone, Copy)]
pub struct ExcaliburTheme;

impl ExcaliburTheme {
    pub fn theme() -> Theme {
        let palette = Palette {
            background: ExcaliburColor::Background1.into(),
            primary: ExcaliburColor::Primary.into(),
            text: ExcaliburColor::Label(LabelColors::Primary).into(),
            success: ExcaliburColor::Success.into(),
            danger: ExcaliburColor::Danger.into(),
        };

        Theme::Custom(Box::new(iced::theme::Custom::new(palette)))
    }
}

/// Sizing of text in Excalibur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Typography {
    Title1 = 28,
    Title2 = 24,
    Title3 = 20,
    Headline = 18,
    Body = 16,
    Subhead = 15,
    Footnote = 14,
    Caption = 12,
}

impl From<Typography> for iced::Pixels {
    fn from(typography: Typography) -> Self {
        match typography {
            Typography::Title1 => iced::Pixels(28.0),
            Typography::Title2 => iced::Pixels(24.0),
            Typography::Title3 => iced::Pixels(20.0),
            Typography::Headline => iced::Pixels(18.0),
            Typography::Body => iced::Pixels(16.0),
            Typography::Subhead => iced::Pixels(15.0),
            Typography::Footnote => iced::Pixels(14.0),
            Typography::Caption => iced::Pixels(12.0),
        }
    }
}

pub const UI_FONT: Font = Font {
    family: iced::font::Family::Name("Yu Gothic UI"),
    weight: iced::font::Weight::Semibold,
    stretch: iced::font::Stretch::Normal,
    monospaced: false,
};
pub const UI_FONT_BOLD: Font = Font {
    family: iced::font::Family::Name("Yu Gothic UI"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    monospaced: false,
};
pub const BRAND_FONT: Font = Font::with_name("DAGGERSQUARE");

#[derive(Debug, Clone, Copy, Default)]
pub enum ExcaliburFonts {
    #[default]
    UI,
    UIBold,
    Branding,
}

impl ExcaliburFonts {
    pub fn font(&self) -> Font {
        match self {
            ExcaliburFonts::UI => UI_FONT,
            ExcaliburFonts::UIBold => UI_FONT_BOLD,
            ExcaliburFonts::Branding => BRAND_FONT,
        }
    }
}

/// For constructing any text rendered in Excalibur.
#[derive(Debug, Clone)]
pub struct ExcaliburText {
    pub value: String,
    pub color: ExcaliburColor,
    pub font: ExcaliburFonts,
    pub size: Typography,
}

impl Default for ExcaliburText {
    fn default() -> Self {
        Self {
            value: "text".to_string(),
            color: ExcaliburColor::Label(LabelColors::Primary),
            font: ExcaliburFonts::UI,
            size: Typography::Body,
        }
    }
}

impl ExcaliburText {
    /// For constructing any text rendered in Excalibur.
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            ..Default::default()
        }
    }

    /// Builds the text for Iced.
    pub fn build<'a>(self) -> Text<'a> {
        Text::new(self.value)
            .style(self.color.color())
            .font(self.font.font())
            .size(self.size)
    }

    // Class

    /// Formats the text based on float value.
    pub fn quantitative(self) -> Self {
        // todo: this is probably very dangerous!
        let value = self.value.parse::<f64>().unwrap_or(0.0);

        let mut color = QuantitativeColors::Hundreds;

        let value = if value < 1_000.0 {
            color = QuantitativeColors::Hundreds;
            format!("{:.2}", value)
        } else if value < 1_000_000.0 {
            color = QuantitativeColors::Thousands;
            format!("{:.2}K", value / 1_000.0)
        } else if value < 1_000_000_000.0 {
            color = QuantitativeColors::Millions;
            format!("{:.2}M", value / 1_000_000.0)
        } else {
            color = QuantitativeColors::Billions;
            format!("{:.2}B", value / 1_000_000_000.0)
        };

        Self {
            value,
            color: ExcaliburColor::Quantitative(color),
            ..self
        }
    }

    // Size

    pub fn title1(self) -> Self {
        Self {
            size: Typography::Title1,
            ..self
        }
    }

    pub fn title2(self) -> Self {
        Self {
            size: Typography::Title2,
            ..self
        }
    }

    pub fn title3(self) -> Self {
        Self {
            size: Typography::Title3,
            ..self
        }
    }

    pub fn headline(self) -> Self {
        Self {
            size: Typography::Headline,
            ..self
        }
    }

    pub fn body(self) -> Self {
        Self {
            size: Typography::Body,
            ..self
        }
    }

    pub fn subhead(self) -> Self {
        Self {
            size: Typography::Subhead,
            ..self
        }
    }

    pub fn footnote(self) -> Self {
        Self {
            size: Typography::Footnote,
            ..self
        }
    }

    pub fn caption(self) -> Self {
        Self {
            size: Typography::Caption,
            ..self
        }
    }

    // Color

    pub fn primary(self) -> Self {
        Self {
            color: ExcaliburColor::Label(LabelColors::Primary),
            ..self
        }
    }

    pub fn secondary(self) -> Self {
        Self {
            color: ExcaliburColor::Label(LabelColors::Secondary),
            ..self
        }
    }

    pub fn tertiary(self) -> Self {
        Self {
            color: ExcaliburColor::Label(LabelColors::Tertiary),
            ..self
        }
    }

    pub fn quaternary(self) -> Self {
        Self {
            color: ExcaliburColor::Label(LabelColors::Quaternary),
            ..self
        }
    }

    pub fn placeholder(self) -> Self {
        Self {
            color: ExcaliburColor::Label(LabelColors::Placeholder),
            ..self
        }
    }

    pub fn highlight(self) -> Self {
        Self {
            color: ExcaliburColor::Label(LabelColors::Highlight),
            ..self
        }
    }

    // Type

    pub fn hundreds(self) -> Self {
        Self {
            color: ExcaliburColor::Quantitative(QuantitativeColors::Hundreds),
            ..self
        }
    }

    pub fn thousands(self) -> Self {
        Self {
            color: ExcaliburColor::Quantitative(QuantitativeColors::Thousands),
            ..self
        }
    }

    pub fn millions(self) -> Self {
        Self {
            color: ExcaliburColor::Quantitative(QuantitativeColors::Millions),
            ..self
        }
    }

    pub fn billions(self) -> Self {
        Self {
            color: ExcaliburColor::Quantitative(QuantitativeColors::Billions),
            ..self
        }
    }

    // Font

    pub fn ui(self) -> Self {
        Self {
            font: ExcaliburFonts::UI,
            ..self
        }
    }

    pub fn ui_bold(self) -> Self {
        Self {
            font: ExcaliburFonts::UIBold,
            ..self
        }
    }

    pub fn branding(self) -> Self {
        Self {
            font: ExcaliburFonts::Branding,
            ..self
        }
    }
}

/// For constructing any text rendered in Excalibur.
pub fn label<'a>(value: &str) -> ExcaliburText {
    ExcaliburText::new(value)
}

/// todo!
pub struct ExcaliburButton;
