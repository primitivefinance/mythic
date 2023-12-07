//! Chart component for line plots.
//! Implement an experimental screen.

use iced::{
    widget::{Column, Container},
    Command, Element, Length, Point,
};
use plotters::{
    coord::Shift,
    define_color,
    prelude::ChartBuilder,
    series::LineSeries,
    style::{IntoTextStyle, RGBColor, TextStyle, RED},
};
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, DrawingArea, Renderer};

use super::*;

/// Chart stuff
#[derive(Debug, Clone, Default)]
pub struct MyChart {
    pub render_label: bool,
    label_position: Option<Point>,
}

impl MyChart {
    pub fn new() -> Self {
        Self {
            render_label: false,
            label_position: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for MyChart {
    type State = MyChart;
    // leave it empty
    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, _builder: ChartBuilder<DB>) {}

    fn draw_chart<DB: DrawingBackend>(&self, _state: &Self::State, root: DrawingArea<DB, Shift>) {
        let children = vec![root];
        for (i, area) in children.iter().enumerate() {
            let builder = ChartBuilder::on(area);
            draw_chart(builder, i + 2);
        }
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: iced::widget::canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (iced::event::Status, Option<Message>) {
        // Update the label position in the update method
        if cursor.is_over(bounds) {
            let chart_x = (cursor.position().unwrap().x - bounds.x) / bounds.width;
            let chart_y = (cursor.position().unwrap().y - bounds.y) / bounds.height;

            if (chart_y - chart_x.powf(2.0)).abs() < 0.01 {
                println!("Cursor is over the curve at ({}, {})", chart_x, chart_y);
                state.label_position = cursor.position();
                state.render_label = true;
            } else {
                state.render_label = false;
            }
        } else {
            state.render_label = false;
        }

        (iced::event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        // If within bounds of the chart return Crosshair
        if cursor.is_over(bounds) {
            let cursor_pos = cursor.position().unwrap();

            let x_label_height = 30.0;
            let left_label_width = 30.0;
            let top_label_height = 0.0;

            let (chart_x, chart_y) = get_chart_coordinates(
                cursor,
                bounds,
                left_label_width,
                x_label_height,
                top_label_height,
                0.0,
            );

            // Check if the cursor is within the Y AXIS LABEL AREA
            if cursor_pos.x - bounds.x < 30.0 {
                println!("Cursor is within the Y AXIS LABEL AREA");
            }
            // Check if the cursor is within the X AXIS LABEL AREA
            else if cursor_pos.y > bounds.y + bounds.height - 30.0 {
                println!("Cursor is within the X AXIS LABEL AREA");
            }
            // Check if the cursor is within the CHART AREA
            else if cursor_pos.x > bounds.x + left_label_width
                && cursor_pos.x < bounds.x + bounds.width
                && cursor_pos.y > bounds.y
                && cursor_pos.y < bounds.y + bounds.height - x_label_height
            {
                println!(
                    "Cursor is within the CHART AREA at ({}, {})",
                    chart_x, chart_y
                );

                // Check if the cursor is over the line y=x^2
                if (chart_y - chart_x.powf(2.0)).abs() < 0.01 {
                    println!(
                        "Cursor is over the line y=x^2 at ({}, {})",
                        chart_x, chart_y
                    );
                }
            }

            iced::mouse::Interaction::Crosshair
        } else {
            iced::mouse::Interaction::default()
        }
    }
}

// define_color!(MINT, 90, 255, 196, "Mint");

const WHITE_CHART_COLOR: RGBColor = RGBColor(255, 255, 255);

fn draw_chart<DB: DrawingBackend>(mut chart: ChartBuilder<DB>, power: usize) {
    let white = RGBColor(255, 255, 255);
    let mut chart = chart
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..1f32, 0f32..1.0f32)
        .unwrap();

    chart
        .configure_mesh()
        .label_style(&white)
        .axis_style(white)
        .x_labels(3)
        .y_labels(3)
        .light_line_style(&white)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..=5)
                .map(|x| x as f32 / 5.0)
                .map(|x| (x, x.powf(power as f32))),
            &RGBColor(90, 255, 196),
        ))
        .unwrap();
}

fn get_chart_coordinates(
    cursor: iced::mouse::Cursor,
    bounds: iced::Rectangle,
    left_label_width: f32,
    bottom_label_height: f32,
    top_label_height: f32,
    right_label_width: f32,
) -> (f32, f32) {
    let cursor_pos = cursor.position().unwrap();

    // Ranges affect scaling. For example, if the x range is 0..1 and the y range is
    // 0..2, then the chart will be twice as tall as it is wide. If the x range
    // is 0..2 and the y range is 0..1, then the chart will be twice as wide as
    // it is tall.
    let x_range = 1.0;
    let y_range = 1.0;

    let chart_bounds = get_chart_bounds(
        bounds,
        left_label_width,
        bottom_label_height,
        right_label_width,
        top_label_height,
        x_range,
        y_range,
    );
    let (chart_x, chart_y) =
        get_cursor_coordinates_in_chart(cursor, chart_bounds, x_range, y_range);

    (chart_x, chart_y)
}

fn get_chart_bounds(
    bounds: iced::Rectangle,
    left_offset: f32,
    bottom_offset: f32,
    right_offset: f32,
    top_offset: f32,
    x_scale: f32,
    y_scale: f32,
) -> iced::Rectangle {
    let chart_width = bounds.width - left_offset - right_offset;
    let chart_height = bounds.height - bottom_offset - top_offset;

    let chart_x = bounds.x + left_offset;
    let chart_y = bounds.y;

    iced::Rectangle {
        x: chart_x,
        y: chart_y,
        width: chart_width,
        height: chart_height,
    }
}

fn get_cursor_coordinates_in_chart(
    cursor: iced::mouse::Cursor,
    chart_bounds: iced::Rectangle,
    x_scale: f32,
    y_scale: f32,
) -> (f32, f32) {
    let cursor_pos = cursor.position().unwrap();

    // Calculate the x and y position of the cursor within the chart bounds
    let chart_x = (cursor_pos.x - chart_bounds.x) / chart_bounds.width;
    let chart_y = 1.0 - ((cursor_pos.y - chart_bounds.y) / chart_bounds.height);

    (chart_x, chart_y)
}
