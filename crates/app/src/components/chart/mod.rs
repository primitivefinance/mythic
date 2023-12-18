//! Interactive chart built on top of iced-plotters and plotters.
//! todo: plot functions instead of the results of the functions!
//! To accomplish that we need to update the plot as the x and y ranges change
//! with the user's scrolling or dragging.

use cfmm_math::trading_functions::rmm::{compute_y_given_x_rust, liq_distribution};
use iced::{
    event,
    mouse::{Cursor, Event},
    widget::canvas::{self},
    Element, Length, Point,
};
use plotters::{coord::ReverseCoordTranslate, prelude::*, style::colors};
use plotters_iced::{Chart, ChartWidget};

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
///
/// - lines - Each line to plot.
/// - color - The color of the line series.
/// - width - The stroke width of the line series.
/// - legend - The legend of the line series.
/// - time_series - Whether the x-axis is a time unit or not.
#[derive(Debug, Clone)]
pub struct ChartLineSeries {
    pub lines: Vec<ChartLine>,
    pub color: RGBColor,
    pub width: u32,
    pub legend: String,
    pub time_series: bool,
}

impl Default for ChartLineSeries {
    fn default() -> Self {
        Self {
            lines: vec![((0.0, 0.0), (1.0, 1.0)).into()],
            color: colors::full_palette::TEAL_A400,
            width: 2,
            legend: "Series".to_string(),
            time_series: false,
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
    /// Flag for overriding the original ranges.
    pub override_ranges: bool,
}

/// Makes a default chart with a point of interest and a series plot.
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
            override_ranges: false,
        }
    }
}

/// Converts a vector of coordinates into a line series plot that draws line
/// segments between each coordinate.
pub fn coords_to_line_series(coords: Vec<(f32, f32)>) -> ChartLineSeries {
    let line: Vec<((f32, f32), (f32, f32))> = coords
        .windows(2)
        .map(|window| ((window[0].0, window[0].1), (window[1].0, window[1].1)))
        .collect();

    let line: Vec<ChartLine> = line.into_iter().map(|line| line.into()).collect();
    let series: ChartLineSeries = line.into();

    series
}

#[derive(Debug, Clone)]
pub enum Message {
    MouseEvent(iced::mouse::Event, iced::Point),
}

impl CartesianChart {
    pub type Message = Message;

    /// Makes a completely fresh chart with no points or series.
    /// Default has a point and a line series.
    pub fn new() -> Self {
        Self {
            series: vec![],
            points: vec![],
            ..Default::default()
        }
    }

    pub fn series(&mut self, new_series: ChartLineSeries) {
        self.series.push(new_series);
    }

    // todo: improve this
    pub fn override_series(&mut self, new_series: Vec<ChartLineSeries>) {
        self.series = new_series;
    }

    // todo: improve this
    pub fn override_points(&mut self, new_points: Vec<ChartPoint>) {
        self.points = new_points;
    }

    pub fn extend_many_series(&mut self, new_series: Vec<ChartLineSeries>) {
        self.series.extend(new_series);
    }

    pub fn point_of_interest(&mut self, new_point: ChartPoint) {
        self.points.push(new_point);
    }

    pub fn points_of_interest(&mut self, new_points: Vec<ChartPoint>) {
        self.points.extend(new_points);
    }

    pub fn view(&self) -> Element<Self::Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

/// Holds the x and y axis ranges for the chart.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// The state of the chart, which is interactively updated by the user in the
/// [`update`] method.
#[derive(Debug, Clone, Default)]
pub struct ChartState {
    /// The bounds of the chart, measured in absolute units.
    pub bounds: Option<iced::Rectangle>,
    /// The x-axis and y-axis ranges of the chart.
    pub range: CartesianRanges,
    /// The original x-axis and y-axis ranges of the chart before the user
    /// changes it with zoom or drag.
    pub original_range: Option<CartesianRanges>,
    /// The absolute position of the cursor inside the chart canvas bounds.
    pub relative_position: Option<Point>,
    /// The absolute position of the cursor with the y-axis flipped.
    pub cartesian_position: Option<Point>,
    /// Whether the left click is down.
    pub cursor_left_click_down: bool,
    /// The final position the left click released at.
    pub final_left_click_position: Option<Point>,
    /// The initial position of the left click.
    pub initial_left_click_position: Option<Point>,
    /// If the original coords have been overwritten. This can only occur once
    /// right now.
    pub permanent_override: bool,
    /// If the chart can be interacted with.
    pub can_interact: bool,
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
            if !_state.can_interact {
                return iced::mouse::Interaction::default();
            }

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
        // Occurs once when the override flag is set.
        // This is because we only have mutable state, but reference to self.
        // Which makes it awkward to update the ranges used by the graph from outside
        // the graph's state context.
        if self.override_ranges && !state.permanent_override {
            state.range = self.range.clone();
            state.original_range = Some(self.range.clone());
            state.permanent_override = true;
        }

