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
use crate::components::{
    chart::CartesianChart,
    system::{label, ExcaliburTable},
    tables::cells::CellBuilder,
};

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
    line_chart: CartesianChart,
}

impl ExperimentalScreen {
    pub fn new() -> Self {
        Self {
            show_example: false,
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
        let chart = self
            .line_chart
            .view()
            .map(move |x| Message::Chart(x).into());

        let content = Column::new()
            .padding(Sizes::Md as u16)
            .spacing(Sizes::Lg as u16)
            .push(label("Good Morning, Alex.").highlight().title2().build())
            .push(label("It looks like your portfolio did well last night, maintaining 99.00% replication health.").highlight().title2().build())
            .push(chart)
            .width(Length::FillPortion(3));

        let cell_data: Vec<Vec<CellBuilder<Self::ViewMessage>>> = vec![
            vec![
                CellBuilder::new().value(Some("BTC".to_string())),
                CellBuilder::new().value(Some("$1,000,000.00".to_string())),
                CellBuilder::new().value(Some("0.00000000".to_string())),
                CellBuilder::new().value(Some("0.00%".to_string())),
            ],
            vec![
                CellBuilder::new().value(Some("ETH".to_string())),
                CellBuilder::new().value(Some("$1,000,000.00".to_string())),
                CellBuilder::new().value(Some("0.00000000".to_string())),
                CellBuilder::new().value(Some("0.00%".to_string())),
            ],
            vec![
                CellBuilder::new().value(Some("USDC".to_string())),
                CellBuilder::new().value(Some("$1,000,000.00".to_string())),
                CellBuilder::new().value(Some("0.00000000".to_string())),
                CellBuilder::new().value(Some("0.00%".to_string())),
            ],
        ];

        let exp_table = ExcaliburTable::new()
            .header("Asset")
            .header("Price")
            .header("Balance")
            .header("Weight")
            .build_custom(cell_data);

        let empty_table = ExcaliburTable::new()
            .header("Asset")
            .header("Price")
            .header("Balance")
            .header("Weight")
            .build_empty();

        Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(content)
                        .push(Column::new().width(Length::FillPortion(2)))
                        .height(Length::FillPortion(3)),
                )
                .push(Row::new().push(empty_table).height(Length::FillPortion(2))),
        )
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
