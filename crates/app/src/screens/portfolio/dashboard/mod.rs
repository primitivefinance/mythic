//! Renders a view of the portfolio's positions and strategies.

pub mod form;
pub mod review;
pub mod simulate;

use std::collections::HashMap;

use profiles::portfolios::{Portfolio, Targetable};

use self::{form::DeltaForm, review::ReviewAdjustment, simulate::Simulate};
use super::*;
use crate::components::{
    containers::CustomContainer,
    tables::{builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, rows::RowBuilder},
};

#[tracing::instrument(skip(name), ret)]
async fn load_portfolio(name: Option<String>) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let path = match name {
        Some(name) => Some(Portfolio::file_path_with_name(name)),
        None => None,
    };
    let portfolio = Portfolio::load(path);
    let portfolio = match portfolio {
        Ok(portfolio) => portfolio,
        Err(e) => {
            tracing::error!("Failed to load portfolio: {:?}", e);
            return Err(Arc::new(e));
        }
    };

    Ok(portfolio)
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Load(anyhow::Result<Portfolio, Arc<anyhow::Error>>),
    ChangePrice(usize, Option<String>),
    ChangeBalance(usize, Option<String>),
    ChangeMarketValue(usize, Option<String>),
    ChangeWeight(usize, Option<String>),
    Submit,
    Delta(DeltaMessage),
    ReviewAdjustment,
    Review(review::Message),
    Simulated(simulate::Message),
}

impl MessageWrapperView for Message {
    type ParentMessage = view::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = developer::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Dash(message)
    }
}

impl From<Message> for <Message as MessageWrapperView>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Developer(developer::Message::Dash(message))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dashboard {
    portfolio: Option<Portfolio>,
    deltas: DeltaForm,
    summary: Option<DeltaSummary>,
    ready: bool,
    state: DashboardState,
    loaded_from: Option<String>,
}

impl From<DashboardWrapper> for Screen {
    fn from(dashboard: DashboardWrapper) -> Self {
        Screen::new(Box::new(dashboard))
    }
}

/// Wrapper around the dashboard so it can be used directly from the root app.
pub struct DashboardWrapper {
    pub dashboard: Dashboard,
}

impl DashboardWrapper {
    pub fn new(name: Option<String>) -> Self {
        Self {
            dashboard: Dashboard::new(name),
        }
    }
}

impl State for DashboardWrapper {
    type AppMessage = app::Message;
    type ViewMessage = view::Message;

    fn load(&self) -> Command<Self::AppMessage> {
        let cmd: Command<developer::Message> = self.dashboard.load().map(|x| x.into());
        return cmd.map(|x| x.into());
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            app::Message::View(view::Message::Developer(msg)) => match msg {
                developer::Message::Dash(message) => {
                    let cmd: Command<developer::Message> =
                        self.dashboard.update(message).map(|x| x.into());
                    return cmd.map(|x| x.into());
                }
                _ => {}
            },
            _ => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        self.dashboard.view().map(|x| x.into())
    }
}

#[derive(Debug, Clone, Default)]
pub enum DashboardState {
    #[default]
    Empty,
    Review(ReviewAdjustment),
    Simulate(Simulate),
}

impl DashboardState {
    pub fn clear(&mut self) {
        *self = DashboardState::Empty;
    }
}

impl Dashboard {
    pub type AppMessage = Message;
    pub type ViewMessage = view::Message;

    pub fn new(name: Option<String>) -> Self {
        // Try loading the portfolio from the name.

        Self {
            portfolio: None,
            deltas: DeltaForm::default(),
            summary: None,
            ready: false,
            state: DashboardState::default(),
            loaded_from: name,
        }
    }

    pub fn loaded(&self) -> bool {
        self.portfolio.is_some()
    }

