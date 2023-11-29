//! Handles the different stages for interacting with a portfolio, from just
//! viewing it to executing adjustments.

pub mod execute;
pub mod prepare;
pub mod review;
pub mod simulate;

use clients::arbiter::portfolio_adjustment::MiniWorldBuilder;
use ethers::utils::parse_ether;
use simulation::agents::token_admin::TokenData;

use self::{prepare::PreparePayload, review::ReviewPackage};
use super::{table::PositionDelta, *};

/// Stores the actual state of the stage in the enum variant argument.
/// Weird? It works.
#[derive(Debug, Clone, Default)]
pub enum DashboardState {
    #[default]
    Empty,
    /// State of reviewing and finalizing the adjustments to make.
    Prepare(prepare::Prepare),
    /// State of reviewing the portfolio adjustment transaction.
    Review(review::ReviewAdjustment),
    /// State of simulating the portfolio adjustment transaction.
    Simulate(simulate::Simulate),
    /// State of executing the portfolio adjustment transaction.
    Execute,
}

impl DashboardState {
    pub fn clear(&mut self) {
        *self = DashboardState::Empty;
    }

    pub fn guide(&self) -> Container<'static, Message> {
        match self {
            DashboardState::Empty => Container::new(Column::new()),
            DashboardState::Prepare(state) => state.guide(Some(Message::Step)),
            DashboardState::Review(state) => state.guide(),
            DashboardState::Simulate(state) => state.guide(Some(Message::Step)),
            DashboardState::Execute => Container::new(Column::new()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Steps the stage forward.
    Step,
    /// Routes to a target stage.
    Route(DashboardState),
    /// Resets the staging to the first step.
    Reset,
    /// Message for Empty -> Prepare stage.
    /// Needs the position index and adjustments, and the estimated price.
    Start(Vec<PositionDelta>),
    /// Prepares the adjustments for simulation.
    Prepare(prepare::Message),
    /// Updates the review stage.
    Review(review::Message),
    /// Updates the simulate stage.
    Simulate(simulate::Message),
    /// Loads the portfolio.
    LoadPortfolio(Portfolio),
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::Stage(msg)
    }
}

/// Quick guide on how the stages work:
/// - Stages is initialized with an Empty current state and empty vec of
///   MiniWorldBuilders.
/// - When step is called from the Empty state we go to the Review state, which
///   has a form
///  for the user to fill out. This form provides us data we need for
/// constructing the MiniWorldBuilder.
/// - When step is called from the Review state we construct the
///   MiniWorldBuilder and add it to the vec.
/// - Then we go to the Simulate state, which stores the world builders to use
///   them
///  in `load` to async spawn the simulations.
#[derive(Debug, Clone, Default)]
pub struct Stages {
    pub portfolio: Option<Portfolio>,
    pub current: DashboardState,
    pub simulations: Vec<MiniWorldBuilder>,
    pub weight_changes: Option<PreparePayload>,
    pub strategy_parameters: Option<ReviewPackage>,
}

impl Stages {
    pub type AppMessage = Message;

    pub fn new() -> Self {
        Self {
            portfolio: None,
            current: DashboardState::Empty,
            simulations: vec![],
            weight_changes: None,
            strategy_parameters: None,
        }
    }