        if let Cursor::Available(point) = cursor {
            match event {
                canvas::Event::Mouse(evt) if !bounds.contains(point) => {
                    state.can_interact = false;
                    if let (Event::WheelScrolled { .. }, false) = (evt, state.can_interact) {
                        // Ignore WheelScrolled event
                        return (event::Status::Ignored, None);
                    }
                }
                canvas::Event::Mouse(evt) if bounds.contains(point) => {
                    if let (Event::WheelScrolled { .. }, false) = (evt, state.can_interact) {
                        // Ignore WheelScrolled event
                        return (event::Status::Ignored, None);
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

                    if state.can_interact {
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
                                state.final_left_click_position = Some(p);
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
                    }

                    if let Event::ButtonPressed(iced::mouse::Button::Right) = evt {
                        // Flip the can_interact flag.
                        state.can_interact = !state.can_interact;

                        return (event::Status::Captured, None);
                    }

                    return (
                        event::Status::Captured,
                        Some(Message::MouseEvent(evt, Point::new(p.x, p.y))),
                    );
                }
                _ => {}
            }
        } else {
            // Reset interaction state.
            state.can_interact = false;

            // Reset the relative position to None if the cursor is not available.
            state.relative_position = None;
            state.cartesian_position = None;

            // Reset the left click down state if the cursor is not available.
            if state.cursor_left_click_down {
                state.cursor_left_click_down = false;
                state.initial_left_click_position = None;
            }
        }

        if state.bounds.is_none() {
            state.bounds = Some(bounds);
        }

        if state.original_range.is_none() {
            state.original_range = Some(self.range.clone());
        }

        // Check if the chart is zoomed at all by comparing the ranges to the original
        // ranges.
        let zoomed = state.range != state.original_range.clone().unwrap();

        // If the chart is not being moved with left click, and the series
        // has gone out of range, readjust the range to fit the series with some buffer.
        // Only automatically adjust if the user has not set its position.
        if !state.cursor_left_click_down
            && state.final_left_click_position.is_none()
            && !zoomed
            && !self.series.is_empty()
        {
            let first_series = &self.series[0];
            let x_values: Vec<f32> = first_series
                .lines
                .iter()
                .map(|l| vec![l.x1, l.x2])
                .flatten()
                .collect();
            let y_values: Vec<f32> = first_series
                .lines
                .iter()
                .map(|l| vec![l.y1, l.y2])
                .flatten()
                .collect();

            let max_x = x_values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let max_y = y_values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

            if max_x > state.range.x_range.1 || max_y > state.range.y_range.1 {
                let x_range = (
                    state.range.x_range.0,
                    max_x + (max_x - state.range.x_range.0) * 0.1,
                );
                let y_range = (
                    state.range.y_range.0,
                    max_y + (max_y - state.range.y_range.0) * 0.1,
                );

                state.range = CartesianRanges { x_range, y_range };
            }
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

        // Draw the points on the chart, if any.
        if !self.points.is_empty() {
            chart
                .draw_series(
                    self.points
                        .iter()
                        .map(|p| Circle::new((p.x, p.y), p.diameter, p.color.filled())),
                )
                .expect("Failed to plot points");
        }

        // Draw the line series' on the chart, if any.
        if !self.series.is_empty() {
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

                // If a series is a line series, draw a point for its latest
                // value and also a static label + y-axis grid
                // line.
                if line_series.time_series {
                    if let Some(last_line) = line_series.lines.last() {
                        let last_point = (last_line.x2, last_line.y2);

                        // Draw a point for the latest value.
                        chart
                            .draw_series(PointSeries::of_element(
                                vec![last_point],
                                5,
                                ShapeStyle::from(line_series.color).filled(),
                                &|c, _s, st| {
                                    return EmptyElement::at(c)
                                        + Circle::new((0, 0), 5, st.filled())
                                        + Circle::new(
                                            (0, 0),
                                            2,
                                            ShapeStyle::from(&self.axis_mesh_style).filled(),
                                        );
                                },
                            ))
                            .expect("Failed to plot points");

                        // Draw a light grid line through the y-axis value of the point.
                        chart
                            .draw_series(LineSeries::new(
                                vec![(last_point.0, 0.0), (last_point.0, last_point.1)].into_iter(),
                                ShapeStyle::from(&self.axis_mesh_style).filled(),
                            ))
                            .expect("Failed to plot lines")
                            .label(format!("{:.2}", last_point.1));

                        // Draw a static label for the latest y value.

                        let cursor_pos_cartesian = if let Some(relative) = state.relative_position {
                            let coord_trans = chart.as_coord_spec();
                            let translated = coord_trans
                                .reverse_translate((relative.x as i32, relative.y as i32))
                                .unwrap_or_default();

                            Some((translated.0, translated.1))
                        } else {
                            None
                        };

                        if let Some(cursor_pos_cartesian) = cursor_pos_cartesian {
                            let margin_y = state.range.y_range.1 - state.range.y_range.0;
                            let margin_y = margin_y * 0.05;

                            let margin_x = 1.0;

                            if cursor_pos_cartesian.0 >= last_point.0 - margin_x
                                && cursor_pos_cartesian.0 <= last_point.0 + margin_x
                                && cursor_pos_cartesian.1 >= last_point.1 - margin_y
                                && cursor_pos_cartesian.1 <= last_point.1 + margin_y
                            {
                                chart
                                    .draw_series(PointSeries::of_element(
                                        vec![last_point],
                                        5,
                                        ShapeStyle::from(&self.y_label_container_style).filled(),
                                        &|c, _s, st| {
                                            let label = format!("{:.2}", c.1);
                                            let label_container_coords = [(0, -35), (85, 0)];
                                            let label_offset = (
                                                label_container_coords[0].0 + 10,
                                                label_container_coords[0].1 + 10,
                                            );
                                            let rect = Rectangle::new(label_container_coords, st);

                                            // If the cursor is near the point, render it, else
                                            // render
                                            // nothing.

                                            return EmptyElement::at(c)
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
                    }
                }
            }
        }

        let coord_trans = chart.as_coord_spec();
        let x_range = coord_trans.get_x_range();
        let y_range = coord_trans.get_y_range();

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

        // If can_interact is false, display some text to right click to unlock.

        let center_coord = chart.as_coord_spec();
        let coord_y = center_coord.y_spec();
        let coord_x = center_coord.x_spec();
        let upper_right = (coord_x.range().end, coord_y.range().end);

        let user_guide = match state.can_interact {
            true => "Right Click to Lock",
            false => "Right Click to Interact",
        };

        chart
            .draw_series(PointSeries::of_element(
                vec![upper_right],
                0,
                ShapeStyle::from(self.label_text_style).filled(),
                &|c: (f32, f32), _s, st| {
                    return EmptyElement::at((c.0, c.1))
                        + Rectangle::new(
                            [(0, 0), (100, 100)],
                            ShapeStyle::from(colors::TRANSPARENT),
                        )
                        + plotters::prelude::Text::new(
                            user_guide,
                            (-200, 30),
                            ("sans-serif", 16.0).into_font().color(&self.border_color),
                        );
                },
            ))
            .expect("Failed to draw empty chart");

        // If there is no chart plots or series, return an empty chart with a "No data"
        // text element.
        if self.points.is_empty() && self.series.is_empty() {
            chart
                .draw_series(PointSeries::of_element(
                    vec![(0.5, 0.5)],
                    0,
                    ShapeStyle::from(self.label_text_style).filled(),
                    &|c: (f32, f32), _s, st| {
                        return EmptyElement::at((c.0, c.1))
                            + Rectangle::new(
                                [(0, 0), (100, 100)],
                                ShapeStyle::from(colors::TRANSPARENT),
                            )
                            + plotters::prelude::Text::new(
                                "No data",
                                (-30, -10),
                                ("sans-serif", self.label_font_size)
                                    .into_font()
                                    .color(&self.border_color),
                            );
                    },
                ))
                .expect("Failed to draw empty chart");

            return;
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

                // get the max distance of the chart, which is x-range^2 + y-range^2
                let max_distance = (state.range.x_range.1 - state.range.x_range.0).powi(2)
                    + (state.range.y_range.1 - state.range.y_range.0).powi(2);

                // Change hover over poi distance.
                let margin = max_distance * 0.005;

                if distance <= margin {
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

pub fn basic_log_normal_curve() -> Vec<(f32, f32)> {
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

/// Plot the points of the basic normal liq distribution.
pub fn basic_liq_dist_curve() -> Vec<(f32, f32)> {
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
    use statrs::assert_almost_eq;

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

        assert_almost_eq!(depth, 0.354, 1e-3);
    }
}
