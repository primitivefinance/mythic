pub mod example;

use std::{cell::RefCell, sync::RwLock};

use cfmm_math::trading_functions::rmm::compute_y_given_x_rust;
use iced::{
    event, executor,
    mouse::Cursor,
    widget::{
        canvas::{self, Cache, Frame, Geometry},
        Column, Container, Text,
    },
    Alignment, Application, Command, Element, Length, Point, Size, Theme,
};
use plotters::{
    coord::{types::RangedCoordf32, ReverseCoordTranslate},
    prelude::*,
    series,
};
use plotters_iced::{Chart, ChartWidget, Renderer};

use super::*;

#[derive(Default)]
pub struct CartesianChart {
    cache: Arc<RwLock<Cache>>,
    relative_position: Option<Point>,
    pub state: CartesianState,
    points: Vec<(f32, f32)>,
    lines: Vec<((f32, f32), (f32, f32))>,
    spec: Mutex<Option<Cartesian2d<RangedCoordf32, RangedCoordf32>>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    MouseEvent(iced::mouse::Event, iced::Point),
}

impl CartesianChart {
    pub type Message = Message;

    pub fn new() -> Self {
        let log_normal_plot = basic_log_normal_curve();
        let lines = log_normal_plot
            .windows(2)
            .map(|window| ((window[0].0, window[0].1), (window[1].0, window[1].1)))
            .collect();

        Self {
            cache: Arc::new(RwLock::new(Cache::default())),
            relative_position: None,
            state: CartesianState::default(),
            points: vec![],
            lines,
            spec: Mutex::new(None),
        }
    }

    pub fn view(&self) -> Element<Self::Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

pub struct CartesianState {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
}

impl Default for CartesianState {
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
    pub relative_position: Option<Point>,
    pub cartesian_position: Option<Point>,
}

impl Chart<Message> for CartesianChart {
    type State = ChartState;

    // this caches the plot which is very efficient, but not interactive!
    // fn draw<R: Renderer, F: Fn(&mut Frame)>(
    // &self,
    // renderer: &R,
    // bounds: Size,
    // draw_fn: F,
    // ) -> Geometry {
    // renderer.draw_cache(&self.cache.clone().read().unwrap(), bounds, draw_fn)
    // }

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

        (event::Status::Ignored, None)
    }

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        use plotters::style::colors;

        const POINT_COLOR: RGBColor = colors::MAGENTA;
        const LINE_COLOR: RGBColor = colors::full_palette::TEAL_A400;
        const HOVER_COLOR: RGBColor = colors::full_palette::AMBER_500;
        const MESH_COLOR: RGBAColor = RGBAColor(0x1c, 0x1c, 0x1c, 0.8);

        // Create the initial chart with the builder.
        let mut chart = builder
            .build_cartesian_2d(
                self.state.x_range.0..self.state.x_range.1,
                self.state.y_range.0..self.state.y_range.1,
            )
            .expect("Failed to build chart");

        // Draw the background mesh grid and axis labels.
        chart
            .configure_mesh()
            .label_style(&colors::WHITE)
            .axis_style(colors::WHITE)
            .light_line_style(MESH_COLOR)
            .x_labels(3)
            .y_labels(3)
            .max_light_lines(3)
            .draw()
            .expect("Failed to draw chart mesh");

        // Draw the points on the chart.
        chart
            .draw_series(
                self.points
                    .iter()
                    .map(|p| Circle::new(*p, 5_i32, POINT_COLOR.filled())),
            )
            .expect("Failed to plot points");

        // Draw the line plots on the chart.
        for line in &self.lines {
            chart
                .draw_series(LineSeries::new(
                    vec![line.0, line.1].into_iter(),
                    LINE_COLOR.filled().stroke_width(2),
                ))
                .expect("Failed to plot lines");
        }