    pub fn position_table(&self) -> TableBuilder<Self::AppMessage> {
        let positions = self.portfolio.clone().unwrap_or_default().positions;

        TableBuilder::new()
            .padding_cell(Sizes::Md)
            .padding_cell_internal(Sizes::Xs)
            .column(
                ColumnBuilder::new()
                    .headers(vec![
                        "Ticker".to_string(),
                        "Price".to_string(),
                        "Delta".to_string(),
                        "Balance".to_string(),
                        "Delta".to_string(),
                        "Market Value".to_string(),
                        "Delta".to_string(),
                        "Weight".to_string(),
                        "Delta".to_string(),
                    ])
                    .rows(
                        positions
                            .iter()
                            .enumerate()
                            .map(|(pos_index, position)| {
                                let target_cells = position
                                    .clone()
                                    .targets
                                    .unwrap_or_default()
                                    .into_iter()
                                    .filter(|target| matches!(target, Targetable::Weight(_)))
                                    .flat_map(|target| {
                                        vec![
                                            CellBuilder::new()
                                                .value(Some(target.clone().to_string())),
                                            CellBuilder::new()
                                                .value(self.deltas.weight.get(&pos_index).cloned())
                                                .on_change(move |x| {
                                                    tracing::trace!(
                                                        "Weight changed: {}",
                                                        x.clone().unwrap_or_default()
                                                    );
                                                    Message::ChangeWeight(pos_index, x)
                                                })
                                                .style(|| {
                                                    CustomContainer::theme(Some(
                                                        iced::Background::Color(GRAY_400),
                                                    ))
                                                }),
                                        ]
                                        .into_iter()
                                    })
                                    .collect::<Vec<CellBuilder<dashboard::Message>>>();

                                RowBuilder::new()
                                    .style(|| {
                                        CustomContainer::theme(Some(iced::Background::Color(
                                            GRAY_500,
                                        )))
                                    })
                                    .cells(
                                        vec![
                                            CellBuilder::new()
                                                .value(Some(position.asset.symbol.clone())),
                                            CellBuilder::new()
                                                .value(position.cost.map(|x| x.to_string())),
                                            CellBuilder::new()
                                                .value(self.deltas.price.get(&pos_index).cloned())
                                                .on_change(move |x| {
                                                    tracing::trace!(
                                                        "Price changed: {}",
                                                        x.clone().unwrap_or_default()
                                                    );
                                                    Message::ChangePrice(pos_index, x)
                                                })
                                                .style(|| {
                                                    CustomContainer::theme(Some(
                                                        iced::Background::Color(GRAY_400),
                                                    ))
                                                }),
                                            CellBuilder::new()
                                                .value(position.balance.map(|x| x.to_string())),
                                            CellBuilder::new()
                                                .value(self.deltas.balance.get(&pos_index).cloned())
                                                .on_change(move |x| {
                                                    tracing::trace!(
                                                        "Balance changed: {}",
                                                        x.clone().unwrap_or_default()
                                                    );
                                                    Message::ChangeBalance(pos_index, x)
                                                })
                                                .style(|| {
                                                    CustomContainer::theme(Some(
                                                        iced::Background::Color(GRAY_400),
                                                    ))
                                                }),
                                            CellBuilder::new()
                                                .value(position.cost.map(|x| x.to_string())),
                                            CellBuilder::new()
                                                .value(
                                                    self.deltas
                                                        .market_value
                                                        .get(&pos_index)
                                                        .cloned(),
                                                )
                                                .on_change(move |x| {
                                                    tracing::trace!(
                                                        "Market value changed: {}",
                                                        x.clone().unwrap_or_default()
                                                    );
                                                    Message::ChangeMarketValue(pos_index, x)
                                                })
                                                .style(|| {
                                                    CustomContainer::theme(Some(
                                                        iced::Background::Color(GRAY_400),
                                                    ))
                                                }),
                                        ]
                                        .into_iter()
                                        .chain(target_cells)
                                        .collect(),
                                    )
                            })
                            .collect(),
                    ),
            )
    }
}

impl State for Dashboard {
    type AppMessage = Message;
    type ViewMessage = view::Message;

