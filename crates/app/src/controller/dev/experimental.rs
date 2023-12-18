//! Implement an experimental screen.

use iced::{
    widget::{Column, Container},
    Command, Element, Length,
};

use super::*;
use crate::components::{
    chart::CartesianChart,
    system::{label, ExcaliburButton, ExcaliburTable},
    tables::cells::CellBuilder,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Chart(chart::ChartMessage),
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
            Self::AppMessage::View(Self::ViewMessage::Experimental(message)) => {
                match message {
                    Message::Empty => {}
                    Message::Chart(message) => {
                        todo!("Handle chart message: {:?}", message)
                    }
                }
            }
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

        let _exp_table = ExcaliburTable::new()
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

        // .push(
        // button(label(&"Danger button").build())
        // .on_press(Self::ViewMessage::Experimental(Message::Empty))
        // .style(CustomButtonStyle::destructive(&ExcaliburTheme::theme()).as_custom()),
        // )
        // .push(
        // button(label(&"Secondary button").build())
        // .on_press(Self::ViewMessage::Experimental(Message::Empty))
        // .style(CustomButtonStyle::secondary(&ExcaliburTheme::theme()).as_custom()),
        // )
        // .push(
        // button(label(&"Positive button").build())
        // .on_press(Self::ViewMessage::Experimental(Message::Empty))
        // .style(CustomButtonStyle::positive(&ExcaliburTheme::theme()).as_custom()),
        // )
        // .push(
        // button(label(&"disabled button").build())
        // .style(CustomButtonStyle::primary(&ExcaliburTheme::theme()).as_custom()),
        // )

        let misc = Column::new()
            .spacing(Sizes::Sm)
            .push(
                ExcaliburButton::new()
                    .primary()
                    .build(label("New button").build())
                    .padding(Sizes::Md)
                    .on_press(Self::ViewMessage::Experimental(Message::Empty)),
            )
            .push(
                ExcaliburButton::new()
                    .primary()
                    .build(label("New button").build())
                    .padding(Sizes::Md),
            )
            .push(
                ExcaliburButton::new()
                    .danger()
                    .build(label("New button").build())
                    .padding(Sizes::Md)
                    .on_press(Self::ViewMessage::Experimental(Message::Empty)),
            )
            .push(
                ExcaliburButton::new()
                    .transparent()
                    .build(label("New button").build())
                    .padding(Sizes::Md)
                    .on_press(Self::ViewMessage::Experimental(Message::Empty)),
            );

        Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(content)
                        .push(Column::new().width(Length::FillPortion(2)))
                        .height(Length::FillPortion(3)),
                )
                .push(
                    Row::new()
                        .push(empty_table.build().width(Length::FillPortion(2)))
                        .push(misc.width(Length::FillPortion(1)))
                        .height(Length::FillPortion(2)),
                ),
        )
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
