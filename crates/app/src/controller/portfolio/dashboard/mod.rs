//! Renders a view of the portfolio's positions and strategies.

pub mod stages;
pub mod table;

use std::collections::HashMap;

use alloy_primitives::utils::parse_ether;
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use datatypes::portfolio::Portfolio;
use sim::{from_ethers_u256, to_ethers_address, to_ethers_u256};
use stages::Stages;
use RustQuant::stochastics::{GeometricBrownianMotion, StochasticProcess, Trajectories};

use self::{stages::DashboardState, table::PortfolioTable};
use super::*;
use crate::{
    components::{
        system::{Card, ExcaliburButton, ExcaliburColor, ExcaliburContainer},
        tables::{
            builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, key_value_table,
            rows::RowBuilder,
        },
    },
    model::portfolio::ALLOY_WAD,
    view::portfolio_view::{DataView, PortfolioPresenter},
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Triggered after `load_portfolio` completes.
    Load(Option<Portfolio>),
    /// Triggered on any form changes or button clicks within the staging area.
    UpdateStaging(stages::Message),
    /// Triggered when the user inputs a delta in the table.
    UpdateTable(table::Message),
    /// Triggered when new data is needs to be fetched.
    UpdateDataModel(Result<Model, Arc<anyhow::Error>>),
    /// Triggered when the view model needs to be synced to the data model.
    UpdateDataView,
    /// A 1s subscription.
    Tick,
    /// todo: remove
    /// Why do we need to remove this?
    Refetch,
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
    /// Connection to the network!
    pub client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,

    /// Root application model.
    pub model: Model,

    /// The table state.
    pub table: PortfolioTable,

    /// The current action that the user is taking.
    pub stage: Stages,

    /// Transforms Model -> View components
    pub presenter: PortfolioPresenter,

    /// Transforms View components -> iced Elements
    pub renderer: DataView,

    /// A test price process
    pub test_price_process: Option<PriceProcess>,
}

impl Dashboard {
    /// Try loading the portfolio from the name.
    /// Calling `new` needs to be really fast, or else the UI will lag.
    /// Therefore, we move as much logic as possible to the `load` function.
    /// In the load function, the presenter can be updated with the model.
    /// Since this happens in `load` instead of `new`, there's no lag when
    /// opening the page.
    pub fn new(
        client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
        model: Model,
    ) -> Self {
        let presenter = PortfolioPresenter::default();
        let renderer = DataView;

        let process = Some(PriceProcess {
            trajectories: GeometricBrownianMotion::new(0.05, 0.9)
                .euler_maruyama(1.0, 0.0, 1.0, 1000, 1, false),
            step: 0,
            max_steps: 1000,
        });

        Self {
            client: client.clone(),
            model,
            table: PortfolioTable::new(),
            stage: Stages::new(client),
            presenter,
            renderer,
            test_price_process: process,
        }
    }

    pub fn handle_updated_model(&mut self, updated_model: Model) -> Command<Message> {
        // Update the model
        self.model = updated_model.clone();

        // Update the presenter with the new model.
        self.presenter.update(updated_model.clone());

        Command::perform(async {}, move |_| {
            Message::Load(Some(updated_model.user.portfolio.clone()))
        })
    }