    /// todo: how to handle different portfolio loads.
    fn load(&self) -> Command<Self::AppMessage> {
        let name = match self.loaded_from.clone() {
            Some(name) => Some(name),
            None => Some("Main".to_string()),
        };

        Command::perform(load_portfolio(name), |x| Message::Load(x).into())
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                self.portfolio = Some(portfolio);
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load portfolio: {:?}", e);
            }
            Message::ChangePrice(index, value) => {
                // If value is None, clear the delta in the hashmap, if it exists.
                // Else, insert it to the hashmap.
                if value.is_none() {
                    self.deltas.price.remove(&index);
                } else {
                    self.deltas.price.insert(index, value.unwrap_or_default());
                }
            }
            Message::ChangeBalance(index, value) => {
                if value.is_none() {
                    self.deltas.balance.remove(&index);
                } else {
                    self.deltas.balance.insert(index, value.unwrap_or_default());
                }
            }
            Message::ChangeMarketValue(index, value) => {
                if value.is_none() {
                    self.deltas.market_value.remove(&index);
                } else {
                    self.deltas
                        .market_value
                        .insert(index, value.unwrap_or_default());
                }
            }
            Message::ChangeWeight(index, value) => {
                if value.is_none() {
                    self.deltas.weight.remove(&index);
                } else {
                    self.deltas.weight.insert(index, value.unwrap_or_default());
                }
            }
            Message::Submit => {
                tracing::trace!("Reviewing...");

                self.state = DashboardState::Review(ReviewAdjustment::default());
            }
            Message::ReviewAdjustment => {
                let mut deltas: Vec<PositionDelta> = vec![];

                // Iterate through the delta hashmaps and fill up this deltas
                // vector.
                for (index, position) in self
                    .portfolio
                    .clone()
                    .unwrap_or_default()
                    .positions
                    .iter()
                    .enumerate()
                {
                    let mut delta = PositionDelta::default();
                    delta.balance = self.deltas.balance.get(&index).cloned();

                    let mut targets: Vec<Option<Targetable>> = vec![];
                    for (i, target) in position.targets.iter().enumerate() {
                        if let Some(delta) = self.deltas.weight.get(&i).cloned() {
                            targets.push(Some(Targetable::Weight(0.00).from_string(delta)));
                        }
                    }

                    if targets.len() > 0 {
                        delta.targets = targets;
                    }

                    deltas.push(delta);
                }

                // Update summary.
                self.summary = Some(DeltaSummary::default().deltas(deltas));

                // Make it ready to review.
                self.ready = true;
            }
            Message::Review(review::Message::Form(review::FormMessage::Submit)) => {
                tracing::trace!("Simulating...");

                let cmd = match &mut self.state {
                    DashboardState::Review(review) => review
                        .update(review::Message::Form(review::FormMessage::Submit))
                        .map(|x| x.into()),
                    _ => Command::none(),
                };

                self.state = DashboardState::Simulate(Simulate::default());

                // return self
                // .state
                // .as_mut()
                // .unwrap()
                // .update(review::Message::Form(review::FormMessage::Submit))
                // .map(|x| x.into());

                return cmd;
            }
            Message::Review(message) => {
                let cmd = match &mut self.state {
                    DashboardState::Review(review) => review.update(message).map(|x| x.into()),
                    _ => Command::none(),
                };

                // return self
                // .review
                // .as_mut()
                // .unwrap()
                // .update(message)
                // .map(|x| x.into());

                return cmd;
            }
            Message::Simulated(simulate::Message::Submit) => {
                tracing::trace!("Executing adjustment...");
                self.state.clear();
                self.summary = None;
                self.deltas.clear();
                self.ready = false;
            }
            _ => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let table: Element<'a, Message> = self.position_table().into();

        let submit = match self.ready {
            true => Some(Message::Submit),
            false => None,
        };

        let instruct: Element<'a, Message> = instructions(
            vec![
                instruction_text("Edit the deltas for each position.".to_string()),
                instruction_text(
                    "Deltas are used to calculate the portfolio's metrics.".to_string(),
                ),
            ],
            Some("Edit Deltas".to_string()),
            None,
            submit,
        )
        .into();

        let mut sub_section = Row::new().spacing(Sizes::Lg);

        if self.summary.is_some() {
            sub_section = sub_section.push(self.summary.clone().unwrap().view());
        }

        sub_section = sub_section.push(
            Column::new()
                .align_items(alignment::Alignment::End)
                .push(instruct.map(|x| view::Message::Developer(developer::Message::Dash(x))))
                .width(Length::FillPortion(1)),
        );

        let summarize: Element<'a, Message> = action_button("Review Adjustments".to_string())
            .on_press(Message::ReviewAdjustment)
            .into();

        let mut content = Column::new()
            .spacing(20)
            .push(h1("Dashboard".to_string()).size(TitleSize::Xl))
            .push(
                Row::new()
                    .push(label_item("Positions".to_string()).size(TitleSize::Sm))
                    .push(summarize.map(|x| view::Message::Developer(developer::Message::Dash(x)))),
            )
            .push(table.map(|x| view::Message::Developer(developer::Message::Dash(x))))
            .push(sub_section);

        match &self.state {
            DashboardState::Review(review) => {
                content = content.push(review.view().map(|x| {
                    view::Message::Developer(developer::Message::Dash(Message::Review(x)))
                }));
            }
            DashboardState::Simulate(simulate) => {
                content = content.push(simulate.view().map(|x| {
                    view::Message::Developer(developer::Message::Dash(Message::Simulated(x)))
                }));
            }
            _ => {}
        }

        Container::new(content)
            .align_y(alignment::Vertical::Top)
            .center_x()
            .max_height(ByteScale::Xl7)
            .max_width(ByteScale::Xl7.between(&ByteScale::Xl8))
            .padding(Sizes::Xl)
            .into()
    }
}

