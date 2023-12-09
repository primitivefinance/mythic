//! Renders a view of the portfolio's positions and strategies.

pub mod stages;
pub mod table;

use std::collections::HashMap;

use datatypes::portfolio::{
    coin::Coin,
    position::{Position, Positions},
    weight::Weight,
    Portfolio,
};
use stages::Stages;
use uuid::Uuid;

use self::{stages::DashboardState, table::PortfolioTable};
use super::*;
use crate::components::{
    system::Card,
    tables::{
        builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, key_value_table,
        rows::RowBuilder,
    },
};

/// Executed on `load` for the Dashboard screen.
#[tracing::instrument(skip(dev_client), ret)]
async fn load_portfolio(
    dev_client: Option<DevClient<DefaultMiddleware>>,
    name: Option<String>,
) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let path = name.clone().map(Portfolio::file_path_with_name);
    let portfolio = Portfolio::load(path);
    let mut portfolio = match portfolio {
        Ok(portfolio) => portfolio,
        Err(_) => {
            // If dev mode, load the dev profile.
            if std::env::var("DEV_MODE").is_ok() {
                // Else create it
                tracing::debug!("Creating the dev portfolio.");
                Portfolio::create_new(Some("dev".to_string()))?
            } else {
                // Else load the default profile.
                tracing::debug!("Loading the default portfolio.");
                Portfolio::load(Some(Portfolio::file_path_with_name("default".to_string())))?
            }
        }
    };

    // If dev mode, load up the basic two coin portfolio.
    if std::env::var("DEV_MODE").is_ok() && dev_client.is_some() {
        let dev_client: DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>> =
            dev_client.clone().unwrap();
        portfolio = fetch_portfolio(portfolio, dev_client).await?;
    }

    Ok(portfolio)
}

async fn fetch_portfolio(
    portfolio: Portfolio,
    dev_client: DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>>,
) -> anyhow::Result<Portfolio, Arc<anyhow::Error>> {
    let mut portfolio = portfolio.clone();
    let coin_x: Coin =
        serde_json::from_str(super::dev::COIN_X).map_err(|e| Arc::new(anyhow::Error::from(e)))?;
    let coin_y: Coin =
        serde_json::from_str(super::dev::COIN_Y).map_err(|e| Arc::new(anyhow::Error::from(e)))?;
    let balance_x = dev_client
        .balance_of_x(dev_client.client().address())
        .await?;
    let balance_y = dev_client
        .balance_of_y(dev_client.client().address())
        .await?;
    let initial_price = super::dev::INITIAL_X_PRICE;
    let balance_x = ethers::utils::format_ether(balance_x)
        .parse::<f64>()
        .unwrap();
    let balance_y = ethers::utils::format_ether(balance_y)
        .parse::<f64>()
        .unwrap();

    // Based on the price of x and the balances, compute the weights of both.
    let total_value = balance_x * initial_price + balance_y;
    let position_x_weight = balance_x * initial_price / total_value;
    let position_y_weight = balance_y / total_value;
    let position_x_weight = Weight {
        id: Uuid::new_v4(),
        value: position_x_weight,
    };
    let position_y_weight = Weight {
        id: Uuid::new_v4(),
        value: position_y_weight,
    };

    let position_x = Position::new(
        coin_x,
        Some(super::dev::INITIAL_X_PRICE),
        Some(balance_x),
        Some(position_x_weight),
        None,
    );

    let position_y = Position::new(
        coin_y,
        Some(super::dev::INITIAL_Y_PRICE),
        Some(balance_y),
        Some(position_y_weight),
        None,
    );

    // Add the positions to the portfolio.
    // note: ran into a problem of adding positions sequentially, as they would be
    // added then the validate() would get called, and if that weight is not 1 then
    // it won't sum to 1.
    // Will need to work on this.
    // Workaround is to override the positions directly.
    let positions = Positions::new(vec![position_x, position_y]);
    portfolio.positions = positions;

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
    /// Triggered every 3s.
    FetchPortfolioState,
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::Dashboard(msg)
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
    /// Dev client for loading the portfolio.
    dev_client: Option<DevClient<DefaultMiddleware>>,
    /// Portfolio that is deposited into an active strategy.
    pub deposited_portfolio: Option<Portfolio>,
    /// Table for the deposited portfolio.
    pub deposited_table: PortfolioTable,
}

impl Dashboard {
    pub type AppMessage = Message;
    pub type ViewMessage = Message;