    pub fn loaded(&self) -> bool {
        self.client.is_some()
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub fn update_data(&self) -> Command<Message> {
        Command::perform(async {}, |_| Message::Refetch)
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub fn adjusted_portfolio_from_table(&self) -> Option<Portfolio> {
        let mut portfolio = self.model.user.portfolio.clone();
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
            let weight = position.weight.unwrap_or_default();
            portfolio.adjust(weight.id, delta).unwrap();
            adjusted = true;
        }

        if !adjusted {
            return None;
        }

        Some(portfolio)
    }

    pub fn render_staging_area(&self) -> Element<'_, Message> {
        match self.stage.current {
            DashboardState::Empty => {
                let instruct: Element<'_, Message> = instructions(
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
                                Card::build_container(
                                    label("Make adjustments to view the estimated results")
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
    /// I think we would have to make a different command for each portfolio load.
    /// then we can implement an abstraction over that to put in here.
    fn load(&self) -> Command<Message> {
        let mut commands = vec![];

        // Populates the presenter's data to prepare for rendering.
        let model = self.model.clone();
        commands.push(Command::perform(async {}, |_| {
            Message::UpdateDataModel(Ok(model))
        }));

        // todo: does this even work for the children components?

        //  AFAIK Child components can indirectly cause updates in parent components 
        // through messages. When a child component generates a message (usually as a 
        // result of user interaction), this message is propagated up to the parent component. 
        // The parent component can then handle this message in its update method and change its state accordingly.
        // it can indirectly cause the parent to change its own state by sending it a message. 
        
        // Loads the staging area, which enters the first stage.
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Load(portfolio) => {
                let mut commands: Vec<Command<Message>> = vec![];
                let portfolio = portfolio.unwrap_or_default();

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
            Message::UpdateDataModel(Ok(model)) => return self.handle_updated_model(model.clone()),
            Message::UpdateDataModel(Err(e)) => {
                tracing::error!("Failed to update data model: {:?}", e);
            }
            Message::Tick => {
                let mut commands = vec![];

                let mut next_price = None;
                if let Some(process) = self.test_price_process.as_mut() {
                    process.step += 1;
                    if process.step < process.max_steps {
                        let price = process.trajectories.paths[0].get(process.step).cloned();
                        if let Some(price) = price {
                            next_price = Some(price);
                        }
                    }
                };

                if let (Some(external_exchange), Some(client)) = (
                    &self.model.portfolio.raw_external_exchange_address,
                    &self.client,
                ) {
                    tracing::info!("Tick, updating price.");
                    let client = client.client().cloned().unwrap();
                    // for testing live price chart.
                    let external_exchange = *external_exchange;

                    commands.push(Command::perform(
                        async move {
                            let next_price = next_price.unwrap_or_default();
                            let client = client.clone();
                            let lex =
                                LiquidExchange::new(to_ethers_address(external_exchange), client);

                            let current_price = lex.price().await?;
                            // make the new price a random price +/- 1% of current price.
                            let random = 1.0 + (rand::random::<f64>() - 0.5) * 0.01;
                            let random = parse_ether(format!("{}", random).as_str()).unwrap();
                            let mut new_price = from_ethers_u256(current_price)
                                .checked_mul(random)
                                .unwrap()
                                .checked_div(ALLOY_WAD)
                                .unwrap();

                            if next_price > 0.0 {
                                new_price = parse_ether(&format!("{}", next_price))?;
                            }

                            let result =
                                lex.set_price(to_ethers_u256(new_price)).send().await?.await;

                            match result {
                                Ok(_tx) => {
                                    tracing::info!("Updated price");
                                    Ok(())
                                }
                                Err(e) => {
                                    tracing::error!("Failed to set price: {:?}", e);
                                    Err(anyhow::Error::from(e))
                                }
                            }
                        },
                        |_| Message::Empty,
                    ));
                }

                commands.push(self.update_data());

                return Command::batch(commands);
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
                            .flat_map(|(pos_index, position)| {
                                let balance = position.balance.unwrap_or_default().to_string();
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
                            .collect::<Vec<Command<Message>>>();
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
                    _portfolio,
                )) = stage.clone()
                {
                    // todo: update current position table with new strategy
                    // position.
                }

                // Catch the FetchPositionResult and call update_data.
                if let stages::Message::Execute(stages::execute::Message::FetchPositionResult(_)) =
                    stage.clone()
                {
                    tracing::info!("Caught fetch position, updating data model.");
                    commands.push(self.update_data());
                }

                commands.push(self.stage.update(stage).map(|x| x.into()));

                return Command::batch(commands);
            }
            _ => {}
        }

        Command::none()
    }

    // Layout is a 2x2 quadrant grid
    fn view(&self) -> Element<'_, Message> {
        let quadrant_1 = self.renderer.metrics_layout(
            &self.presenter.portfolio_strategy_plot,
            label("Liquidity curve").headline().highlight(),
            self.presenter.get_external_price(),
            self.presenter.get_external_portfolio_value(),
            self.presenter.get_internal_portfolio_value(),
            self.presenter.get_portfolio_health(),
            self.presenter.get_last_sync_timestamp(),
            self.presenter.get_block_number(),
        );
        let quadrant_2 = self.renderer.chart_and_greet_layout(
                &self.presenter.portfolio_value_series,
                label("Good morning, Alex.").title3().highlight(),
                label("Your portfolio has maintained replication health since inception. Consider reviewing your portfolio liquidity distribution to maximize liquidity provision.").body(),
                label("Portfolio value / time").highlight().headline(),
                self.presenter.get_last_sync_timestamp()
            );

        let (table_builder, table_cells) = self
            .renderer
            .get_positions_table_builder(self.presenter.get_positions());

        let quadrant_3 = ExcaliburContainer::default()
            .build(self.renderer.table_layout(
                label("Positions").highlight(),
                vec![ExcaliburButton::new()
                .transparent()
                .build(label("Refetch").caption().secondary().build())
                .on_press(Message::Refetch).into()], // TODO: since we want to remove the refetch should we remove this whole
                table_builder,
                table_cells,
                self.presenter.get_last_sync_timestamp(),
                self.presenter.get_last_sync_block(),
            ))
            .padding(Sizes::Md);
        let quadrant_4 = Container::new(self.render_staging_area());

        Container::new(
            Column::new()
                .spacing(Sizes::Xl)
                .push(
                    Row::new()
                        .spacing(Sizes::Sm)
                        .push(quadrant_2.width(Length::FillPortion(2)))
                        .push(quadrant_1.width(Length::FillPortion(2)))
                        .height(Length::FillPortion(3))
                        .align_items(alignment::Alignment::Start),
                )
                .push(
                    Row::new()
                        .spacing(Sizes::Sm)
                        .push(quadrant_3.width(Length::FillPortion(2)))
                        .push(quadrant_4.width(Length::FillPortion(2)))
                        .height(Length::FillPortion(2)),
                )
                .width(Length::Fill),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let s1 = iced::time::every(std::time::Duration::from_secs(5)).map(|_| Message::Tick);
        Subscription::batch(vec![s1])
    }
}

/// For testing the UI with a "live" price.
pub struct PriceProcess {
    pub trajectories: Trajectories,
    pub step: usize,
    pub max_steps: usize,
}

impl Clone for PriceProcess {
    fn clone(&self) -> Self {
        let times: Vec<f64> = self.trajectories.times.clone();
        let paths: Vec<Vec<f64>> = self.trajectories.paths.clone();
        Self {
            trajectories: Trajectories { times, paths },
            step: self.step,
            max_steps: self.max_steps,
        }
    }
}

impl std::fmt::Debug for PriceProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriceProcess")
            .field("trajectories", &self.trajectories.paths)
            .field("step", &self.step)
            .field("max_steps", &self.max_steps)
            .finish()
    }
}
