//! Handles the different stages for interacting with a portfolio, from just
//! viewing it to executing adjustments.

pub mod execute;
pub mod review;
pub mod simulate;

use clients::arbiter::portfolio_adjustment::MiniWorldBuilder;
use ethers::utils::parse_ether;
use simulation::agents::token_admin::TokenData;

use super::*;

/// Stores the actual state of the stage in the enum variant argument.
/// Weird? It works.
#[derive(Debug, Clone, Default)]
pub enum DashboardState {
    #[default]
    Empty,
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
}

impl Stages {
    pub type AppMessage = Message;

    pub fn new() -> Self {
        Self {
            portfolio: None,
            current: DashboardState::Empty,
            simulations: vec![],
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
        match &self.current {
            DashboardState::Empty => {
                self.current = DashboardState::Review(review::ReviewAdjustment::default());
            }
            DashboardState::Review(state) => {
                // Construct the MiniWorldBuilder from the review form data and portfolio.
                self.construct();
                // Prepare the simulate screen with the MiniWorldBuilder.
                let screen = simulate::Simulate::new(self.simulations.clone());
                self.current = DashboardState::Simulate(screen);

                // Trigger a callback message to let the app know that we are
                // ready to simulate. This should be caught by the simulate
                // screen and the result should be stored in the screen.

                return Command::perform(
                    portfolio_adjustment::spawn(self.simulations.clone()),
                    simulate::Message::Ready,
                )
                .map(|x| x.into());
            }
            DashboardState::Simulate(state) => {
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
            DashboardState::Review(state) => {}
            DashboardState::Simulate(state) => {}
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
                // batch executes simultaneously, so whats the effect here?
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
        match &self.current {
            DashboardState::Empty => Column::new().into(),
            DashboardState::Review(state) => state.view().map(|x| x.into()),
            DashboardState::Simulate(state) => state.view().map(|x| x.into()),
            DashboardState::Execute => Column::new().into(),
        }
    }
}