/// Individual deltas of a given position.
#[derive(Debug, Clone, Default)]
pub struct PositionDelta {
    pub balance: Option<String>,
    pub targets: Vec<Option<Targetable>>,
}

/// Renders a summary of the adjustments made to the portfolio, given the
/// current deltas.
#[derive(Debug, Clone, Default)]
pub struct DeltaSummary {
    pub deltas: Vec<PositionDelta>,
}

#[derive(Debug, Clone, Default)]
pub enum DeltaMessage {
    #[default]
    Empty,
    ChangeBalance(usize, Option<String>),
    ChangeTarget(usize, usize, Option<Targetable>),
}

impl From<DeltaMessage> for Message {
    fn from(msg: DeltaMessage) -> Self {
        Message::Delta(msg)
    }
}

impl DeltaSummary {
    pub fn new() -> Self {
        Self {
            deltas: vec![PositionDelta::default()],
        }
    }

    pub fn deltas(self, deltas: Vec<PositionDelta>) -> Self {
        Self { deltas }
    }

    pub fn table(&self, index: usize) -> TableBuilder<DeltaMessage> {
        let position = self
            .deltas
            .get(index)
            .unwrap_or(&PositionDelta::default())
            .clone();

        let mut data: Vec<(String, String)> = Vec::new();
        if let Some(balance) = position.balance.clone() {
            data.push(("Balance".to_string(), balance));
        }

        for (i, target) in position.targets.iter().enumerate() {
            if let Some(target) = target.clone() {
                data.push((format!("{:?}", target), target.to_string()));
            }
        }

        TableBuilder::new().padding_cell(Sizes::Md).column(
            ColumnBuilder::new()
                .headers(vec!["Field".to_string(), "Delta".to_string()])
                .rows(
                    data.iter()
                        .map(|(label, value)| {
                            RowBuilder::new()
                                .style(|| {
                                    CustomContainer::theme(Some(iced::Background::Color(GRAY_500)))
                                })
                                .cells(vec![
                                    CellBuilder::new().value(Some(label.clone())),
                                    CellBuilder::new().value(Some(value.clone())).style(|| {
                                        CustomContainer::theme(Some(iced::Background::Color(
                                            GRAY_400,
                                        )))
                                    }),
                                ])
                        })
                        .collect(),
                ),
        )
    }

    pub fn update(&mut self, message: DeltaMessage) -> Command<app::Message> {
        match message {
            DeltaMessage::ChangeBalance(index, value) => {
                tracing::trace!(
                    "Balance {} changed: {}",
                    index,
                    value.clone().unwrap_or_default()
                );
                self.deltas[index].balance = value;
            }
            DeltaMessage::ChangeTarget(pos_index, target_index, value) => {
                tracing::trace!(
                    "Target {} {} changed: {}",
                    pos_index,
                    target_index,
                    value.clone().unwrap_or_default()
                );
                self.deltas[pos_index].targets[target_index] = value;
            }
            _ => {}
        }

        Command::none()
    }

    pub fn view<'a>(&self) -> Element<'a, view::Message> {
        let mut rows: Vec<Element<'a, view::Message>> = vec![];

        for (i, pos) in self.deltas.iter().enumerate() {
            let table: Element<'a, DeltaMessage> = self.table(i).build().into();
            rows.push(
                Column::new()
                    .spacing(Sizes::Sm)
                    .push(key_value_row("Name".to_string(), format!("Position {}", i)))
                    .push(table.map(|x| {
                        view::Message::Developer(developer::Message::Dash(Message::Delta(x)))
                    }))
                    .into(),
            );
        }

        Column::new()
            .push(label_item("Adjustments Overview".to_string()))
            .push(Row::with_children(rows).spacing(Sizes::Lg))
            .width(Length::FillPortion(3))
            .into()
    }
}
