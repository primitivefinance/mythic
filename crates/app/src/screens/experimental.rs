//! Implement an experimental screen.

use iced::{
    widget,
    widget::{Column, Container},
    Command, Element, Length,
};
use plotters::{coord::Shift, prelude::ChartBuilder, series::LineSeries, style::RED};
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, DrawingArea, Renderer};

use super::*;

pub struct ExperimentalScreen {
    chart: MyChart,
}

impl ExperimentalScreen {
    pub fn new() -> Self {
        Self {
            chart: MyChart::new(),
        }
    }
}

impl From<ExperimentalScreen> for Screen {
    fn from(screen: ExperimentalScreen) -> Self {
        Screen::new(Box::new(screen))
    }
}

impl State for ExperimentalScreen {
    fn load(&self) -> Command<Message> {
        Command::none()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            _ => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let chart = self.chart.view().map(move |_x| view::Message::Experimental);
        let content = Column::new()
            .padding(Sizes::Lg as u16)
            .push(h1("experimental".to_string()))
            .push(chart);

        view::app_layout(
            &view::Page::Experimental,
            Container::new(content)
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .into()
    }
}

/// Chart stuff
#[allow(unused)]
struct MyChart;

impl MyChart {
    pub fn new() -> Self {
        Self
    }

    fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for MyChart {
    type State = ();
    // leave it empty
    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, _builder: ChartBuilder<DB>) {}

    fn draw_chart<DB: DrawingBackend>(&self, _state: &Self::State, root: DrawingArea<DB, Shift>) {
        let children = root.split_evenly((2, 2));
        for (i, area) in children.iter().enumerate() {
            let builder = ChartBuilder::on(area);
            draw_chart(builder, i + 1);
        }
    }
}

fn draw_chart<DB: DrawingBackend>(mut chart: ChartBuilder<DB>, power: usize) {
    let mut chart = chart
        .margin(30)
        .caption(format!("y=x^{}", power), ("sans-serif", 22))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(3)
        .y_labels(3)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-50..=50)
                .map(|x| x as f32 / 50.0)
                .map(|x| (x, x.powf(power as f32))),
            &RED,
        ))
        .unwrap();
}