        // Draw the preview line.
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
            let dot_and_label = |x: f32, y: f32, x_label: bool| {
                let label = match x_label {
                    true => format!("{:.2}", x),
                    false => format!("{:.2}", y),
                };

                let near_left = x - bound_x * 0.025 <= bound_x_start;
                let near_right = x >= bound_x - bound_x * 0.025;

                // Reversed because we are on cartesian grid!
                let near_top = y >= bound_y_end - bound_y_end * 0.025;
                let near_bottom = y <= bound_y + bound_y_end * 0.025;

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
                    ShapeStyle::from(&colors::full_palette::GREY_800).filled(),
                );

                return EmptyElement::at((x, y))
                    + rect
                    + plotters::prelude::Text::new(
                        label,
                        label_offset,
                        ("sans-serif", 15.0).into_font().color(&colors::WHITE),
                    );
            };

            // Draw a line through the y cursor position.
            chart
                .draw_series(LineSeries::new(
                    vec![(bound_x_start, translated.1), (bound_x, translated.1)].into_iter(),
                    colors::full_palette::GREY_A400.filled(),
                ))
                .expect("Failed to plot lines");

            // Draw a line through the x cursor position.
            chart
                .draw_series(LineSeries::new(
                    vec![(translated.0, bound_y), (translated.0, bound_y_end)].into_iter(),
                    colors::full_palette::GREY_A400.filled(),
                ))
                .expect("Failed to plot lines");

            // Draw the y axis label.
            chart
                .draw_series(PointSeries::of_element(
                    vec![y_label_coords],
                    5,
                    &colors::WHITE,
                    &|c, s, st| {
                        return dot_and_label(c.0, c.1, false);
                    },
                ))
                .expect("Failed to plot points");

            // Draw the x axis label.
            chart
                .draw_series(PointSeries::of_element(
                    vec![x_label_coords],
                    5,
                    &colors::WHITE,
                    &|c, s, st| {
                        return dot_and_label(c.0, c.1, true);
                    },
                ))
                .expect("Failed to plot points");
        }

        let coord_trans = chart.as_coord_spec();
        let x_range = coord_trans.get_x_range();
        let y_range = coord_trans.get_y_range();

        // Draw a white line on the y=0 and x=0
        chart
            .draw_series(LineSeries::new(
                vec![(x_range.start, 0.0), (x_range.end, 0.0)].into_iter(),
                colors::full_palette::GREY_600.filled(),
            ))
            .expect("Failed to plot lines");

        chart
            .draw_series(LineSeries::new(
                vec![(0.0, y_range.start), (0.0, y_range.end)].into_iter(),
                colors::full_palette::GREY_600.filled(),
            ))
            .expect("Failed to plot lines");

        // Finally, draw borders around the chart.
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
                colors::full_palette::GREY_800.filled(),
            ))
            .expect("Failed to plot lines");

        // Draw the labels on the chart.
        // chart
        // .configure_series_labels()
        // .border_style(colors::RED)
        // .margin(10)
        // .label_font(("sans-serif", 20, &colors::BLACK))
        // .position(SeriesLabelPosition::Coordinate(
        // state.relative_position.unwrap_or_default().x as i32,
        // state.relative_position.unwrap_or_default().y as i32,
        // ))
        // .draw()
        // .expect("Failed to draw labels");

        // This line is saving the current state of the chart (like its range and scale)
        // so it can be reused or referenced later without having to be recalculated.
        *self.spec.lock().unwrap() = Some(chart.as_coord_spec().clone());
    }
}

fn basic_log_normal_curve() -> Vec<(f32, f32)> {
    let x_min = 0.0;
    let x_max = 1.0;

    let mut points = vec![];

    let mut x = x_min;
    while x < x_max {
        let y = compute_y_given_x_rust(x, 1.0, 1.0, 2.0, 1.0);
        points.push((x, y));
        x += 0.005;
    }

    points.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_log_normal_curve() {
        let points = basic_log_normal_curve();
        for (x, y) in &points {
            println!("({}, {})", x, y);
        }
        assert_eq!(points.len(), 100);
    }
}
