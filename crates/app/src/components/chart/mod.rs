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
            x_range: (0.0, 1.0),
            y_range: (0.0, 1.0),
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
        const LINE_COLOR: RGBColor = colors::CYAN;
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
                    LINE_COLOR.filled(),
                ))
                .expect("Failed to plot lines");
        }

        // if let Some(relative_position) = state.relative_position {
        // tracing::info!("Drawing hover point {:?}", relative_position);
        // let distance_threshold = 0.25; // Define a threshold for "closeness"
        // let points = vec![(0.0, 0.0), (0.5, 0.5)];
        // for point in &points {
        // let distance = ((relative_position.x - point.0).powi(2)
        // + (relative_position.y - point.1).powi(2))
        // .sqrt();
        // if distance <= distance_threshold {
        // chart.draw_series(PointSeries::of_element(
        // vec![*point],
        // 5,
        // &colors::WHITE,
        // &|c, s, st| {
        // return EmptyElement::at(c)    // We want to construct a composed element
        // on-the-fly
        // + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate
        //   is established
        // + plotters::prelude::Text::new(format!("{:?}", c), (10, 0), ("sans-serif",
        //   10).into_font());
        // },
        // )).expect("Failed to plot points");
        // }
        // }
        // }

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
            let bound_x = coord_trans.get_x_range().end;
            let bound_y = coord_trans.get_y_range().start;

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

                let label_offset = match x_label {
                    true => (-10, -50),
                    false => (-50, -10),
                };

                return EmptyElement::at((x, y))
                    + Circle::new((0, 0), 3, ShapeStyle::from(&colors::WHITE).filled())
                    + plotters::prelude::Text::new(
                        label,
                        label_offset,
                        ("sans-serif", 15.0).into_font().color(&colors::WHITE),
                    );
            };

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

            //  chart.draw_series(PointSeries::of_element(
            // vec![relative_position],
            // 5,
            // &colors::WHITE,
            // &|c, s, st| {
            // return EmptyElement::at(c)    // We want to construct a composed
            // element on-the-fly
            // + Circle::new((0,0),s,st.filled()) // At this point, the new
            //   pixel coordinate is established
            // + plotters::prelude::Text::new(format!("{:?}", c), (10, 0),
            //   ("sans-serif", 10).into_font());
            // },
            // )).expect("Failed to plot points");
            // chart
            // .draw_series(std::iter::once(Circle::new(
            // relative_position,
            // 5_i32,
            // HOVER_COLOR.filled(),
            // )))
            // .expect("Failed to plot hover point")
            // .label(format!("Coords: {:?}", relative_position));
        }

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
        let y = compute_y_given_x_rust(x, 1.0, 1.0, 1.0, 1.0);
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
