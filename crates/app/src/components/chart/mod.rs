use cfmm_math::trading_functions::rmm::{
    compute_half_sigma_power_2_tau, compute_ln_s_div_k, compute_sigma_sqrt_tau,
    compute_y_given_x_rust,
};
use iced::{
    event,
    mouse::{Cursor, Event},
    widget::canvas::{self},
    Element, Length, Point,
};
use plotters::{coord::ReverseCoordTranslate, prelude::*, style::colors};
use plotters_iced::{Chart, ChartWidget};
use reikna::integral::*;
use statrs::distribution::Continuous;

/// A point to plot on the chart.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartPoint {
    pub x: f32,
    pub y: f32,
    pub diameter: i32,
    pub color: RGBColor,
}

impl Default for ChartPoint {
    fn default() -> Self {
        Self {
            x: 0.5,
            y: 0.5,
            diameter: 3,
            color: colors::full_palette::AMBER_500,
        }
    }
}

/// A line series to plot on the chart.
/// - Each line series is a collection of lines.
/// - Each line is a collection of points.
/// - Each point is a tuple of (x, y) coordinates.
/// - Each coordinate is a f32.
/// - Each line series has a color and width.
/// - Each line series has a legend.
#[derive(Debug, Clone)]
pub struct ChartLineSeries {
    pub lines: Vec<ChartLine>,
    pub color: RGBColor,
    pub width: u32,
    pub legend: String,
}

impl Default for ChartLineSeries {
    fn default() -> Self {
        Self {
            lines: vec![((0.0, 0.0), (1.0, 1.0)).into()],
            color: colors::full_palette::TEAL_A400,
            width: 2,
            legend: "Series".to_string(),
        }
    }
}

/// A line to plot on the chart.
/// The line is drawn from (x1, y1) to (x2, y2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartLine {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Default for ChartLine {
    fn default() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CartesianChart {
    /// The axis ranges for the chart.
    pub range: CartesianRanges,
    /// The points to plot on the chart.
    pub points: Vec<ChartPoint>,
    /// The line series' to plot on the chart.
    pub series: Vec<ChartLineSeries>,
    /// The space between the axis regions of the chart and the chart canvas.
    /// The order is clockwise from top: [top, right, bottom, left].
    pub axis_margin: [u32; 4],
    /// The axis label text styling. todo: support beyond just color.
    pub axis_text_style: RGBAColor,
    /// The axis mesh styling. i.e. y = 0, x = 0.
    pub axis_mesh_style: RGBAColor,
    /// The mesh grid styling.
    pub mesh_grid_style: RGBAColor,
    /// Maximum quantity of labels on the y-axis.
    pub y_labels: usize,
    /// Maximum quantity of labels on the x-axis.
    pub x_labels: usize,
    /// Maximum amount of in between mesh lines between labels.
    pub max_light_lines: usize,
    /// Style of the y-axis label container, pinned to the end of the x
    /// range.
    pub y_label_container_style: RGBAColor,
    /// Style of the x-axis label container, pinned to the start of the y range.
    pub x_label_container_style: RGBAColor,
    /// Color of the label text.
    pub label_text_style: RGBAColor,
    /// Font size of the labe text.
    pub label_font_size: f32,
    /// Color of the grid line emitted from the cursor position on the x-axis.
    pub x_cursor_line_style: RGBAColor,
    /// Color of the grid line emitted from the cursor position on the y-axis.
    pub y_cursor_line_style: RGBAColor,
    /// Background color of the legend.
    pub legend_background_color: RGBAColor,
    /// Legend margin
    pub legend_margin: u32,
    /// Legend text font size.
    pub legend_font_size: f32,
    /// Origin x-axis line color.
    pub origin_x_line_style: RGBAColor,
    /// Origin y-axis line color.
    pub origin_y_line_style: RGBAColor,
    /// Chart border color.
    pub border_color: RGBAColor,
    /// Background color of the point label container on hover.
    pub point_label_container_style: RGBAColor,
}

