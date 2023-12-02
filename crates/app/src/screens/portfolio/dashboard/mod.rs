//! Renders a view of the portfolio's positions and strategies.

pub mod stages;
pub mod table;

use std::collections::HashMap;

use datatypes::{
    portfolio::{coin::Coin, position::Position, weight::Weight, Portfolio},
    weight,
};
use stages::Stages;
use uuid::Uuid;

use self::{stages::DashboardState, table::PortfolioTable};
use super::*;
use crate::components::{
    containers::CustomContainer,
    tables::{
        builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, key_value_table,
        rows::RowBuilder,
    },
};

/// Executed on `load` for the Dashboard screen.
#[tracing::instrument(skip(name), ret)]
async fn load_portfolio(name: Option<String>) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let path = name.clone().map(Portfolio::file_path_with_name);
    let portfolio = Portfolio::load(path);
    let mut portfolio = match portfolio {
        Ok(portfolio) => portfolio,
        Err(e) => {
            tracing::error!("Failed to load portfolio: {:?}", e);
            // return Err(Arc::new(e));

            tracing::info!("Creating a new default portfolio.");
            Portfolio::create_new(name.clone())?
        }
    };

    // if dev mode, load up the basic two coin portfolio.
    if std::env::var("DEV_MODE").is_ok() {
        let coin_x: Coin = serde_json::from_str(super::dev::COIN_X)
            .map_err(|e| Arc::new(anyhow::Error::from(e)))?;
        let coin_y: Coin = serde_json::from_str(super::dev::COIN_Y)
            .map_err(|e| Arc::new(anyhow::Error::from(e)))?;

        let position_x_weight = Weight {
            id: Uuid::new_v4(),
            value: super::dev::INITIAL_X_WEIGHT.value,
        };

        let position_x = Position::new(
            coin_x,
            Some(super::dev::INITIAL_X_PRICE),
            Some(super::dev::INITIAL_X_BALANCE),
            Some(position_x_weight),
            None,
        );

        let position_y_weight = Weight {
            id: Uuid::new_v4(),
            value: 1.0,
        };

        let position_y = Position::new(
            coin_y,
            Some(super::dev::INITIAL_Y_PRICE),
            Some(super::dev::INITIAL_Y_BALANCE),
            Some(position_y_weight),
            None,
        );

        // Add the positions to the portfolio.
        // note: ran into a problem of adding positions sequentially, as they would be
        // added then the validate() would get called, and if that weight is not 1 then
        // it won't sum to 1.
        // Will need to work on this.
        // Workaround is to set the first weight to 1.0, then add the second position.
        portfolio += position_y;
        portfolio += position_x;
    }

    Ok(portfolio)
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Triggered after `load_portfolio` completes.
    Load(anyhow::Result<Portfolio, Arc<anyhow::Error>>),
    /// Triggered on any form changes or button clicks within the staging area.
    UpdateStaging(stages::Message),
    /// Triggered when the user inputs a delta in the table.
    UpdateTable(table::Message),
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

    #[tracing::instrument(skip(self), level = "debug")]
    pub fn adjusted_portfolio_from_table(&self) -> Option<Portfolio> {
        let mut portfolio = self.portfolio.clone()?;
        let mut positions = portfolio.positions.0.clone();

        // Loop over each position and see if there is an existing __weight__ delta.
        let mut adjusted = false;
        for (i, position) in positions.iter_mut().enumerate() {
            let delta = self.table.form.weight.get(&i).cloned();

            // Skip if delta == "-"
            if delta.is_none() || delta.clone().unwrap() == "-" {
                continue;
            }

            let delta = delta.unwrap().parse::<f64>();
            let delta = match delta {
                Ok(delta) => delta,
                Err(e) => {
                    tracing::error!("Failed to parse delta: {:?}", e);
                    continue;
                }
            };

            //  Adjust the portfolio with using the weight delta.
            let weight = position.weight.unwrap_or_default().clone();
            portfolio.adjust(weight.id, delta).unwrap();
            adjusted = true;
        }

        if !adjusted {
            return None;
        }

        Some(portfolio)
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

    pub fn render_staging_area(&self) -> Element<'_, Self::AppMessage> {
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
            None => None,
        };

        let mut commands = vec![];

        // Loads the initial portfolio from file.
        commands.push(Command::perform(load_portfolio(name), Message::Load));

        // todo: does this even work for the children components?
        // Loads the staging area, which enters the first stage.
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                self.portfolio = Some(portfolio.clone());

                let mut commands: Vec<Command<Self::AppMessage>> = vec![];

                // Store the portfolio in the staging area to reference it.
                commands.push(
                    self.stage
                        .update(stages::Message::Load(portfolio.clone()))
                        .map(|x| x.into()),
                );

                // Store the portfolio in the table to reference it.
                commands.push(
                    self.table
                        .update(table::Message::Load(portfolio.clone()))
                        .map(|x| x.into()),
                );

                return Command::batch(commands);
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load portfolio: {:?}", e);
            }
            // todo: this might be a little slow, since it gets the adjusted portfolio.
            Message::UpdateTable(message) => {
                // Catch the WeightUpdated message and try to update the staging area with the
                // adjusted portfolio.
                if let table::Message::DeltaForm(table::form::DeltaFormMessage::WeightUpdated) =
                    message
                {
                    tracing::debug!("Weight updated, updating staging area.");

                    let adjusted = self.adjusted_portfolio_from_table();
                    if adjusted.is_none() {
                        tracing::debug!("Weight updated but adjusted portfolio is None.");
                    }

                    let mut commands = vec![];

                    // todo: clean up this logic?
                    if adjusted.is_some() {
                        commands = adjusted
                            .clone()
                            .unwrap()
                            .positions
                            .0
                            .iter()
                            .enumerate()
                            .map(|(pos_index, position)| {
                                let balance =
                                    position.balance.clone().unwrap_or_default().to_string();
                                let market_value = position.market_value().clone().to_string();

                                let balance_command = self
                                    .table
                                    .update(table::Message::DeltaForm(
                                        table::form::DeltaFormMessage::Balance(
                                            pos_index,
                                            Some(balance),
                                        ),
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
                    }

                    commands.push(
                        self.stage
                            .update(stages::Message::SetAdjusted(adjusted))
                            .map(|x| x.into()),
                    );

                    return Command::batch(commands);
                }

                return self.table.update(message.clone()).map(|x| x.into());
            }
            Message::UpdateStaging(stage) => {
                return self.stage.update(stage).map(|x| x.into());
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Lg);
        content = content.push(self.render_header().map(|x| x.into()));
        content = content.push(self.render_table().map(|x| x.into()));
        content = content.push(self.render_staging_area().map(|x| x.into()));

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
