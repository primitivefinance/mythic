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
use crate::components::chart::{example::MyChart, CartesianChart};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Chart(chart::Message),
}

impl From<Message> for view::Message {
    fn from(message: Message) -> Self {
        Self::Experimental(message)
    }
}

pub struct ExperimentalScreen {
    pub show_example: bool,
    example_chart: MyChart,
    line_chart: CartesianChart,
}

impl ExperimentalScreen {
    pub fn new() -> Self {
        Self {
            show_example: false,
            example_chart: MyChart::new(),
            line_chart: CartesianChart::new(),
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

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::Empty => {}
            Self::AppMessage::View(message) => match message {
                Self::ViewMessage::Experimental(message) => match message {
                    Message::Empty => {}
                    Message::Chart(message) => match message {
                        chart::Message::MouseEvent(event, point) => {
                            // tracing::info!("Mouse event: {:?} {:?}", event,
                            // point);
                        }
                    },
                },
                _ => {}
            },
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let chart = match self.show_example {
            true => self
                .example_chart
                .view()
                .map(move |_: view::Message| view::Message::Empty),
            false => self
                .line_chart
                .view()
                .map(move |x| Message::Chart(x).into()),
        };

        let content = Column::new()
            .padding(Sizes::Md as u16)
            .spacing(Sizes::Lg as u16)
            .push(highlight_label("Good Morning, Alex.".to_string()).size(FontSizes::Md))
            .push(highlight_label("It looks like your portfolio did well last night, maintaining 99.00% replication health.".to_string()).size(FontSizes::Md))
            .push(chart)
            .width(Length::FillPortion(3));

        Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(content)
                        .push(Column::new().width(Length::FillPortion(2)))
                        .height(Length::FillPortion(3)),
                )
                .push(Row::new().height(Length::FillPortion(2))),
        )
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