    /// Try loading the portfolio from the name.
    pub fn new(name: Option<String>, dev_client: Option<DevClient<DefaultMiddleware>>) -> Self {
        Self {
            portfolio: None,
            table: PortfolioTable::new(),
            stage: Stages::new(dev_client.clone()),
            loaded_from: name,
            dev_client,
            deposited_portfolio: None,
            deposited_table: PortfolioTable::new(),
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
            .push(label(&"Dashboard").title1().build())
            .into()
    }

    pub fn render_table(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Lg)
            .push(label(&"Positions").title2().build())
            .push(self.table.view().map(|x| x.into()))
            .into()
    }

    pub fn render_deposited_table(&self) -> Element<'_, Self::AppMessage> {
        // If the table is not loaded, return an empty element.
        if self.deposited_portfolio.is_none() {
            return Container::new(label(&"No deposited positions.").build())
                .center_x()
                .center_y()
                .into();
        }

        Column::new()
            .spacing(Sizes::Lg)
            .push(label(&"Active Strategies").title2().build())
            .push(self.deposited_table.view().map(|x| x.into()))
            .into()
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
                                Card::new(
                                    label(&"Make adjustments to view the estimated results")
                                        .title3()
                                        .build(),
                                )
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
    type ViewMessage = Message;

    /// todo: how to handle different portfolio loads.
    fn load(&self) -> Command<Self::AppMessage> {
        let name = match self.loaded_from.clone() {
            Some(name) => Some(name),
            None => None,
        };

        let mut commands = vec![];

        // Loads the initial portfolio from the dev client
        let dev_client = self.dev_client.clone();
        commands.push(Command::perform(
            load_portfolio(dev_client, None),
            Message::Load,
        ));

        // todo: does this even work for the children components?
        // Loads the staging area, which enters the first stage.
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                self.portfolio = Some(portfolio.clone());
                tracing::debug!("Loaded portfolio: in Load in dashboard {:?}", portfolio);

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
                let mut commands = vec![];
                // If its an Execute::NewStrategyPosition message, then update the deposited
                // portfolio.
                if let stages::Message::Execute(stages::execute::Message::NewStrategyPosition(
                    portfolio,
                )) = stage.clone()
                {
                    self.deposited_portfolio = Some(portfolio.clone());
                    commands.push(
                        self.deposited_table
                            .update(table::Message::Load(portfolio.clone()))
                            .map(|x| x.into()),
                    );
                }

                commands.push(self.stage.update(stage).map(|x| x.into()));

                return Command::batch(commands);
            }
            Message::FetchPortfolioState => {
                // Refetch the origin and deposited portfolios via dev client, if dev client is
                // available.
                if let (Some(dev_client), Some(origin)) =
                    (&self.dev_client, &self.portfolio).clone()
                {
                    let dev_client = dev_client.clone();
                    let origin = self.portfolio.clone();
                    let deposited = self.deposited_portfolio.clone();

                    let mut commands = vec![];

                    commands.push(Command::perform(
                        async move {
                            let origin = match origin {
                                Some(origin) => {
                                    Some(fetch_portfolio(origin, dev_client.clone()).await?)
                                }
                                None => None,
                            };
                            let deposited = match deposited {
                                Some(deposited) => {
                                    Some(fetch_portfolio(deposited, dev_client.clone()).await?)
                                }
                                None => None,
                            };
                            Ok::<(Option<Portfolio>, Option<Portfolio>), Arc<anyhow::Error>>((
                                origin, deposited,
                            ))
                        },
                        move |result| match result {
                            Ok((origin, _deposited)) => {
                                tracing::debug!(
                                    "Fetched portfolio state in FETCH message. {:?}",
                                    origin.clone()
                                );
                                if let Some(origin) = origin {
                                    Message::Load(Ok(origin))
                                } else {
                                    Message::Empty
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to fetch portfolio state: {:?}", e);
                                Message::Empty
                            }
                        },
                    ));

                    // Fetches the deposited position, which emits an event that we capture in here
                    // anyway.
                    commands.push(Command::perform(async {}, |_| {
                        Message::UpdateStaging(stages::Message::Execute(
                            stages::execute::Message::FetchPosition,
                        ))
                    }));
                    return Command::batch(commands);
                }
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Lg);
        content = content.push(self.render_header().map(|x| x.into()));
        content = content.push(self.render_deposited_table().map(|x| x.into()));
        content = content.push(self.render_table().map(|x| x.into()));
        content = content.push(self.render_staging_area().map(|x| x.into()));

        Container::new(content)
            .align_y(alignment::Vertical::Top)
            .center_x()
            .max_height(ByteScale::Xl7)
            .max_width(ByteScale::Xl7.between(&ByteScale::Xl8))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        // every 5s fetch portfolio state
        iced::time::every(std::time::Duration::from_secs(5)).map(|_| Message::FetchPortfolioState)
    }
}