    pub fn construct(&mut self) {
        if let Some(ref portfolio) = self.portfolio {
            // Start editing the config builder, using the current state of the
            // review form and the portfolio.
            let mut builder = MiniWorldBuilder::default();

            // Compute the amount of steps given the time step size of 15 and the time range
            // provided by the user.
            // todo: this is temp value, but do above
            let steps: usize = 1000;
            // Initial amount to deposit.
            let deposit = parse_ether(0.01).unwrap();
            // initial price.
            let price = parse_ether(1.0).unwrap();
            // Time between blocks, in seconds.
            let timestep_size: u64 = 15;

            // First add the coins. Make sure they are in the portfolio.
            let coin_x: TokenData = portfolio
                .positions
                .iter()
                .find(|x| x.asset.symbol == "X")
                .expect("no X coin found")
                .clone()
                .into();
            let coin_y: TokenData = portfolio
                .positions
                .iter()
                .find(|x| x.asset.symbol == "Y")
                .expect("no Y coin found")
                .clone()
                .into();
            builder.config_builder.coins(coin_x, coin_y);

            // Edit the lp agent.
            builder.config_builder.deposit_x(deposit);
            builder.config_builder.initial_price(price);

            // Edit the seconds between blocks.
            builder.config_builder.timestep_size(timestep_size);

            // Edit the price and amount of steps.
            // todo: matching the dca/static.toml config right now, change later
            builder.config_builder.price_changer.seed(1);
            builder.config_builder.price_changer.num_steps(steps);
            builder.config_builder.price_changer.num_paths(10);
            builder.config_builder.price_changer.initial_price(price);
            builder.config_builder.price_changer.t_0(0.0);
            builder.config_builder.price_changer.t_n(0.1);
            builder.config_builder.price_changer.drift(0.1);
            builder.config_builder.price_changer.volatility(0.35);

            // Edit the portfolio manager.
            let fee_wad = parse_ether(0.003).unwrap();
            let start_weight_wad = parse_ether(0.01).unwrap();
            let end_weight_wad = parse_ether(0.99).unwrap();
            builder.config_builder.portfolio_manager.fee(fee_wad);
            builder
                .config_builder
                .portfolio_manager
                .start_weight_x(start_weight_wad);
            builder
                .config_builder
                .portfolio_manager
                .end_weight_x(end_weight_wad);
            builder
                .config_builder
                .portfolio_manager
                .end_timestamp(14985);

            // Edit the swapper.
            let balance = parse_ether(1.0).unwrap();
            builder.config_builder.swapper.num_swaps(12);
            builder.config_builder.swapper.start_timestamp(15);
            builder.config_builder.swapper.end_timestamp(15000);
            builder.config_builder.swapper.initial_balance(balance);
            builder.config_builder.swapper.swap_direction(false);

            // Finally, after making all the modifications to the config builder,
            // we can add it to the builders.
            self.simulations.push(builder);
        }
    }

