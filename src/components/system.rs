//! Entire Mythic component system.

use std::collections::BTreeMap;
use std::sync::Arc;

use iced::{
    alignment,
    border::Radius as BorderRadius,
    theme::Palette,
    widget::{button, container, tooltip, Button, Container, Text},
    Color, Element, Fill, Font, Renderer, Theme,
};

use iced_aw::{bootstrap::icon_to_char, Bootstrap};

use super::styles::{Sizes, DISABLED_TEXT_GRAY, GRAY_600};

use mythic_charts::prelude::*;

const BG1: Color = Color::from_rgb(
    0x04 as f32 / 255.0,
    0x04 as f32 / 255.0,
    0x04 as f32 / 255.0,
);

const BG2: Color = Color::from_rgb(
    0x0A as f32 / 255.0,
    0x0A as f32 / 255.0,
    0x0A as f32 / 255.0,
);

// not darker than BG1, but placed at the lowest level of the app.
const BG3: Color = Color::from_rgb(
    0x14 as f32 / 255.0,
    0x14 as f32 / 255.0,
    0x14 as f32 / 255.0,
);

const BG4: Color = Color::from_rgb(
    0x28 as f32 / 255.0,
    0x28 as f32 / 255.0,
    0x28 as f32 / 255.0,
);

const CARD_BG: Color = Color::from_rgb(
    0x2A as f32 / 255.0,
    0x29 as f32 / 255.0,
    0x2E as f32 / 255.0,
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
    Disabled,
}

const BLUE: Color = Color::from_rgb(
    0x0E as f32 / 255.0,
    0x44 as f32 / 255.0,
    0xCC as f32 / 255.0,
);

const BLUE_DISABLED: Color = Color::from_rgba(
    0x0E as f32 / 255.0,
    0x44 as f32 / 255.0,
    0xCC as f32 / 255.0,
    0.5,
);

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

const GREEN_BUTTON: Color = Color::from_rgb(
    0x26 as f32 / 255.0,
    0xAA as f32 / 255.0,
    0x5B as f32 / 255.0,
);

const LIGHT_GREEN: Color = Color::from_rgba(
    0x41 as f32 / 255.0,
    0xE4 as f32 / 255.0,
    0xB3 as f32 / 255.0,
    0.2,
);

const RED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x45 as f32 / 255.0,
    0x3A as f32 / 255.0,
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

/// Available colors in Mythic
/// - Background1 - Primary background color
/// - Background2 - Secondary background color
/// - Primary - Primary color, e.g. button bg fill.
/// - Success - Success color, e.g. successful transaction.
/// - Danger - Danger color, e.g. destructive action button.
/// - Label - Label color, e.g. text color.
/// - Quantitative - Quantitative color, e.g. quantitative text value color.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ExcaliburColor {
    Background1,
    Background2,
    Background3,
    Background4,
    Transparent,
    Card,
    #[default]
    Primary,
    Success,
    Danger,
    Mint,
    Error,
    Pending,
    PrimaryDisabled,
    Button(ButtonColors),
    Label(LabelColors),
    Quantitative(QuantitativeColors),
    Custom(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonColors {
    #[default]
    Primary,
    Success,
    Pending,
    Error,
}

impl ExcaliburColor {
    pub fn color(&self) -> Color {
        match self {
            ExcaliburColor::Background1 => BG1,
            ExcaliburColor::Background2 => BG2,
            ExcaliburColor::Background3 => BG3,
            ExcaliburColor::Background4 => BG4,
            ExcaliburColor::Transparent => Color::TRANSPARENT,
            ExcaliburColor::Card => CARD_BG,
            ExcaliburColor::Primary => BLUE,
            ExcaliburColor::Success => GREEN,
            ExcaliburColor::Danger => RED,
            ExcaliburColor::Mint => MINT,
            ExcaliburColor::Error => RED,
            ExcaliburColor::Pending => LIGHT_GREEN,
            ExcaliburColor::PrimaryDisabled => BLUE_DISABLED,
            ExcaliburColor::Button(button_color) => match button_color {
                ButtonColors::Primary => BLUE,
                ButtonColors::Success => GREEN_BUTTON,
                ButtonColors::Pending => LIGHT_GREEN,
                ButtonColors::Error => AMBER,
            },
            ExcaliburColor::Label(label_color) => match label_color {
                LabelColors::Primary => PRIMARY_LABEL,
                LabelColors::Secondary => SECONDARY_LABEL,
                LabelColors::Tertiary => TERTIARY_LABEL,
                LabelColors::Quaternary => QUATERNARY_LABEL,
                LabelColors::Placeholder => PLACEHOLDER_LABEL,
                LabelColors::Highlight => MINT,
                LabelColors::Disabled => DISABLED_TEXT_GRAY,
            },
            ExcaliburColor::Quantitative(quantitative_color) => match quantitative_color {
                QuantitativeColors::Hundreds => Color::WHITE,
                QuantitativeColors::Thousands => AMBER,
                QuantitativeColors::Millions => GREEN,
                QuantitativeColors::Billions => HIGHLIGHT,
            },
            ExcaliburColor::Custom(color) => *color,
        }
    }
}

impl From<ExcaliburColor> for Color {
    fn from(color: ExcaliburColor) -> Self {
        color.color()
    }
}

impl From<ExcaliburColor> for iced::Background {
    fn from(color: ExcaliburColor) -> Self {
        iced::Background::Color(color.color())
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

        Theme::Custom(Arc::new(iced::theme::Custom::new(
            "main".to_string(),
            palette,
        )))
    }
}

/// Sizing of text in Mythic.
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
    Caption2 = 10,
}

