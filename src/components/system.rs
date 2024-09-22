//! Entire Mythic component system.

use std::collections::BTreeMap;

use super::*;
use iced::{
    theme::Palette,
    widget::{component, text_input, tooltip, Component, Text},
    Color, Font,
};
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

        Theme::Custom(Box::new(iced::theme::Custom::new(palette)))
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
            ExcaliburFonts::Icon => iced_aw::ICON_FONT,
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
    pub horizontal_alignment: alignment::Horizontal,
    pub vertical_alignment: alignment::Vertical,
}

impl Default for ExcaliburText {
    fn default() -> Self {
        Self {
            value: "text".to_string(),
            color: ExcaliburColor::Label(LabelColors::Primary),
            font: ExcaliburFonts::UI,
            size: Typography::Body,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
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
            .style(self.color.color())
            .font(self.font.font())
            .size(self.size)
            .horizontal_alignment(self.horizontal_alignment)
            .vertical_alignment(self.vertical_alignment)
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
            horizontal_alignment: alignment::Horizontal::Left,
            ..self
        }
    }

    /// Sets text horizontal alignment to center.
    pub fn center(self) -> Self {
        Self {
            horizontal_alignment: alignment::Horizontal::Center,
            ..self
        }
    }

    /// Sets text horizontal alignment to right.
    pub fn right(self) -> Self {
        Self {
            horizontal_alignment: alignment::Horizontal::Right,
            ..self
        }
    }

    /// Sets text vertical alignment to top.
    pub fn top(self) -> Self {
        Self {
            vertical_alignment: alignment::Vertical::Top,
            ..self
        }
    }

    /// Sets text vertical alignment to middle.
    pub fn middle(self) -> Self {
        Self {
            vertical_alignment: alignment::Vertical::Center,
            ..self
        }
    }

