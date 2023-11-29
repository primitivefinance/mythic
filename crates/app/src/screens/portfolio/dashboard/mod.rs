//! Renders a view of the portfolio's positions and strategies.

pub mod stages;
pub mod table;

use std::collections::HashMap;

use profiles::portfolios::Portfolio;
use stages::Stages;

use self::{
    stages::DashboardState,
    table::{PortfolioTable, PositionDelta},
};
use super::*;
use crate::{
    components::{
        containers::CustomContainer,
        tables::{
            builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, key_value_table,
            rows::RowBuilder,
        },
    },
    screens::portfolio::dashboard::stages::prepare::adjust_weights_algorithm,
};

/// Executed on `load` for the Dashboard screen.
#[tracing::instrument(skip(name), ret)]
async fn load_portfolio(name: Option<String>) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let path = name.map(Portfolio::file_path_with_name);
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
    /// Triggered after `load_portfolio` completes.
    Load(anyhow::Result<Portfolio, Arc<anyhow::Error>>),
    /// Triggered when a user wants to review the proposed adjustments.
    Prepare,
    /// Triggered action from the main button on the dashboard.
    BeginAdjustment,
    /// Triggered when the user clicks the review adjustments button.
    Stage(stages::Message),
    /// Triggered when the underlying portfolio table form is edited.
    PortfolioTable(table::Message),
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
    /// Underlying data structure.
    portfolio: Option<Portfolio>,
    /// Table to render the underlying data.
    table: PortfolioTable,
    /// Stages of the dashboard for interacting with the underlying data.
    stage: Stages,
    /// Original name of the loaded portfolio.
    loaded_from: Option<String>,
}

impl Dashboard {
    pub type AppMessage = Message;
    pub type ViewMessage = view::Message;

    /// Try loading the portfolio from the name.
    pub fn new(name: Option<String>) -> Self {
        Self {
            portfolio: None,
            table: PortfolioTable::new(),
            stage: Stages::new(),
            loaded_from: name,
        }
    }

    pub fn loaded(&self) -> bool {
        self.portfolio.is_some()
    }

    pub fn render_header(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Md)
            .push(h1("Portfolio Dashboard".to_string()).size(TitleSize::Xl))
            .into()
    }

    pub fn render_table(&self) -> Element<'_, Self::AppMessage> {
        self.table.view().map(|x| x.into())
    }

    pub fn render_stages(&self) -> Element<'_, Self::AppMessage> {
        match self.stage.current {
            DashboardState::Empty => {
                let instruct: Element<'_, Self::AppMessage> = instructions(
                    vec![instruction_text(
                        "Change the position deltas in the table to start the portfolio adjustment process.".to_string(),
                    )],
                    Some("Continue".to_string()),
                    None,
                    None,
                )
                .into();

                Row::new()
                    .spacing(Sizes::Lg)
                    .push(
                        Column::new()
                            .align_items(alignment::Alignment::Start)
                            .push(
                                Card::new(h3(
                                    "Make adjustments to view the estimated results".to_string()
                                ))
                                .center_x()
                                .center_y()
                                .padding(Sizes::Lg)
                                .width(Length::Fill),
                            )
                            .width(Length::FillPortion(3)),
                    )
                    .push(
                        Column::new()
                            .align_items(alignment::Alignment::End)
                            .push(instruct)
                            .width(Length::FillPortion(1)),
                    )
                    .into()
            }
            _ => self.stage.view().map(|x| x.into()),
        }
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

        let mut commands = vec![];
        commands.push(Command::perform(load_portfolio(name), Message::Load));

        // todo: does this even work for the children components?
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                self.portfolio = Some(portfolio.clone());

                let mut commands: Vec<Command<Self::AppMessage>> = vec![];

                commands.push(
                    self.stage
                        .update(stages::Message::LoadPortfolio(portfolio.clone()))
                        .map(|x| x.into()),
                );

                commands.push(
                    self.table
                        .update(table::Message::Portfolio(portfolio.clone()))
                        .map(|x| x.into()),
                );

                return Command::batch(commands);
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load portfolio: {:?}", e);
            }
            Message::PortfolioTable(message) => {
                let mut commands = vec![];

                // If the table changes one of its weight fields, "catch" it to fresh the
                // preview.
                if let table::Message::DeltaForm(table::form::DeltaFormMessage::Weight(_, _)) =
                    message
                {
                    commands.push(Command::perform(async {}, |_| Message::BeginAdjustment));
                }

                commands.push(self.table.update(message.clone()).map(|x| x.into()));

                return Command::batch(commands);
            }
            Message::Stage(stage) => {
                return self.stage.update(stage).map(|x| x.into());
            }
            // Renders the summary of adjustments table.
            Message::Prepare => {
                return self.table.update(table::Message::Ready).map(|x| x.into());
            }

            // TRIGGERED WHEN USER ENTERS A DELTA IN THE TABLE, VERY IMPORTANT EVENT!
            // Triggers the staging process...
            // todo: move this weight adjustment computation outside so its easier to find / debug.
            // todo: this can be triggered by directly emitting Stage(Step) message.
            Message::BeginAdjustment => {
                // placeholder, but should try to fetch the current prices and update this.
                let mut proposed_deltas = self.table.get_form_deltas();
                // If all position deltas are empty, then reset the staging.
                if proposed_deltas.iter().all(|x| x.is_empty()) {
                    tracing::info!("Resetting stages...");
                    return self.stage.update(stages::Message::Reset).map(|x| x.into());
                }

                // Get the weights of the portfolio
                let mut weights: Vec<f64> = self
                    .portfolio
                    .clone()
                    .unwrap_or_default()
                    .positions
                    .iter()
                    .map(|position| position.weight.unwrap_or_default())
                    .collect();

                // Apply the adjustments to the weights
                let changes: Vec<(usize, f64)> = proposed_deltas
                    .iter()
                    .map(|delta| {
                        let pos_index = delta.id;
                        let weight = delta
                            .weight
                            .clone()
                            .unwrap_or_default()
                            .parse::<f64>()
                            .unwrap_or_default();
                        (pos_index, weight)
                    })
                    .collect();

                adjust_weights_algorithm(&mut weights, changes);

                // Return the new vector of deltas
                proposed_deltas = weights
                    .iter()
                    .enumerate()
                    .map(|(pos_index, &weight)| {
                        let mut delta = proposed_deltas[pos_index].clone();
                        delta.weight = Some(weight.to_string());
                        delta
                    })
                    .collect();

                for delta in &mut proposed_deltas {
                    if let Some(position) = self
                        .portfolio
                        .clone()
                        .unwrap_or_default()
                        .positions
                        .get(delta.id)
                    {
                        delta.price = position.cost.map(|cost| cost.to_string());
                    }
                }

                // We also need to compute the changes in the balances of the portfolio!
                // Then we need to compute the change in market value based in the price, and
                // apply the adjustments to the table delta fields.
                let adjusted_balances = compute_balance_adjustments_algorithm(
                    &self.portfolio.clone().unwrap_or_default(),
                    &proposed_deltas,
                );

                proposed_deltas = proposed_deltas
                    .iter()
                    .enumerate()
                    .map(|(pos_index, position)| {
                        let mut position = position.clone();
                        position.balance = adjusted_balances[pos_index].balance.clone();
                        position
                    })
                    .collect();

                let adjusted_market_values = adjusted_balances
                    .iter()
                    .map(|balance| {
                        let price = balance
                            .price
                            .clone()
                            .unwrap_or_default()
                            .parse::<f64>()
                            .unwrap_or_default();
                        let balance = balance
                            .balance
                            .clone()
                            .unwrap_or_default()
                            .parse::<f64>()
                            .unwrap_or_default();
                        price * balance
                    })
                    .collect::<Vec<f64>>();

                // For each position delta, we need to send a message to update the table form.
                let mut commands = proposed_deltas
                    .iter()
                    .enumerate()
                    .map(|(pos_index, position)| {
                        let balance = position.balance.clone();
                        let market_value = adjusted_market_values[pos_index].to_string();

                        let balance_command = self
                            .table
                            .update(table::Message::DeltaForm(
                                table::form::DeltaFormMessage::Balance(pos_index, balance),
                            ))
                            .map(|x| x.into());

                        let market_value_command = self
                            .table
                            .update(table::Message::DeltaForm(
                                table::form::DeltaFormMessage::MarketValue(
                                    pos_index,
                                    Some(market_value.clone()),
                                ),
                            ))
                            .map(|x| x.into());

                        vec![balance_command, market_value_command]
                    })
                    .flatten()
                    .collect::<Vec<Command<Self::AppMessage>>>();

                commands.push(
                    self.stage
                        .update(stages::Message::Start(proposed_deltas))
                        .map(|x| x.into()),
                );

                return Command::batch(commands);
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Lg);
        content = content.push(self.render_header().map(|x| x.into()));
        content = content.push(self.render_table().map(|x| x.into()));
        content = content.push(self.render_stages().map(|x| x.into()));

        Container::new(content)
            .align_y(alignment::Vertical::Top)
            .center_x()
            .max_height(ByteScale::Xl7)
            .max_width(ByteScale::Xl7.between(&ByteScale::Xl8))
            .into()
    }
}