impl Default for CartesianChart {
    fn default() -> Self {
        Self {
            range: CartesianRanges::default(),
            points: vec![(0.5, 0.5).into()],
            series: vec![((0.0, 0.0), (1.0, 1.0)).into()],
            axis_margin: [0, 30, 30, 0],
            axis_text_style: colors::full_palette::GREY_600.into(),
            axis_mesh_style: colors::TRANSPARENT,
            mesh_grid_style: RGBAColor(0x1c, 0x1c, 0x1c, 0.8),
            y_labels: 5,
            x_labels: 5,
            max_light_lines: 1,
            y_label_container_style: RGBAColor(0x1c, 0x1c, 0x1c, 0.8),
            x_label_container_style: RGBAColor(0x1c, 0x1c, 0x1c, 0.8),
            label_text_style: colors::WHITE.into(),
            label_font_size: 15.0,
            legend_font_size: 10.0,
            x_cursor_line_style: colors::full_palette::GREY_A400.into(),
            y_cursor_line_style: colors::full_palette::GREY_A400.into(),
            legend_background_color: colors::BLACK.into(),
            legend_margin: 10,
            origin_x_line_style: colors::full_palette::GREY_600.into(),
            origin_y_line_style: colors::full_palette::GREY_600.into(),
            border_color: colors::full_palette::GREY_800.into(),
            point_label_container_style: colors::full_palette::GREY_900.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    MouseEvent(iced::mouse::Event, iced::Point),
}

impl CartesianChart {
    pub type Message = Message;

    pub fn new() -> Self {
        let log_normal_plot = basic_log_normal_curve();
        let line: Vec<((f32, f32), (f32, f32))> = log_normal_plot
            .windows(2)
            .map(|window| ((window[0].0, window[0].1), (window[1].0, window[1].1)))
            .collect();

        let line: Vec<ChartLine> = line.into_iter().map(|line| line.into()).collect();
        let mut series: ChartLineSeries = line.into();
        series.legend = "Log Normal".to_string();

        let liq_dist_plot = basic_liq_dist_curve();
        let line: Vec<((f32, f32), (f32, f32))> = liq_dist_plot
            .windows(2)
            .map(|window| ((window[0].0, window[0].1), (window[1].0, window[1].1)))
            .collect();

        let line: Vec<ChartLine> = line.into_iter().map(|line| line.into()).collect();
        let mut series2: ChartLineSeries = line.into();
        series2.legend = "Liq. Dist.".to_string();
        series2.color = colors::full_palette::DEEPPURPLE_400;

        let lines: Vec<ChartLineSeries> = vec![series, series2];

        let range = CartesianRanges {
            x_range: (-0.1, 1.0),
            y_range: (-0.1, 5.0),
        };

        Self {
            points: vec![(0.5, 0.5).into()],
            series: lines.clone(),
            range,
            ..Default::default()
        }
    }

    pub fn view(&self) -> Element<Self::Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

/// Holds the x and y axis ranges for the chart.
#[derive(Debug, Clone)]
pub struct CartesianRanges {
    /// The x-axis range.
    pub x_range: (f32, f32),
    /// The y-axis range.
    pub y_range: (f32, f32),
}

impl Default for CartesianRanges {
    fn default() -> Self {
        Self {
            x_range: (-0.1, 1.0),
            y_range: (-0.1, 1.0),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChartState {
    pub bounds: Option<iced::Rectangle>,
    pub range: CartesianRanges,
    pub original_range: Option<CartesianRanges>,
    pub relative_position: Option<Point>,
    pub cartesian_position: Option<Point>,
    pub cursor_left_click_down: bool,
    pub initial_left_click_position: Option<Point>,
}

impl Chart<Message> for CartesianChart {
    type State = ChartState;

    /// Renders a crosshair when in the chart area.
    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> iced::mouse::Interaction {
        if let Cursor::Available(point) = cursor {
            if bounds.contains(point) {
                return iced::mouse::Interaction::Crosshair;
            }
        }

        iced::mouse::Interaction::default()
    }

    /// Sets the [`State`] with the position of the cursor.
    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if let Cursor::Available(point) = cursor {
            match event {
                canvas::Event::Mouse(evt) if bounds.contains(point) => {
                    // If we scrolled the wheel, need to zoom the range in or out.
                    if let Event::WheelScrolled { delta } = evt {
                        let delta = match delta {
                            iced::mouse::ScrollDelta::Lines { y, .. } => y,
                            iced::mouse::ScrollDelta::Pixels { y, .. } => y,
                        };

                        let delta = -delta as f32;

                        let x_range = state.range.x_range.1 - state.range.x_range.0;
                        let y_range = state.range.y_range.1 - state.range.y_range.0;

                        let x_range = x_range * delta * 0.1;
                        let y_range = y_range * delta * 0.1;

                        let x_range = (
                            state.range.x_range.0 - x_range,
                            state.range.x_range.1 + x_range,
                        );
                        let y_range = (
                            state.range.y_range.0 - y_range,
                            state.range.y_range.1 + y_range,
                        );

                        state.range = CartesianRanges { x_range, y_range };

                        return (event::Status::Captured, None);
                    }

                    // Get the origin of the bounding rectangle. This is necessary because the
                    // point's coordinates are relative to the entire canvas,
                    // not just the chart's bounding rectangle.
                    let p_origin = bounds.position();

                    // Subtract the origin from the current point to get the relative position
                    // within the chart's bounding rectangle. This is done to
                    // convert the point's coordinates from being relative to the entire canvas to
                    // being relative to the chart.
                    let p = point - p_origin;

                    // Set the absolute position of the cursor inside the chart canvas bounds.
                    state.relative_position = Some(Point::new(p.x, p.y));

                    // Convert to cartesian coordinates. This is done because the chart uses a
                    // cartesian coordinate system, where the origin (0,0) is at
                    // the bottom left corner. In the canvas system, the origin is at the top left
                    // corner. Therefore, we need to subtract the y-coordinate
                    // from the height of the bounds to flip the y-axis and align the coordinate
                    // systems.
                    let p = Point::new(p.x, bounds.height - p.y);

                    state.cartesian_position = Some(Point::new(p.x, p.y));

                    if let Event::ButtonPressed(iced::mouse::Button::Left) = evt {
                        // todo: grab canvas

                        // Set down.
                        if !state.cursor_left_click_down {
                            state.cursor_left_click_down = true;
                            state.initial_left_click_position = Some(p);
                            state.original_range = Some(state.range.clone());
                        }

                        return (event::Status::Captured, None);
                    }

                    if let Event::ButtonReleased(iced::mouse::Button::Left) = evt {
                        // Set up.
                        if state.cursor_left_click_down {
                            state.cursor_left_click_down = false;
                            state.initial_left_click_position = None;
                        }

                        return (event::Status::Captured, None);
                    }

                    if state.cursor_left_click_down {
                        // Compute the delta shift for the canvas.
                        let delta_x = state.initial_left_click_position.unwrap().x
                            - state.cartesian_position.unwrap().x;
                        let delta_y = state.initial_left_click_position.unwrap().y
                            - state.cartesian_position.unwrap().y;

                        // Convert the delta shift to cartesian coordinates.
                        let original_range = state.original_range.clone().unwrap();
                        let delta_x = delta_x / bounds.width
                            * (original_range.x_range.1 - original_range.x_range.0);
                        let delta_y = delta_y / bounds.height
                            * (original_range.y_range.1 - original_range.y_range.0);

                        // Compute the new canvas ranges based on the delta shift.
                        let new_range = (
                            (
                                original_range.x_range.0 + delta_x,
                                original_range.x_range.1 + delta_x,
                            ),
                            (
                                original_range.y_range.0 + delta_y,
                                original_range.y_range.1 + delta_y,
                            ),
                        );

                        state.range = CartesianRanges {
                            x_range: (new_range.0 .0, new_range.0 .1),
                            y_range: (new_range.1 .0, new_range.1 .1),
                        };
                    }

                    return (
                        event::Status::Captured,
                        Some(Message::MouseEvent(evt, Point::new(p.x, p.y))),
                    );
                }
                _ => {}
            }
        }

        if state.bounds.is_none() {
            state.bounds = Some(bounds);
        }

        if state.original_range.is_none() {
            state.original_range = Some(self.range.clone());
        }

        (event::Status::Ignored, None)
    }

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        // Create the initial chart with the builder using the component's
        // CartesianRanges.
        let mut chart = builder
            .top_x_label_area_size(self.axis_margin[0])
            .right_y_label_area_size(self.axis_margin[1])
            .x_label_area_size(self.axis_margin[2])
            .y_label_area_size(self.axis_margin[3])
            .build_cartesian_2d(
                state.range.x_range.0..state.range.x_range.1,
                state.range.y_range.0..state.range.y_range.1,
            )
            .expect("Failed to build chart");

        // Draw the background mesh grid and axis labels.
        chart
            .configure_mesh()
            .label_style(&self.axis_text_style)
            .axis_style(self.axis_mesh_style)
            .light_line_style(self.mesh_grid_style)
            .x_labels(self.x_labels)
            .y_labels(self.y_labels)
            .max_light_lines(self.max_light_lines)
            .draw()
            .expect("Failed to draw chart mesh");

        // Draw the points on the chart.
        chart
            .draw_series(
                self.points
                    .iter()
                    .map(|p| Circle::new((p.x, p.y), p.diameter, p.color.filled())),
            )
            .expect("Failed to plot points");

        // Draw the line series' on the chart.
        for line_series in self.series.iter().cloned() {
            chart
                .draw_series(LineSeries::new(
                    line_series
                        .lines
                        .iter()
                        .flat_map(|line| vec![(line.x1, line.y1), (line.x2, line.y2)]),
                    line_series.color.filled().stroke_width(line_series.width),
                ))
                .expect("Failed to plot lines")
                .label(line_series.legend)
                .legend(move |(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 20, y)],
                        line_series.color.filled().stroke_width(line_series.width),
                    )
                });
        }

        // Draw the x-axis and y-axis indicators from the cursor position.
        if let Some(relative_position) = state.relative_position {
            // | ---------------- |
            // | - | relative | - |
            // | ---------------- |
            let relative_position = (relative_position.x, relative_position.y);

            // Get the cartesian coordinates.
            let coord_trans = chart.as_coord_spec();

            // The origin (0,0) is translated as follows:
            //
            // Original Position:
            // | ---|(0,0)-----|--|
            // | - | ^ origin  | - |
            // | --|-----------|--|
            //
            // Translated Position:
            // | --|-----------|--- |
            // | - | v origin  | - |
            // | --|(0,0)------|-- |
            let translated = coord_trans
                .reverse_translate((relative_position.0 as i32, relative_position.1 as i32))
                .unwrap_or_default();

            // Get the furthest points on each axis. This is the rightmost point on the
            // x-axis, and the bottommost point on the y-axis.
            let bound_x_start = coord_trans.get_x_range().start;
            let bound_x = coord_trans.get_x_range().end;
            let bound_y = coord_trans.get_y_range().start;
            let bound_y_end = coord_trans.get_y_range().end;

            // Set the labels to be variable depending on the position of the cursor, but
            // locked on their opposite axis.
            let x_label_coords = (translated.0, bound_y);
            let y_label_coords = (bound_x, translated.1);

            // Closure for drawing the label.
            let dot_and_label = |x: f32,
                                 y: f32,
                                 x_label: bool,
                                 container_style: ShapeStyle,
                                 text_color: RGBAColor,
                                 text_size: f32| {
                let label = match x_label {
                    true => format!("{:.2}", x),
                    false => format!("{:.2}", y),
                };

                /* let near_left = x - bound_x * 0.025 <= bound_x_start;
                let near_right = x >= bound_x - bound_x * 0.025;

                // Reversed because we are on cartesian grid!
                let near_top = y >= bound_y_end - bound_y_end * 0.025;
                let near_bottom = y <= bound_y + bound_y_end * 0.025; */
                let boundaries = near_boundaries(
                    x,
                    y,
                    (bound_x_start, bound_x),
                    (bound_y, bound_y_end),
                    0.05,
                );
                let (near_left, near_right, near_top, near_bottom) = (
                    boundaries.left,
                    boundaries.right,
                    boundaries.top,
                    boundaries.bottom
                );

                let container_width = 50;
                let container_height = 35;

                let label_container_coords: [(i32, i32); 2] = match x_label {
                    true => {
                        if near_left {
                            [(0, -container_height), (container_width, 0)]
                        } else if near_right {
                            [(-container_width, -container_height), (0, 0)]
                        } else {
                            [
                                (-container_width / 2, -container_height),
                                (container_width / 2, 0),
                            ]
                        }
                    }
                    false => {
                        if near_top {
                            [(-container_width, 0), (0, container_height)]
                        } else if near_bottom {
                            [(-container_width, -container_height), (0, 0)]
                        } else {
                            [
                                (-container_width, -container_height / 2),
                                (0, container_height / 2),
                            ]
                        }
                    }
                };

                let label_offset = match x_label {
                    true => {
                        // Clamping the labels based on the position of the label container, which
                        // is clamped, on the x-axis.
                        if near_left {
                            (
                                label_container_coords[0].0 + container_width / 4,
                                label_container_coords[0].1 + container_height / 4,
                            )
                        } else if near_right {
                            (
                                label_container_coords[0].0 + container_width / 4,
                                label_container_coords[0].1 + container_height / 4,
                            )
                        } else {
                            (
                                label_container_coords[0].0 + container_width / 4,
                                label_container_coords[0].1 + container_height / 4,
                            )
                        }
                    }
                    false => {
                        // Clamp the labels based on the position of the label container, which
                        // is clamped, on the y-axis.
                        // Adding  container_height / 6 to the x coordinate will move it inward from
                        // the leftmost edge of the container.
                        if near_top {
                            (
                                label_container_coords[0].0 + container_width / 4,
                                label_container_coords[0].1 + container_height / 4,
                            )
                        } else if near_bottom {
                            (
                                label_container_coords[0].0 + container_width / 6,
                                label_container_coords[0].1 + container_height / 4,
                            )
                        } else {
                            (
                                label_container_coords[0].0 + container_width / 4,
                                label_container_coords[0].1 + container_height / 4,
                            )
                        }
                    }
                };

                let rect = Rectangle::new(
                    label_container_coords,
                    container_style,
                );

                return EmptyElement::at((x, y))
                    + rect
                    + plotters::prelude::Text::new(
                        label,
                        label_offset,
                        ("sans-serif", text_size).into_font().color(&text_color),
                    );
            };

            // Draw a line through the y cursor position.
            chart
                .draw_series(LineSeries::new(
                    vec![(bound_x_start, translated.1), (bound_x, translated.1)].into_iter(),
                    self.y_cursor_line_style.filled(),
                ))
                .expect("Failed to plot lines");

            // Draw a line through the x cursor position.
            chart
                .draw_series(LineSeries::new(
                    vec![(translated.0, bound_y), (translated.0, bound_y_end)].into_iter(),
                    self.x_cursor_line_style.filled(),
                ))
                .expect("Failed to plot lines");

            // Draw the y axis label.
            chart
                .draw_series(PointSeries::of_element(
                    vec![y_label_coords],
                    5,
                    ShapeStyle::from(self.y_label_container_style).filled(),
                    &|c, _s, st| {
                        return dot_and_label(
                            c.0,
                            c.1,
                            false,
                            st,
                            self.label_text_style,
                            self.label_font_size,
                        );
                    },
                ))
                .expect("Failed to plot points");

            // Draw the x axis label.
            chart
                .draw_series(PointSeries::of_element(
                    vec![x_label_coords],
                    5,
                    ShapeStyle::from(self.x_label_container_style).filled(),
                    &|c, _s, st| {
                        return dot_and_label(
                            c.0,
                            c.1,
                            true,
                            st,
                            self.label_text_style,
                            self.label_font_size,
                        );
                    },
                ))
                .expect("Failed to plot points");
        }

        // Draw labels if hovering over a point.
        if let Some(relative_position) = state.relative_position {
            // Search and find if we are near a point, and which one.
            let mut nearest_point: Option<(f32, f32)> = None;

            // Get the cartesian coordinates.
            let coord_trans = chart.as_coord_spec();

            // The origin (0,0) is translated from top left to bottom left.
            let translated = coord_trans
                .reverse_translate((relative_position.x as i32, relative_position.y as i32))
                .unwrap_or_default();

            for point in &self.points {
                let distance = (point.x - translated.0).powi(2) + (point.y - translated.1).powi(2);

                if distance <= 0.001 {
                    nearest_point = Some((point.x, point.y));
                    break;
                }
            }

            if let Some(near_point) = nearest_point {
                // Draw a label near that point with its coordinates.
                chart
                    .draw_series(PointSeries::of_element(
                        vec![near_point],
                        5,
                        ShapeStyle::from(&self.point_label_container_style).filled(),
                        &|c, _s, st| {
                            let (x, y) = c;
                            let label = format!("({:.2},{:.2})", x, y);
                            let label_container_coords = [(0, -35), (85, 0)];
                            let label_offset = (
                                label_container_coords[0].0 + 10,
                                label_container_coords[0].1 + 10,
                            );
                            let rect = Rectangle::new(label_container_coords, st);

                            return EmptyElement::at((x, y))
                                + rect
                                + plotters::prelude::Text::new(
                                    label,
                                    label_offset,
                                    ("sans-serif", self.label_font_size)
                                        .into_font()
                                        .color(&self.label_text_style),
                                );
                        },
                    ))
                    .expect("Failed to plot points");
            }
        }

        let coord_trans = chart.as_coord_spec();
        let x_range = coord_trans.get_x_range();
        let y_range = coord_trans.get_y_range();

        // Draw the origin line on the x-axis.
        chart
            .draw_series(LineSeries::new(
                vec![(x_range.start, 0.0), (x_range.end, 0.0)].into_iter(),
                self.origin_x_line_style.filled(),
            ))
            .expect("Failed to plot lines");

        // Draw the origin line on the y-axis.
        chart
            .draw_series(LineSeries::new(
                vec![(0.0, y_range.start), (0.0, y_range.end)].into_iter(),
                self.origin_y_line_style.filled(),
            ))
            .expect("Failed to plot lines");

        // Draw borders around the chart.
        chart
            .draw_series(LineSeries::new(
                vec![
                    (x_range.start, y_range.start),
                    (x_range.end, y_range.start),
                    (x_range.end, y_range.end),
                    (x_range.start, y_range.end),
                    (x_range.start, y_range.start),
                ]
                .into_iter(),
                self.border_color.filled(),
            ))
            .expect("Failed to plot lines");

        // Draw the labels on the chart.
        chart
            .configure_series_labels()
            .background_style(self.legend_background_color.filled())
            .border_style(colors::full_palette::BLACK)
            .position(SeriesLabelPosition::UpperLeft)
            .label_font(("sans-serif", self.legend_font_size, &self.label_text_style))
            .margin(self.legend_margin)
            .draw()
            .expect("Failed to draw labels");
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Boundaries {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

/// In a cartesian coordinate system,
/// takes in a current coordinate (x,y) and the bounds of the chart, and returns
/// booleans for each boundary it is near, within some [`epsilon`] parameter.
/// The order is clockwise from top: [top, right, bottom, left].
/// x - the x coordinate of the point
/// y - the y coordinate of the point
/// x_range - the x range of the chart
/// y_range - the y range of the chart
/// epsilon_percentage - the distance from the boundary to be considered "near"
fn near_boundaries(
    x: f32,
    y: f32,
    x_range: (f32, f32),
    y_range: (f32, f32),
    epsilon_percentage: f32,
) -> Boundaries {
    let x_epsilon = (x_range.1 - x_range.0) * epsilon_percentage;
    let y_epsilon = (y_range.1 - y_range.0) * epsilon_percentage;

    let top = y >= y_range.1 - y_epsilon;
    let right = x >= x_range.1 - x_epsilon;
    let bottom = y <= y_range.0 + y_epsilon;
    let left = x <= x_range.0 + x_epsilon;

    Boundaries {
        top,
        right,
        bottom,
        left,
    }
}

fn basic_log_normal_curve() -> Vec<(f32, f32)> {
    let x_min = 0.0;
    let x_max = 1.0;
    let liquidity = 1.0;
    let strike = 1.0;
    let sigma = 1.0;
    let time_to_expiry = 1.0;

    let mut points = vec![];

    let mut x = x_min;
    while x < x_max {
        let y = compute_y_given_x_rust(x, liquidity, strike, sigma, time_to_expiry);
        points.push((x, y));
        x += 0.01;
    }

    points.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
}

/// K e^(-1/2 v^2 t) (v sqrt(t) e^(ln(x / K) + 1/2 v^2 t) / Gaussian.pdf(ln(x/K)
/// / v sqrt(t) + 1/2 v sqrt(t)))
fn g_x(x: f64, strike: f64, sigma: f64, time_to_expiry: f64) -> f64 {
    // If x is within float epilson of 0, return 0.
    if x < f64::EPSILON && x > -f64::EPSILON {
        return 0.0;
    }

    let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
    // v sqrt(t)
    let v_sqrt_t = compute_sigma_sqrt_tau(sigma, time_to_expiry);
    // ln(x / K)
    let ln_x_div_k = compute_ln_s_div_k(x, strike);
    // 1/2 v sqrt(t)
    let half_sigma_pow_two_tau = compute_half_sigma_power_2_tau(sigma, time_to_expiry);
    // K e^(-1/2 v^2 t)
    let term_1 = strike * (-half_sigma_pow_two_tau).exp();
    // v sqrt(t) e^(ln(x / K) + 1/2 v^2 t)
    let numerator = v_sqrt_t * (ln_x_div_k + half_sigma_pow_two_tau).exp();
    // Gaussian.pdf(ln(x/K) / v sqrt(t) + 1/2 v sqrt(t))
    let denominator = normal.pdf(ln_x_div_k / v_sqrt_t + half_sigma_pow_two_tau);

    term_1 * numerator / denominator
}

/// a = L / integral(0, 1000)( 1 / g(x) gx )
fn get_a(liquidity: f64, strike: f64, sigma: f64, time_to_expiry: f64) -> f64 {
    let g_x = reikna::func!(move |x: f64| 1.0 / g_x(x, strike, sigma, time_to_expiry));
    let integral = reikna::integral::integrate(&g_x, f64::EPSILON, 1000.0);
    liquidity / integral
}

/// a / g(x)
fn liq_distribution(x: f64, liquidity: f64, strike: f64, sigma: f64, time_to_expiry: f64) -> f64 {
    let a = get_a(liquidity, strike, sigma, time_to_expiry);
    a / g_x(x, strike, sigma, time_to_expiry)
}

/// Plot the points of the basic normal liq distribution.
fn basic_liq_dist_curve() -> Vec<(f32, f32)> {
    let x_min = 0.01;
    let x_max = 1.0;
    let liquidity = 1.0;
    let strike = 1.0;
    let sigma = 1.0;
    let time_to_expiry = 1.0;

    let mut points = vec![];

    let mut x = x_min;
    while x < x_max {
        let y = liq_distribution(x, liquidity, strike, sigma, time_to_expiry);
        points.push((x, y));
        x += 0.01;
    }

    points.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
}

impl From<ChartLine> for ChartLineSeries {
    fn from(line: ChartLine) -> Self {
        Self {
            lines: vec![line],
            ..Default::default()
        }
    }
}

impl From<Vec<ChartLine>> for ChartLineSeries {
    fn from(lines: Vec<ChartLine>) -> Self {
        Self {
            lines,
            ..Default::default()
        }
    }
}

impl From<((f32, f32), (f32, f32))> for ChartLine {
    fn from(line: ((f32, f32), (f32, f32))) -> Self {
        Self {
            x1: line.0 .0,
            y1: line.0 .1,
            x2: line.1 .0,
            y2: line.1 .1,
        }
    }
}

impl From<((f32, f32), (f32, f32))> for ChartLineSeries {
    fn from(line: ((f32, f32), (f32, f32))) -> Self {
        Self {
            lines: vec![line.into()],
            ..Default::default()
        }
    }
}

impl From<(f32, f32)> for ChartPoint {
    fn from(point: (f32, f32)) -> Self {
        Self {
            x: point.0,
            y: point.1,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use statrs::{
        assert_almost_eq,
        distribution::{Continuous, ContinuousCDF, Normal},
    };

    use super::*;

    #[test]
    fn test_basic_log_normal_curve() {
        let points = basic_log_normal_curve();
        for (x, y) in &points {
            println!("({}, {})", x, y);
        }
        assert_eq!(points.len(), 100);
    }

    #[test]
    fn test_liquidity_distribution_plot() {
        let points = basic_liq_dist_curve();
        for (x, y) in &points {
            println!("({}, {})", x, y);
        }
        assert_eq!(points.len(), 100);
    }

    #[test]
    fn test_derivative_liq_distribution() {
        // Loop over from x min to x max,
        // and compute the derivative of the liq distribution.
        // make sure the derivative makes sense!

        let x_min = 0.00001;
        let x_max = 1.0;
        let liquidity = 1.0;
        let strike = 1.0;
        let sigma = 2.0;
        let time_to_expiry = 1.0;

        let mut points = vec![];

        let mut x = x_min;
        while x < x_max {
            let y = liq_distribution(x, liquidity, strike, sigma, time_to_expiry);
            points.push((x, y));
            x += 0.01;
        }

        let mut points: Vec<(f32, f32)> =
            points.iter().map(|(x, y)| (*x as f32, *y as f32)).collect();

        let mut prev = points.remove(0);

        for (x, y) in points {
            let dy = y - prev.1;
            let dx = x - prev.0;
            let slope = dy / dx;
            println!("({}, {})", x, slope);
            prev = (x, y);
        }
    }

    #[test]
    fn test_liquidity_distribution() {
        let x = 0.05;
        let liquidity = 1.0;
        let strike = 1.0;
        let sigma = 1.0;
        let time_to_expiry = 1.0;

        let depth = liq_distribution(x, liquidity, strike, sigma, time_to_expiry);
        println!("depth: {}", depth);
    }

    #[test]
    fn test_g_x() {
        let x = 0.5;
        let liquidity = 1.0;
        let strike = 1.0;
        let sigma = 1.0;
        let time_to_expiry = 1.0;

        let depth = g_x(x, strike, sigma, time_to_expiry);
        println!("depth: {}", depth);
    }

    #[test]
    fn test_get_a() {
        let x = 0.5;
        let liquidity = 1.0;
        let strike = 1.0;
        let sigma = 1.0;
        let time_to_expiry = 1.0;

        let depth = get_a(liquidity, strike, sigma, time_to_expiry);
        println!("depth: {}", depth);

        assert_almost_eq!(depth, 1.0, 1e-2);
    }
}