    /// Sets text vertical alignment to bottom.
    pub fn bottom(self) -> Self {
        Self {
            vertical_alignment: alignment::Vertical::Bottom,
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

impl container::StyleSheet for ExcaliburContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        let background = self.background.color();
        let border_color = self.border_color.color();
        let border_radius = self.border_radius;
        let border_width = self.border_width;

        container::Appearance {
            background: Some(iced::Background::Color(background)),
            border_radius,
            border_width,
            border_color,
            ..Default::default()
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
        Container::new(element).style(self.theme())
    }

    pub fn theme(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(self))
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
pub struct ExcaliburButton {
    pub style: CustomButtonStyle,
}

impl ExcaliburButton {
    pub fn new() -> Self {
        Self {
            style: CustomButtonStyle::new(),
        }
    }

    pub fn build<'a, Message>(self, element: impl Into<Element<'a, Message>>) -> Button<'a, Message>
    where
        Message: 'a,
    {
        button(Container::new(element).center_x().center_y()).style(self.theme())
    }

    pub fn theme(self) -> iced::theme::Button {
        self.style.as_custom()
    }

    #[allow(dead_code)]
    pub fn style(mut self, style: CustomButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn active(mut self) -> Self {
        self.style.current_state = ButtonState::Active;
        self
    }
    #[allow(dead_code)]
    pub fn hovered(mut self) -> Self {
        self.style.current_state = ButtonState::Hovered;
        self
    }
    #[allow(dead_code)]
    pub fn pressed(mut self) -> Self {
        self.style.current_state = ButtonState::Pressed;
        self
    }
    // do we need disabled button state?
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.style.current_state = ButtonState::Disabled;
        self
    }

    /// Overrides the active button state's background color.
    pub fn background(mut self, color: ExcaliburColor) -> Self {
        self.style = self.style.background(Some(color.into()));
        self
    }

    /// Overrides all button states with a border radius.
    pub fn border_radius(self, border_radius: BorderRadius) -> Self {
        let style = self
            .style
            .active()
            .border_radius(border_radius)
            .hovered()
            .border_radius(border_radius)
            .pressed()
            .border_radius(border_radius)
            .disabled()
            .border_radius(border_radius);

        Self { style }
    }

    pub fn primary(self) -> Self {
        let color = ExcaliburColor::Label(LabelColors::Primary).into();
        let border_radius = 3.0.into();
        let disabled_color = ExcaliburColor::Label(LabelColors::Disabled);
        let style = CustomButtonStyle::primary(&ExcaliburTheme::theme())
            .text_color(color)
            .border_radius(border_radius)
            .hovered()
            .text_color(color)
            .border_radius(border_radius)
            .pressed()
            .text_color(color)
            .border_radius(border_radius)
            .disabled()
            .background(Some(ExcaliburColor::PrimaryDisabled.into()))
            .text_color(disabled_color.into())
            .border_radius(border_radius);
        Self { style }
    }

    pub fn danger(self) -> Self {
        let color = ExcaliburColor::Label(LabelColors::Primary).into();
        let border_radius = 3.0.into();
        let disabled_color = ExcaliburColor::Label(LabelColors::Disabled).into();
        let style = CustomButtonStyle::destructive(&ExcaliburTheme::theme())
            .text_color(color)
            .border_radius(border_radius)
            .hovered()
            .text_color(color)
            .border_radius(border_radius)
            .pressed()
            .text_color(color)
            .border_radius(border_radius)
            .disabled()
            .background_color(ExcaliburColor::Custom(Color::from_rgba(0.8, 0.2, 0.2, 0.1)).into())
            .text_color(disabled_color)
            .border_radius(border_radius);
        Self { style }
    }

    pub fn transparent(self) -> Self {
        let background = Color::TRANSPARENT;
        let semi_transparent_background = Color::from_rgba(40.0, 40.0, 40.0, 0.05);
        let color = ExcaliburColor::Label(LabelColors::Primary).into();
        let disabled_color = ExcaliburColor::Label(LabelColors::Disabled).into();
        let border_radius = 3.0.into();

        let style = CustomButtonStyle::primary(&ExcaliburTheme::theme())
            .background(Some(background.into()))
            .text_color(color)
            .border_radius(border_radius)
            .hovered()
            .background(Some(semi_transparent_background.into()))
            .text_color(color)
            .border_radius(border_radius)
            .pressed()
            .background(Some(semi_transparent_background.into()))
            .text_color(color)
            .border_radius(border_radius)
            .disabled()
            .background(Some(background.into()))
            .text_color(disabled_color)
            .border_radius(border_radius);
        Self { style }
    }

    /// Button that can be pressed as a selection item.
    pub fn selectable(self) -> Self {
        let style = CustomButtonStyle::primary(&ExcaliburTheme::theme())
            .background(Some(ExcaliburColor::Background3.into()))
            .border_color(ExcaliburColor::Custom(GRAY_600).into())
            .border_width(1.0)
            .text_color(ExcaliburColor::Label(LabelColors::Primary).into())
            .hovered()
            .background(Some(ExcaliburColor::Background4.into()))
            .border_color(ExcaliburColor::Custom(GRAY_600).into())
            .border_width(1.0)
            .text_color(ExcaliburColor::Label(LabelColors::Primary).into())
            .pressed()
            .background(Some(ExcaliburColor::Background2.into()))
            .border_color(ExcaliburColor::Custom(GRAY_600).into())
            .border_width(1.0)
            .text_color(ExcaliburColor::Label(LabelColors::Primary).into())
            .disabled()
            .background(Some(ExcaliburColor::Background2.into()))
            .border_color(ExcaliburColor::Custom(GRAY_500).into())
            .border_width(1.0)
            .text_color(ExcaliburColor::Label(LabelColors::Disabled).into());

        Self { style }
    }
}

/// Constructs a table using the table builder to be used across Mythic.
/// Uses predefined sizes, colors, and fonts.
/// todo: static lifetime kind of bad?
pub struct ExcaliburTable<Message: Default>
where
    Message: 'static,
{
    pub col: ColumnBuilder<Message>,
    pub row: RowBuilder<Message>,
    pub headers: Vec<String>,
}

impl<Message: Default + Clone> ExcaliburTable<Message> {
    pub fn new() -> Self {
        Self {
            col: ColumnBuilder::new(),
            row: RowBuilder::new(),
            headers: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn build(self, cells: Vec<CellBuilder<Message>>) -> TableBuilder<Message> {
        TableBuilder::new()
            .padding_cell(Sizes::Md)
            .padding_cell_internal(Sizes::Xs)
            .column(
                self.col
                    .headers(self.headers)
                    .rows(vec![self.row.cells(cells)]),
            )
    }

    pub fn build_custom(self, cells: Vec<Vec<CellBuilder<Message>>>) -> TableBuilder<Message> {
        TableBuilder::new()
            .padding_cell(Sizes::Md)
            .padding_cell_internal(Sizes::Xs)
            .column(
                self.col
                    .headers(self.headers)
                    .header_row(
                        RowBuilder::new()
                            .border_bottom(ExcaliburContainer::default().light_border().theme()),
                    )
                    .rows(
                        cells
                            .into_iter()
                            .map(|cells| {
                                self.row
                                    .clone()
                                    .border_bottom(
                                        ExcaliburContainer::default().light_border().theme(),
                                    )
                                    .cells(cells)
                            })
                            .collect(),
                    ),
            )
    }

    #[allow(dead_code)]
    pub fn build_empty(self) -> TableBuilder<Message> {
        TableBuilder::new()
            .padding_cell(Sizes::Xl)
            .padding_cell_internal(Sizes::Xl)
            .column(
                self.col
                    .headers(self.headers)
                    .header_row(
                        RowBuilder::new()
                            .border_bottom(ExcaliburContainer::default().white_border().theme()),
                    )
                    .rows(vec![RowBuilder::new().cell(CellBuilder::new().child(
                        button("Create position").on_press(Message::default()),
                    ))])
                    .row(
                        RowBuilder::new()
                            .border_bottom(ExcaliburContainer::default().light_border().theme()),
                    )
                    .row(
                        RowBuilder::new().cell(
                            CellBuilder::new().child(
                                label("Last sync: 12:00 Dec. 6, 2023.")
                                    .caption()
                                    .secondary()
                                    .build(),
                            ),
                        ),
                    ),
            )
    }

    // Add a header to the table.
    pub fn header(mut self, header: &str) -> Self {
        self.headers.push(header.to_string());
        self
    }

    pub fn headers(mut self, headers: Vec<impl ToString>) -> Self {
        self.headers = headers.iter().map(ToString::to_string).collect();
        self
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
        self.chart.view()
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
        self.chart.view()
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

#[derive(Clone)]
pub struct ExcaliburInputBuilder {
    padding: Option<Padding>,
    placeholder: Option<String>,
    font: Option<iced::Font>,
    size: Option<f32>,
    icon: Option<iced::widget::text_input::Icon<iced::Font>>,
    style: CustomInputStyle,
    width: Length,
}

impl Default for ExcaliburInputBuilder {
    fn default() -> Self {
        Self {
            padding: None,
            placeholder: None,
            font: None,
            size: None,
            icon: None,
            style: CustomInputStyle::new(),
            width: Length::Shrink,
        }
    }
}

impl ExcaliburInputBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build<'a, Message>(
        self,
        value: Option<String>,
        on_change: impl Fn(Option<String>) -> Message + 'a,
    ) -> ExcaliburInput<'a, Message> {
        ExcaliburInput::new(
            value,
            on_change,
            self.padding,
            self.placeholder,
            None,
            None,
            self.size,
            self.font,
            self.icon,
        )
        .style(move || self.style.build())
        .width(self.width)
    }

    pub fn size(mut self, size: Typography) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn light_border(self) -> Self {
        self.style
            .active()
            .border_color(ExcaliburColor::Custom(GRAY_600))
            .border_width(1.0)
            .value_color(ExcaliburColor::Label(system::LabelColors::Highlight))
            .placeholder_color(ExcaliburColor::Label(system::LabelColors::Tertiary))
            .hovered()
            .border_color(ExcaliburColor::Custom(GRAY_600))
            .border_width(1.0)
            .value_color(ExcaliburColor::Label(system::LabelColors::Highlight))
            .placeholder_color(ExcaliburColor::Label(system::LabelColors::Tertiary))
            .background(ExcaliburColor::Background4);

        self
    }

    pub fn border_radius(self, radius: BorderRadius) -> Self {
        self.style.active().border_radius(radius);
        self.style.focused().border_radius(radius);
        self.style.hovered().border_radius(radius);
        self.style.disabled().border_radius(radius);
        self
    }

    pub fn icon(mut self, icon: iced::widget::text_input::Icon<iced::Font>) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Change(String),
    Paste(String),
    Submit,
}

pub struct ExcaliburInput<'a, Message> {
    placeholder: String,
    value: Option<String>,
    is_secure: bool,
    font: Option<iced::Font>,
    width: Length,
    padding: Padding,
    size: Option<f32>,
    line_height: text::LineHeight,
    on_input: Option<Box<dyn Fn(Option<String>) -> Message + 'a>>,
    on_paste: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    icon: Option<iced::widget::text_input::Icon<iced::Font>>,
    style: Option<Box<dyn Fn() -> <iced::Theme as iced::widget::text_input::StyleSheet>::Style>>,
}

impl<'a, Message> ExcaliburInput<'a, Message> {
    // TODO: this is a bit of a mess
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        value: Option<String>,
        on_input: impl Fn(Option<String>) -> Message + 'a,
        padding: Option<Padding>,
        placeholder: Option<String>,
        on_paste: Option<Box<dyn Fn(String) -> Message + 'a>>,
        on_submit: Option<Message>,
        size: Option<f32>,
        font: Option<iced::Font>,
        icon: Option<iced::widget::text_input::Icon<iced::Font>>,
    ) -> Self {
        Self {
            placeholder: placeholder.unwrap_or("".to_string()),
            value,
            is_secure: false,
            font,
            width: Length::Shrink,
            padding: padding.unwrap_or(0.0.into()),
            size,
            line_height: text::LineHeight::from(1.0),
            on_input: Some(Box::new(on_input)),
            on_paste,
            on_submit,
            icon,
            style: Some(Box::new(|| {
                <iced::Theme as iced::widget::text_input::StyleSheet>::Style::default()
            })),
        }
    }

    pub fn style(
        mut self,
        style: impl Fn() -> <iced::Theme as iced::widget::text_input::StyleSheet>::Style + 'static,
    ) -> Self {
        self.style = Some(Box::new(style));
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<'a, Message> Component<Message, iced::Renderer> for ExcaliburInput<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = InputEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Self::Event::Change(value) => {
                self.value = Some(value.clone());

                if let Some(on_input) = &self.on_input {
                    if value.is_empty() {
                        Some((on_input)(None))
                    } else {
                        let parsed_value = value.parse();
                        match parsed_value {
                            Ok(parsed_value) => Some((on_input)(Some(parsed_value))),
                            Err(e) => {
                                tracing::warn!("Error parsing input: {:?}", e);
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Self::Event::Paste(value) => {
                self.value = Some(value.clone());

                if let Some(on_paste) = &self.on_paste {
                    if value.is_empty() {
                        Some((on_paste)(value))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Self::Event::Submit => self.on_submit.as_ref().cloned(),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, iced::Renderer> {
        let mut input = text_input(
            &self.placeholder,
            &self
                .value
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or("".to_string()),
        )
        .on_input(Self::Event::Change)
        .padding(self.padding)
        .width(self.width)
        .line_height(self.line_height)
        .on_submit(Self::Event::Submit)
        .on_paste(Self::Event::Paste);

        if let Some(size) = self.size {
            input = input.size(size);
        }

        if let Some(icon) = &self.icon {
            input = input.icon(icon.clone());
        }

        if let Some(font) = &self.font {
            input = input.font(*font);
        }

        if self.is_secure {
            input = input.password();
        }

        if let Some(style) = &self.style {
            input = input.style(style());
        }

        input.into()
    }
}

impl<'a, Event> From<ExcaliburInput<'a, Event>> for Element<'a, Event, iced::Renderer>
where
    Event: 'a + Clone,
{
    fn from(config_input: ExcaliburInput<'a, Event>) -> Self {
        component(config_input)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InputState {
    #[default]
    Active,
    Focused,
    Hovered,
    Disabled,
}

#[derive(Debug, Clone, Copy)]
pub struct CustomInputStyle {
    pub active: text_input::Appearance,
    pub focused: text_input::Appearance,
    pub hovered: text_input::Appearance,
    pub disabled: text_input::Appearance,
    pub current: InputState,
    pub placeholder_color: Color,
    pub value_color: Color,
    pub disabled_color: Color,
    pub selection_color: Color,
}

impl Default for CustomInputStyle {
    fn default() -> Self {
        let default = text_input::Appearance {
            background: ExcaliburColor::Transparent.into(),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: ExcaliburColor::Transparent.into(),
            icon_color: ExcaliburColor::Label(LabelColors::Primary).into(),
        };
        Self {
            active: default,
            focused: default,
            hovered: default,
            disabled: default,
            current: InputState::Active,
            placeholder_color: ExcaliburColor::Label(LabelColors::Placeholder).into(),
            value_color: ExcaliburColor::Label(LabelColors::Primary).into(),
            disabled_color: ExcaliburColor::Label(LabelColors::Disabled).into(),
            selection_color: ExcaliburColor::Label(LabelColors::Tertiary).into(),
        }
    }
}

impl CustomInputStyle {
    pub fn new() -> Self {
        Self::default()
    }

    // Edit the different states

    pub fn active(mut self) -> Self {
        self.current = InputState::Active;
        self
    }

    pub fn focused(mut self) -> Self {
        self.current = InputState::Focused;
        self
    }

    pub fn hovered(mut self) -> Self {
        self.current = InputState::Hovered;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.current = InputState::Disabled;
        self
    }

    // Edit the colors of the text

    pub fn placeholder_color(mut self, color: ExcaliburColor) -> Self {
        self.placeholder_color = color.into();
        self
    }

    pub fn value_color(mut self, color: ExcaliburColor) -> Self {
        self.value_color = color.into();
        self
    }
    #[allow(dead_code)]
    pub fn disabled_color(mut self, color: ExcaliburColor) -> Self {
        self.disabled_color = color.into();
        self
    }
    #[allow(dead_code)]
    pub fn selection_color(mut self, color: ExcaliburColor) -> Self {
        self.selection_color = color.into();
        self
    }

    // Edit the values of different states

    pub fn background(mut self, color: ExcaliburColor) -> Self {
        match self.current {
            InputState::Active => self.active.background = color.into(),
            InputState::Focused => self.focused.background = color.into(),
            InputState::Hovered => self.hovered.background = color.into(),
            InputState::Disabled => self.disabled.background = color.into(),
        }
        self
    }

    pub fn border_radius(mut self, radius: BorderRadius) -> Self {
        match self.current {
            InputState::Active => self.active.border_radius = radius,
            InputState::Focused => self.focused.border_radius = radius,
            InputState::Hovered => self.hovered.border_radius = radius,
            InputState::Disabled => self.disabled.border_radius = radius,
        }
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        match self.current {
            InputState::Active => self.active.border_width = width,
            InputState::Focused => self.focused.border_width = width,
            InputState::Hovered => self.hovered.border_width = width,
            InputState::Disabled => self.disabled.border_width = width,
        }
        self
    }

    pub fn border_color(mut self, color: ExcaliburColor) -> Self {
        match self.current {
            InputState::Active => self.active.border_color = color.into(),
            InputState::Focused => self.focused.border_color = color.into(),
            InputState::Hovered => self.hovered.border_color = color.into(),
            InputState::Disabled => self.disabled.border_color = color.into(),
        }
        self
    }
    #[allow(dead_code)]
    pub fn icon_color(mut self, color: ExcaliburColor) -> Self {
        match self.current {
            InputState::Active => self.active.icon_color = color.into(),
            InputState::Focused => self.focused.icon_color = color.into(),
            InputState::Hovered => self.hovered.icon_color = color.into(),
            InputState::Disabled => self.disabled.icon_color = color.into(),
        }
        self
    }

    pub fn build(&self) -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::new(*self))
    }
}

impl text_input::StyleSheet for CustomInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        self.active
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        self.focused
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        self.hovered
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        self.disabled
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.placeholder_color
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.value_color
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.selection_color
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.disabled_color
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
            custom_element: label(icon_to_char(Icon::Info)).icon().secondary().caption(),
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
        tooltip_content: impl ToString,
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
        tooltip_content: impl ToString,
    ) -> iced::widget::Tooltip<'a, Message>
    where
        Message: 'a,
    {
        let mut builder = tooltip(element, tooltip_content, self.position)
            .padding(self.padding)
            .gap(self.gap)
            .snap_within_viewport(self.snap_in_viewport);

        if let Some(font) = self.font {
            builder = builder.font(font);
        }

        if let Some(text_size) = self.text_size {
            builder = builder.size(text_size);
        }

        builder.style(
            ExcaliburContainer::default()
                .top()
                .text_color(self.text_color)
                .light_border()
                .theme(),
        )
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
        let mut element = label(icon_to_char(Icon::Info))
            .icon()
            .custom_color(self.text_color);

        if let Some(size) = self.text_size {
            element = element.custom_typography(size)
        }

        self.custom_element = element;
        self
    }
}