impl From<Typography> for f32 {
    fn from(typography: Typography) -> Self {
        match typography {
            Typography::Title1 => 28.0,
            Typography::Title2 => 24.0,
            Typography::Title3 => 20.0,
            Typography::Headline => 18.0,
            Typography::Body => 16.0,
            Typography::Subhead => 15.0,
            Typography::Footnote => 14.0,
            Typography::Caption => 12.0,
            Typography::Caption2 => 10.0,
        }
    }
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
            Typography::Caption2 => iced::Pixels(10.0),
        }
    }
}

pub const SYMBOL_FONT: Font = Font::with_name("Yu Gothic");
pub const UI_FONT: Font = Font::with_name("Yu Gothic UI");
pub const UI_FONT_SEMIBOLD: Font = Font {
    family: iced::font::Family::Name("Yu Gothic UI"),
    weight: iced::font::Weight::Semibold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};
pub const UI_FONT_BOLD: Font = Font {
    family: iced::font::Family::Name("Yu Gothic UI"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};
pub const BRAND_FONT: Font = Font::with_name("DAGGERSQUARE");

#[derive(Debug, Clone, Copy, Default)]
pub enum ExcaliburFonts {
    #[default]
    UI,
    UISemibold,
    UIBold,
    Branding,
    Symbol,
    Icon,
    Custom(iced::Font),
}

impl ExcaliburFonts {
    pub fn font(&self) -> Font {
        match self {
            ExcaliburFonts::UI => UI_FONT,
            ExcaliburFonts::UISemibold => UI_FONT_SEMIBOLD,
            ExcaliburFonts::UIBold => UI_FONT_BOLD,
            ExcaliburFonts::Branding => BRAND_FONT,
            ExcaliburFonts::Symbol => SYMBOL_FONT,
            ExcaliburFonts::Icon => iced_aw::BOOTSTRAP_FONT,
            ExcaliburFonts::Custom(font) => *font,
        }
    }
}

impl From<ExcaliburFonts> for Font {
    fn from(font: ExcaliburFonts) -> Self {
        font.font()
    }
}

/// For constructing any text rendered in Mythic.
#[derive(Debug, Clone)]
pub struct ExcaliburText {
    pub value: String,
    pub color: ExcaliburColor,
    pub font: ExcaliburFonts,
    pub size: Typography,
    pub align_x: alignment::Horizontal,
    pub align_y: alignment::Vertical,
}

impl Default for ExcaliburText {
    fn default() -> Self {
        Self {
            value: "text".to_string(),
            color: ExcaliburColor::Label(LabelColors::Primary),
            font: ExcaliburFonts::UI,
            size: Typography::Body,
            align_x: alignment::Horizontal::Left,
            align_y: alignment::Vertical::Top,
        }
    }
}

/// For constructing any text rendered in Mythic.
pub fn label(value: impl ToString) -> ExcaliburText {
    ExcaliburText::new(value)
}

pub fn format_number(num: f64) -> (String, QuantitativeColors) {
    let (divisor, suffix, color) = if num >= 1_000_000_000.0 {
        (1_000_000_000.0, "B", QuantitativeColors::Billions)
    } else if num >= 1_000_000.0 {
        (1_000_000.0, "M", QuantitativeColors::Millions)
    } else if num >= 1_000.0 {
        (1_000.0, "K", QuantitativeColors::Thousands)
    } else {
        (1.0, "", QuantitativeColors::Hundreds)
    };

    let value = num / divisor;
    let int_len = value.trunc().to_string().len();
    let decimal_places = if int_len >= 4 { 0 } else { 4 - int_len };
    let decimal_places = if suffix.is_empty() {
        decimal_places + 1
    } else {
        decimal_places
    };

    (format!("{:.*}{}", decimal_places, value, suffix), color)
}

impl ExcaliburText {
    /// For constructing any text rendered in Mythic.
    pub fn new(value: impl ToString) -> Self {
        Self {
            value: value.to_string(),
            ..Default::default()
        }
    }

    /// Builds the text for Iced.
    pub fn build<'a>(self) -> Text<'a> {
        Text::new(self.value)
            .color(self.color.color())
            .font(self.font.font())
            .size(self.size)
            .align_x(self.align_x)
            .align_y(self.align_y)
    }

    pub fn custom_typography(self, typography: Typography) -> Self {
        Self {
            size: typography,
            ..self
        }
    }

    pub fn custom_font(self, font: ExcaliburFonts) -> Self {
        Self { font, ..self }
    }

    pub fn custom_color(self, color: ExcaliburColor) -> Self {
        Self { color, ..self }
    }

    // Class

    /// Customizes the text parsed in as an f64 and processed through a
    /// classifier. For example, the classifier can return a custom color if
    /// the value is > 10.0. This is useful for colorizing values that might
    /// have different "good" values vs. "bad" values.
    ///
    /// Return None in the classifier if you want to default to the original
    /// text.
    pub fn custom_format(self, classifier: impl FnOnce(f64) -> Option<ExcaliburColor>) -> Self {
        // this fails when a user loads for the first time which should be okay to
        // default to zero.
        let value = self.value.parse::<f64>().unwrap_or(0.0);
        if let Some(color) = classifier(value) {
            Self { color, ..self }
        } else {
            self
        }
    }

    /// Formats the text based on float value.
    pub fn quantitative(self) -> Self {
        let value = self.value.parse::<f64>().unwrap_or(0.0);
        let (value, color) = format_number(value);
        Self {
            value,
            color: ExcaliburColor::Quantitative(color),
            ..self
        }
    }

    /// Formats the text based on percentage value.
    pub fn percentage(self) -> Self {
        let value = self.value.parse::<f64>().unwrap_or(0.0);
        let percentage_value = value * 100.0;
        let color = if percentage_value < 10.0 {
            QuantitativeColors::Hundreds
        } else if percentage_value < 50.0 {
            QuantitativeColors::Thousands
        } else if percentage_value < 95.0 {
            QuantitativeColors::Millions
        } else {
            QuantitativeColors::Billions
        };

        let value = format!("{:.2}%", percentage_value);

        Self {
            value,
            color: ExcaliburColor::Quantitative(color),
            ..self
        }
    }

    pub fn usd(self) -> Self {
        let value = format!("{} USD", self.value);
        Self { value, ..self }
    }

    pub fn usd_symbol(self) -> Self {
        let value = format!("${}", self.value);
        Self { value, ..self }
    }

    pub fn style(mut self, color: iced::Color) -> Self {
        self.color = ExcaliburColor::Custom(color);
        self
    }

    pub fn brightness(mut self, brightness: f32) -> Self {
        let internal_color = self.color.color();
        self.color = ExcaliburColor::Custom(Color::from_rgba(
            internal_color.r,
            internal_color.g,
            internal_color.b,
            brightness,
        ));
        self
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

    pub fn caption2(self) -> Self {
        Self {
            size: Typography::Caption2,
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

    pub fn sf(self) -> Self {
        Self {
            font: ExcaliburFonts::Custom(iced::font::Font::DEFAULT),
            ..self
        }
    }

    pub fn mono(self) -> Self {
        Self {
            font: ExcaliburFonts::Custom(iced::font::Font::MONOSPACE),
            ..self
        }
    }

    pub fn ui(self) -> Self {
        Self {
            font: ExcaliburFonts::UI,
            ..self
        }
    }

    /// todo: fix on macos
    pub fn ui_semibold(self) -> Self {
        Self {
            font: ExcaliburFonts::UISemibold,
            ..self
        }
    }

    /// todo: fix on macos
    pub fn ui_bold(self) -> Self {
        Self {
            font: ExcaliburFonts::UIBold,
            ..self
        }
    }

    pub fn symbol(self) -> Self {
        Self {
            font: ExcaliburFonts::Symbol,
            ..self
        }
    }

    pub fn branding(self) -> Self {
        Self {
            font: ExcaliburFonts::Branding,
            ..self
        }
    }

    pub fn icon(self) -> Self {
        Self {
            font: ExcaliburFonts::Icon,
            ..self
        }
    }

    // Alignment

    /// Sets text horizontal alignment to left.
    pub fn left(self) -> Self {
        Self {
            align_x: alignment::Horizontal::Left,
            ..self
        }
    }

    /// Sets text horizontal alignment to center.
    pub fn center(self) -> Self {
        Self {
            align_x: alignment::Horizontal::Center,
            ..self
        }
    }

    /// Sets text horizontal alignment to right.
    pub fn right(self) -> Self {
        Self {
            align_x: alignment::Horizontal::Right,
            ..self
        }
    }

    /// Sets text vertical alignment to top.
    pub fn top(self) -> Self {
        Self {
            align_y: alignment::Vertical::Top,
            ..self
        }
    }

    /// Sets text vertical alignment to middle.
    pub fn middle(self) -> Self {
        Self {
            align_y: alignment::Vertical::Center,
            ..self
        }
    }

    /// Sets text vertical alignment to bottom.
    pub fn bottom(self) -> Self {
        Self {
            align_y: alignment::Vertical::Bottom,
            ..self
        }
    }
}

impl<Message> From<ExcaliburText> for Element<'_, Message> {
    fn from(text: ExcaliburText) -> Self {
        text.build().into()
    }
}

/// For constructing Mythic containers.
#[allow(dead_code)]
pub fn panel() -> ExcaliburContainer {
    ExcaliburContainer::default()
}

/// For building containers with different background shading, text, and
/// borders.
#[derive(Debug, Clone, Copy)]
pub struct ExcaliburContainer {
    pub background: ExcaliburColor,
    pub border_radius: BorderRadius,
    pub border_width: f32,
    pub border_color: ExcaliburColor,
    pub text_color: ExcaliburColor,
}

impl Default for ExcaliburContainer {
    fn default() -> Self {
        Self {
            background: ExcaliburColor::Transparent,
            border_radius: Sizes::Sm.into(),
            border_width: 0.0,
            border_color: ExcaliburColor::Label(LabelColors::Quaternary),
            text_color: ExcaliburColor::Label(LabelColors::Primary),
        }
    }
}

impl ExcaliburContainer {
    pub fn theme(&self) -> container::Style {
        let background = self.background.color();
        let border_color = self.border_color.color();
        let border_radius = self.border_radius;
        let border_width = self.border_width;

        container::Style {
            background: Some(iced::Background::Color(background)),
            border: iced::Border {
                width: border_width,
                radius: border_radius,
                color: border_color,
            },
            ..container::Style::default()
        }
    }
}

impl ExcaliburContainer {
    pub fn indicator() -> Self {
        Self {
            background: ExcaliburColor::Primary,
            border_radius: Sizes::Xs.into(),
            border_width: 0.0,
            border_color: ExcaliburColor::Custom(Color::WHITE),
            text_color: ExcaliburColor::Custom(Color::WHITE),
        }
    }

    pub fn build<'a, Message>(
        self,
        element: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a,
    {
        Container::new(element).style(move |_theme| self.theme())
    }

    // Levels

    /// The layer that is furthest away and therefore the darkest.
    #[allow(dead_code)]
    pub fn bottom(mut self) -> Self {
        self.background = ExcaliburColor::Background1;
        self
    }

    /// The layer between the bottom and middle layer.
    pub fn middle_bottom(mut self) -> Self {
        self.background = ExcaliburColor::Background2;
        self
    }

    /// The layer between the middle and top layer.
    pub fn middle_top(mut self) -> Self {
        self.background = ExcaliburColor::Background3;
        self
    }

    /// The layer that is closest and therefore the lightest.
    pub fn top(mut self) -> Self {
        self.background = ExcaliburColor::Background4;
        self
    }

    /// Transparent background
    pub fn transparent(mut self) -> Self {
        self.background = ExcaliburColor::Transparent;
        self
    }

    /// Act as an indicator or barrier.
    pub fn background(mut self, color: ExcaliburColor) -> Self {
        self.background = color;
        self
    }

    /// Choose your own color!
    pub fn background_iced(mut self, color: iced::Color) -> Self {
        self.background = ExcaliburColor::Custom(color);
        self
    }

    /// Choose your own color!
    #[allow(dead_code)]
    pub fn background_rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.background = ExcaliburColor::Custom(iced::Color::from_rgb(r, g, b));
        self
    }

    pub fn text_color(mut self, color: ExcaliburColor) -> Self {
        self.text_color = color;
        self
    }

    // Border radius
    #[allow(dead_code)]
    pub fn sharp(mut self) -> Self {
        self.border_radius = 0.0.into();
        self
    }

    pub fn round(mut self, size: Sizes) -> Self {
        self.border_radius = size.into();
        self
    }

    pub fn border_radius(mut self, size: BorderRadius) -> Self {
        self.border_radius = size;
        self
    }

    // Border
    #[allow(dead_code)]
    pub fn border(mut self, color: ExcaliburColor, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn light_border(mut self) -> Self {
        self.border_color = ExcaliburColor::Custom(GRAY_600);
        self.border_width = 1.0;
        self
    }
    #[allow(dead_code)]
    pub fn black_border(mut self) -> Self {
        self.border_color = ExcaliburColor::Custom(Color::BLACK);
        self.border_width = 2.0;
        self
    }

    pub fn white_border(mut self) -> Self {
        self.border_color = ExcaliburColor::Custom(Color::WHITE);
        self.border_width = 1.0;
        self
    }

    // Presets
    #[allow(dead_code)]
    pub fn card(mut self) -> Self {
        self.background = ExcaliburColor::Card;
        self.border_radius = Sizes::Sm.into();
        self
    }
}

/// todo!
/// Exposes an easier to use API over the CustomButtonStyle, which is a wrapper
/// over the iced Button stylesheet.
pub struct ExcaliburButton;

impl ExcaliburButton {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build<'a, Message>(self, element: impl Into<Element<'a, Message>>) -> Button<'a, Message>
    where
        Message: 'a,
    {
        button(Container::new(element).center_x(Fill).center_y(Fill))
    }
}

/// Simple template for a Card built from the Mythic components.
#[allow(dead_code)]
pub struct Card;

impl Card {
    #[allow(dead_code)]
    pub fn build_container<'a, Message>(
        element: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a,
    {
        panel().card().build(element)
    }
}

/// For constructing charts in Mythic!
#[derive(Debug, Clone, Default)]
pub struct ExcaliburChart {
    pub chart: CartesianChart,
}

impl ExcaliburChart {
    pub fn new() -> Self {
        Self {
            chart: CartesianChart::new(),
        }
    }

    pub fn build(&self) -> Element<'_, mythic_charts::ChartMessage> {
        todo!()
    }

    // Configuring the chart
    #[allow(dead_code)]
    pub fn series_color(mut self, index: usize, color: ExcaliburColor) -> Self {
        // Convert the Mythic color into a plotters RGBA color.
        let color = color.color();
        let converted_color = plotters::style::RGBColor(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        );

        self.chart.series[index].color = converted_color;
        self
    }

    /// Add a series to the chart.
    #[allow(dead_code)]
    pub fn series(mut self, series: ChartLineSeries) -> Self {
        self.chart.series(series);
        self
    }

    // todo: this is probably inefficient
    pub fn override_series(&mut self, new_series: Vec<ChartLineSeries>) {
        self.chart.override_series(new_series);
    }

    pub fn override_points(&mut self, points_of_interest: Vec<ChartPoint>) {
        self.chart.override_points(points_of_interest);
    }

    /// Add multiple series to the chart.
    #[allow(dead_code)]
    pub fn many_series(mut self, new_series: Vec<ChartLineSeries>) -> Self {
        self.chart.extend_many_series(new_series);
        self
    }

    /// Update the x-axis range.
    pub fn update_x_range(&mut self, x_range: (f32, f32)) {
        self.chart.range.x_range = x_range;
    }

    /// Update the y-axis range.
    pub fn update_y_range(&mut self, y_range: (f32, f32)) {
        self.chart.range.y_range = y_range;
    }

    /// Add a point of interest to the chart.
    #[allow(dead_code)]
    pub fn point_of_interest(mut self, point_of_interest: ChartPoint) -> Self {
        self.chart.point_of_interest(point_of_interest);
        self
    }

    /// Add multiple points of interest to the chart.
    #[allow(dead_code)]
    pub fn points_of_interest(mut self, points_of_interest: Vec<ChartPoint>) -> Self {
        self.chart.points_of_interest(points_of_interest);
        self
    }

    /// Add an x-range to the chart.
    pub fn x_range(mut self, x_range: (f32, f32)) -> Self {
        self.chart.range.x_range = x_range;
        self
    }

    /// Add a y-range to the chart.
    pub fn y_range(mut self, y_range: (f32, f32)) -> Self {
        self.chart.range.y_range = y_range;
        self
    }

    pub fn override_ranges_flag(&mut self, flag: bool) {
        self.chart.override_ranges = flag;
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExcaliburHistogram {
    pub chart: HistogramChart,
}

impl ExcaliburHistogram {
    pub fn new() -> Self {
        Self {
            chart: HistogramChart::new(),
        }
    }

    pub fn build(&self) -> Element<'_, mythic_charts::ChartMessage> {
        todo!()
    }

    pub fn override_data(mut self, data: BTreeMap<u32, u32>) -> Self {
        self.chart.data = data;
        self
    }

    pub fn x_range(mut self, x_range: (f32, f32)) -> Self {
        self.chart.range.x_range = x_range;
        self
    }

    pub fn y_range(mut self, y_range: (f32, f32)) -> Self {
        self.chart.range.y_range = y_range;
        self
    }

    pub fn notable_bars(mut self, notable_bars: BTreeMap<u32, u32>) -> Self {
        self.chart.notable_bars = notable_bars;
        self
    }
}

/// A customizable tooltip with Mythic styling.
///
/// How to use this component:
/// - Create a new or default tooltip.
/// - Edit the text styling using the custom methods or templated methods like
///   `caption`.
/// - Edit the position of the tooltip using the `position` method.
/// - Edit the element that is hovered over to display the tooltip using the
///   `custom_content` method.
/// - Use the template tooltips like `info`, which renders an info "i" as the
///   element to hover over.
/// - Build the tooltip using the `build` method.
#[derive(Debug, Clone)]
pub struct ExcaliburTooltip {
    position: iced::widget::tooltip::Position,
    padding: iced::Pixels,
    gap: f32,
    font: Option<iced::Font>,
    text_size: Option<Typography>,
    text_color: ExcaliburColor,
    snap_in_viewport: bool,
    /// Customize the element that is hovered over to display the tooltip.
    custom_element: ExcaliburText,
}

impl Default for ExcaliburTooltip {
    fn default() -> Self {
        Self {
            position: iced::widget::tooltip::Position::Top,
            padding: 0.0.into(),
            gap: 0.0,
            font: None,
            text_size: None,
            text_color: ExcaliburColor::Label(LabelColors::Primary),
            snap_in_viewport: true,
            custom_element: label(icon_to_char(Bootstrap::Info))
                .icon()
                .secondary()
                .caption(),
        }
    }
}

#[allow(dead_code)]
impl ExcaliburTooltip {
    pub fn new() -> Self {
        Self::default()
    }

    /// Uses the `self.custom_element` to build the tooltip. This must be set,
    /// defaults to the [`Info`] Icon.
    pub fn build<'a, Message>(
        self,
        tooltip_content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> iced::widget::Tooltip<'a, Message>
    where
        Message: 'a,
    {
        let custom = self.custom_element.clone();
        self.build_custom(custom, tooltip_content)
    }

    pub fn build_custom<'a, Message>(
        self,
        element: impl Into<Element<'a, Message>>,
        tooltip_content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> iced::widget::Tooltip<'a, Message>
    where
        Message: 'a,
    {
        let builder = tooltip(element, tooltip_content, self.position)
            .padding(self.padding)
            .gap(self.gap)
            .snap_within_viewport(self.snap_in_viewport);

        builder.style(move |_theme| {
            ExcaliburContainer::default()
                .top()
                .text_color(self.text_color)
                .light_border()
                .theme()
        })
    }

    pub fn position(mut self, position: iced::widget::tooltip::Position) -> Self {
        self.position = position;
        self
    }

    pub fn icon(mut self) -> Self {
        self.font = Some(ExcaliburFonts::Icon.into());
        self
    }

    pub fn branding(mut self) -> Self {
        self.font = Some(ExcaliburFonts::Branding.into());
        self
    }

    pub fn caption(mut self) -> Self {
        self.text_size = Some(Typography::Caption);
        self
    }

    pub fn caption2(mut self) -> Self {
        self.text_size = Some(Typography::Caption2);
        self
    }

    pub fn padding(mut self, padding: impl Into<iced::Pixels>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn top(self) -> Self {
        self.position(iced::widget::tooltip::Position::Top)
    }

    pub fn bottom(self) -> Self {
        self.position(iced::widget::tooltip::Position::Bottom)
    }

    pub fn left(self) -> Self {
        self.position(iced::widget::tooltip::Position::Left)
    }

    pub fn right(self) -> Self {
        self.position(iced::widget::tooltip::Position::Right)
    }

    pub fn text_color(mut self, color: impl Into<ExcaliburColor>) -> Self {
        self.text_color = color.into();
        self
    }

    pub fn secondary(self) -> Self {
        self.text_color(ExcaliburColor::Label(LabelColors::Secondary))
    }

    pub fn tertiary(self) -> Self {
        self.text_color(ExcaliburColor::Label(LabelColors::Tertiary))
    }

    pub fn quaternary(self) -> Self {
        self.text_color(ExcaliburColor::Label(LabelColors::Quaternary))
    }

    pub fn disabled(self) -> Self {
        self.text_color(ExcaliburColor::Label(LabelColors::Disabled))
    }

    pub fn highlight(self) -> Self {
        self.text_color(ExcaliburColor::Label(LabelColors::Highlight))
    }

    pub fn custom_content(mut self, element: ExcaliburText) -> Self {
        self.custom_element = element;
        self
    }

    // Template tooltips.
    // Color must be specified prior to consuming the tooltip.
    pub fn info(mut self) -> Self {
        let mut element = label(icon_to_char(Bootstrap::Info))
            .icon()
            .custom_color(self.text_color);

        if let Some(size) = self.text_size {
            element = element.custom_typography(size)
        }

        self.custom_element = element;
        self
    }
}
