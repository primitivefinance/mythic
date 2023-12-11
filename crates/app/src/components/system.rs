//! Entire Excalibur component system.

use iced::Font;

use super::{
    chart::{
        basic_liq_dist_curve, basic_log_normal_curve, coords_to_line_series, CartesianChart,
        ChartLineSeries, ChartPoint,
    },
    *,
};

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

const BG3: Color = Color::from_rgb(
    0x28 as f32 / 255.0,
    0x28 as f32 / 255.0,
    0x28 as f32 / 255.0,
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

/// Available colors in Excalibur
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
    #[default]
    Primary,
    Success,
    Danger,
    Label(LabelColors),
    Quantitative(QuantitativeColors),
    Custom(Color),
}

impl ExcaliburColor {
    pub fn color(&self) -> Color {
        match self {
            ExcaliburColor::Background1 => BG1,
            ExcaliburColor::Background2 => BG2,
            ExcaliburColor::Background3 => BG3,
            ExcaliburColor::Primary => BLUE,
            ExcaliburColor::Success => GREEN,
            ExcaliburColor::Danger => RED,
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
    Caption2 = 10,
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
    Symbol,
    Custom(iced::Font),
}

impl ExcaliburFonts {
    pub fn font(&self) -> Font {
        match self {
            ExcaliburFonts::UI => UI_FONT,
            ExcaliburFonts::UIBold => UI_FONT_BOLD,
            ExcaliburFonts::Branding => BRAND_FONT,
            ExcaliburFonts::Symbol => SYMBOL_FONT,
            ExcaliburFonts::Custom(font) => *font,
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

/// For constructing any text rendered in Excalibur.
pub fn label<'a>(value: &str) -> ExcaliburText {
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
            .horizontal_alignment(self.horizontal_alignment)
            .vertical_alignment(self.vertical_alignment)
    }

    // Class

    /// Formats the text based on float value.
    pub fn quantitative(self) -> Self {
        // todo: this is probably very dangerous!
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
        let mut color = QuantitativeColors::Hundreds;

        let percentage_value = value * 100.0;

        let value = if percentage_value < 10.0 {
            color = QuantitativeColors::Hundreds;
            format!("{:.2}%", percentage_value)
        } else if percentage_value < 50.0 {
            color = QuantitativeColors::Thousands;
            format!("{:.2}%", percentage_value)
        } else if percentage_value < 95.0 {
            color = QuantitativeColors::Millions;
            format!("{:.2}%", percentage_value)
        } else {
            color = QuantitativeColors::Billions;
            format!("{:.2}%", percentage_value)
        };

        Self {
            value,
            color: ExcaliburColor::Quantitative(color),
            ..self
        }
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

/// For constructing Excalibur containers.
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
}

impl Default for ExcaliburContainer {
    fn default() -> Self {
        Self {
            background: ExcaliburColor::Background1,
            border_radius: Sizes::Sm.into(),
            border_width: 0.0,
            border_color: ExcaliburColor::Custom(Color::WHITE),
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
    pub fn bottom(mut self) -> Self {
        self.background = ExcaliburColor::Background1;
        self
    }

    /// The layer between the bottom and top layers.
    pub fn middle(mut self) -> Self {
        self.background = ExcaliburColor::Background2;
        self
    }

    /// The layer that is closest and therefore the lightest.
    pub fn top(mut self) -> Self {
        self.background = ExcaliburColor::Background3;
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
    pub fn background_rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.background = ExcaliburColor::Custom(iced::Color::from_rgb(r, g, b));
        self
    }

    // Border radius

    pub fn sharp(mut self) -> Self {
        self.border_radius = 0.0.into();
        self
    }

    pub fn round(mut self, size: Sizes) -> Self {
        self.border_radius = size.into();
        self
    }

    pub fn border_radius(mut self, size: BorderRadius) -> Self {
        self.border_radius = size.into();
        self
    }

    // Border

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

    pub fn black_border(mut self) -> Self {
        self.border_color = ExcaliburColor::Custom(Color::BLACK);
        self.border_width = 1.0;
        self
    }

    pub fn white_border(mut self) -> Self {
        self.border_color = ExcaliburColor::Custom(Color::WHITE);
        self.border_width = 1.0;
        self
    }

    // Presets

    pub fn card(mut self) -> Self {
        self.background = ExcaliburColor::Background3;
        self.border_radius = Sizes::Sm.into();
        self.border_color = ExcaliburColor::Custom(GRAY_600);
        self.border_width = 1.0;
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

    pub fn style(mut self, style: CustomButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn primary(self) -> Self {
        let color = ExcaliburColor::Label(LabelColors::Primary).into();
        let border_radius = 3.0.into();
        let disabled_color = ExcaliburColor::Label(LabelColors::Disabled).into();
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
            .text_color(disabled_color)
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
}

/// Constructs a table using the table builder to be used across Excalibur.
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
                            .border_bottom(ExcaliburContainer::default().white_border().theme()),
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
}

/// Simple template for a Card built from the Excalibur components.
pub struct Card;

impl Card {
    pub fn new<'a, Message>(element: impl Into<Element<'a, Message>>) -> Container<'a, Message>
    where
        Message: 'a,
    {
        panel().card().build(element).into()
    }
}

/// For constructing charts in Excalibur!
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

    pub fn build(&self) -> Element<'_, chart::Message> {
        self.chart.view()
    }

    // Configuring the chart

    pub fn series_color(mut self, index: usize, color: ExcaliburColor) -> Self {
        // Convert the Excalibur color into a plotters RGBA color.
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
    pub fn point_of_interest(mut self, point_of_interest: ChartPoint) -> Self {
        self.chart.point_of_interest(point_of_interest);
        self
    }

    /// Add multiple points of interest to the chart.
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

    // Chart templates
    pub fn rmm_trading_fn(mut self) -> Self {
        let log_normal_plot = basic_log_normal_curve();
        let mut series = coords_to_line_series(log_normal_plot);
        series.legend = "Log Normal".to_string();

        let liq_dist_plot = basic_liq_dist_curve();
        let mut series2 = coords_to_line_series(liq_dist_plot);
        series2.legend = "Liq. Dist.".to_string();
        series2.color = plotters::style::colors::full_palette::DEEPPURPLE_400;

        let lines: Vec<ChartLineSeries> = vec![series, series2];

        self.chart.extend_many_series(lines);
        self = self.x_range((-0.1, 1.0));
        self = self.y_range((-0.1, 1.0));

        self
    }
}