    pub fn step(&mut self) -> Command<Self::AppMessage> {
        match self.current.clone() {
            DashboardState::Empty => {
                // todo: figure out what happens here? Should call start before
                // stepping from empty.
            }
            DashboardState::Prepare(state) => {
                self.current = DashboardState::Review(review::ReviewAdjustment::default());
            }
            DashboardState::Review(state) => {
                // Review's package should be filled out, so we can edit the MiniWorldBuilder.
                let package = match &state.package {
                    Some(package) => package,
                    // Exit early, review was not submitted, somehow step got called?
                    None => {
                        tracing::error!("Step called before submitting review form. Bug!");
                        return Command::none();
                    }
                };

                // Store the package for later use.
                self.strategy_parameters = Some(package.clone());

                tracing::info!("package: {:?}", package);

                // Construct the MiniWorldBuilder from the review form data and portfolio.
                self.construct();

                // Edit the MiniWorldBuilder with the package data and form info.
                // todo: use the chosen strategy in the config...
                let builder = self.simulations.last_mut().unwrap();
                builder
                    .config_builder
                    .portfolio_manager
                    .fee(parse_ether(package.fee_percentage).unwrap());

                // todo: implement the start time in the portfolio manager too?
                builder
                    .config_builder
                    .swapper
                    .start_timestamp(package.start_time_seconds.round() as u64);

                builder.config_builder.portfolio_manager.end_timestamp(
                    (package.start_time_seconds + package.duration_seconds).round() as u64,
                );

                let adjustments = match &self.weight_changes {
                    Some(adjustments) => adjustments,
                    None => {
                        tracing::error!("Step called before weight changes were computed. Bug!");
                        return Command::none();
                    }
                };

                let original_x_balance = adjustments
                    .original
                    .positions
                    .iter()
                    .filter(|x| x.asset.symbol == "X")
                    .filter(|x| x.balance.is_some())
                    .map(|x| x.balance.unwrap())
                    .next()
                    .unwrap();

                let start_weight_x = adjustments
                    .original
                    .positions
                    .iter()
                    .filter(|x| x.asset.symbol == "X")
                    .filter(|x| x.weight.is_some())
                    .map(|x| x.weight.unwrap())
                    .next()
                    .unwrap();

                let end_weight_x = adjustments
                    .adjusted
                    .positions
                    .iter()
                    .filter(|x| x.asset.symbol == "X")
                    .filter(|x| x.weight.is_some())
                    .map(|x| x.weight.unwrap())
                    .last()
                    .unwrap();

                builder
                    .config_builder
                    .portfolio_manager
                    .start_weight_x(parse_ether(start_weight_x).unwrap());

                builder
                    .config_builder
                    .portfolio_manager
                    .end_weight_x(parse_ether(end_weight_x).unwrap());

                builder
                    .config_builder
                    .deposit_x(parse_ether(original_x_balance).unwrap());

                // Prepare the simulate screen with the MiniWorldBuilder.
                let screen = simulate::Simulate::new(self.simulations.clone());
                self.current = DashboardState::Simulate(screen);

                // Trigger a callback message to let the app know that we are
                // ready to simulate. This should be caught by the simulate
                // screen and the result should be stored in the screen.

                tracing::info!("Running sim... please wait a few seconds.");
                return Command::perform(
                    portfolio_adjustment::spawn(self.simulations.clone()),
                    simulate::Message::Ready,
                )
                .map(|x| x.into());
            }
            DashboardState::Simulate(_state) => {
                self.current = DashboardState::Execute;
            }
            DashboardState::Execute => {
                self.current = DashboardState::Empty;
            }
        }

        Command::none()
    }
}

impl State for Stages {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        match &self.current {
            DashboardState::Empty => {}
            DashboardState::Review(_state) => {}
            DashboardState::Simulate(_state) => {}
            DashboardState::Execute => {}
            _ => {}
        }

        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::LoadPortfolio(portfolio) => {
                self.portfolio = Some(portfolio);
            }
            Message::Step => return self.step(),
            Message::Route(state) => {
                self.current = state;
            }
            Message::Reset => {
                self.current = DashboardState::Empty;
            }
            Message::Start(deltas) => {
                let prepare = prepare::Prepare::new(self.portfolio.clone().unwrap(), deltas);
                self.weight_changes = Some(prepare.payload.clone());
                self.current = DashboardState::Prepare(prepare);
            }
            // Below is where the complexity is...
            // The `current` state stores the specific screen that the user is on.
            // Its an enum which exposes the actual component within its enum variant arguments.
            // This is mutable and can be updated, so any messages for these components
            // are propagated through this `self.current` state.
            // This avoids us having to make individual state in the Stages struct for each possible
            // stage...
            Message::Review(message) => {
                // Catch the submit form message and route to the next stage.
                // todo: this is a bit hacky... maybe we change how we do this?
                // The outcome of this is that we step + update if its a Submit message on the
                // review's child form component.
                // If its not a submit message, we just do the regular update.
                // todo: write tests!
                let should_step = match &self.current {
                    DashboardState::Review(_) => {
                        matches!(message, review::Message::Form(review::FormMessage::Submit))
                    }
                    _ => false,
                };

                let mut commands = vec![];

                // todo: figure out proper order of operations here...
                // batch executes simultaneously, so what's the effect here?
                if let DashboardState::Review(state) = &mut self.current {
                    commands.push(state.update(message.clone()).map(|x| x.into()));
                }

                if should_step {
                    commands.push(self.step());
                }

                return Command::batch(commands);
            }
            Message::Simulate(message) => {
                let should_step = match &self.current {
                    DashboardState::Simulate(_) => {
                        matches!(message, simulate::Message::Submit)
                    }
                    _ => false,
                };

                let mut commands = vec![];

                if let DashboardState::Simulate(state) = &mut self.current {
                    commands.push(state.update(message.clone()).map(|x| x.into()));
                }

                if should_step {
                    commands.push(self.step());
                }

                return Command::batch(commands);
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        // Storing different stages in this enum allows us to easily switch between them
        // using view() and the MessageWrapper trait.
        let content = match &self.current {
            DashboardState::Empty => Column::new().into(),
            DashboardState::Prepare(state) => state.view().map(|x| x.into()),
            DashboardState::Review(state) => state.view().map(|x| x.into()),
            DashboardState::Simulate(state) => state.view().map(|x| x.into()),
            DashboardState::Execute => Column::new().into(),
        };

        Container::new(
            Row::new()
                .spacing(Sizes::Lg)
                .push(Column::new().push(content).width(Length::FillPortion(3)))
                .push(self.current.guide().width(Length::FillPortion(1))),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
