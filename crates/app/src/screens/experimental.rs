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
use crate::components::chart::MyChart;

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

#[tracing::instrument(ret)]
async fn loading() -> anyhow::Result<(), Arc<anyhow::Error>> {
    Ok(())
}

impl State for ExperimentalScreen {
    type AppMessage = app::Message;
    type ViewMessage = view::Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::perform(loading(), |_| Self::AppMessage::Empty)
    }

    fn update(&mut self, _message: Self::AppMessage) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let chart = self.chart.view().map(move |_x| view::Message::Empty);
        let content = Column::new()
            .padding(Sizes::Lg as u16)
            .push(h1("experimental".to_string()))
            .push(chart);

        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