/// Wrapper around the dashboard so it can be used directly from the root app.
pub struct DashboardWrapper {
    pub dashboard: Dashboard,
}

impl From<DashboardWrapper> for Screen {
    fn from(dashboard: DashboardWrapper) -> Self {
        Screen::new(Box::new(dashboard))
    }
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
        cmd.map(|x| x.into())
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

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        self.dashboard.view()
    }
}

/// i hate this please clean it up
#[tracing::instrument(skip(portfolio), ret)]
pub fn compute_balance_adjustments_algorithm(
    portfolio: &Portfolio,
    proposed_deltas: &Vec<PositionDelta>,
) -> Vec<PositionDelta> {
    let total_portfolio_value = portfolio.compute_total_portfolio_value();
    tracing::info!("Total portfolio value: {}", total_portfolio_value);

    let mut adjusted_balances: Vec<PositionDelta> = vec![];

    let prices: Vec<f64> = portfolio
        .positions
        .iter()
        .map(|position| position.cost.unwrap_or_default())
        .collect::<Vec<f64>>();

    // For each position delta, apply the weight adjustment to the balances of the
    // coins
    for (index, proposed_delta) in proposed_deltas.iter().enumerate() {
        let mut adjusted_balance = PositionDelta::default();
        adjusted_balance.id = index;
        if let Some(weight_str) = &proposed_delta.weight {
            if let Ok(weight) = weight_str.parse::<f64>() {
                tracing::info!("Weight: {}", weight);
                adjusted_balance.balance = Some(
                    (weight * total_portfolio_value / prices[index])
                        .round()
                        .to_string(),
                );

                tracing::info!(
                    "Adjusted balance: {}",
                    adjusted_balance.balance.clone().unwrap_or_default()
                );
            }
        }

        if let Some(price_str) = &proposed_delta.price {
            if let Ok(price) = price_str.parse::<f64>() {
                adjusted_balance.price = Some(price.to_string());
            }
        } else {
            // Else set to the current price.
            // This is mostly to make it easier to compute the market value.
            adjusted_balance.price = Some(prices[index].to_string());
        }

        adjusted_balances.push(adjusted_balance);
    }

    // Return the balances of coins with the new weights
    adjusted_balances
}
