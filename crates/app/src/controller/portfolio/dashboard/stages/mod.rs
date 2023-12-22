//! Handles the different stages for interacting with a portfolio, from just
//! viewing it to executing adjustments.

pub mod execute;
pub mod prepare;
pub mod review;
pub mod simulate;

use sim::engine::ArbiterInstanceManager;

use super::{table::PositionDelta, *};

/// Stores the actual state of the stage in the enum variant argument.
/// Weird? It works.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum DashboardState {
    Empty,
    /// State of reviewing and finalizing the adjustments to make.
    Prepare,
    /// State of reviewing the portfolio adjustment transaction.
    Review,
    /// State of simulating the portfolio adjustment transaction.
    Simulate,
    /// State of executing the portfolio adjustment transaction.
    /// FIXME: temp initial state while we test.
    #[default]
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
    /// Loads the original portfolio.
    Load(Portfolio),
    /// Sets the adjusted portfolio.
    SetAdjusted(Option<Portfolio>),
    /// Resets the staging to the first step.
    Reset,
    /// Steps the stage forward.
    Step,
    /// Routes to a target stage.
    Route(DashboardState),
    /// Message for Empty -> Prepare stage.
    /// Needs the position index and adjustments, and the estimated price.
    Start(Vec<PositionDelta>),
    /// Prepares the adjustments for simulation.
    Prepare(prepare::Message),
    /// Updates the review stage.
    Review(review::Message),
    /// Updates the simulate stage.
    Simulate(simulate::Message),
    /// Updates the execute stage.
    Execute(execute::Message),
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::UpdateStaging(msg)
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
    /// Original portfolio loaded from file.
    pub original: Option<Portfolio>,
    pub adjusted: Option<Portfolio>,
    pub current: DashboardState,
    pub prepare: prepare::Prepare,
    pub review: review::Review,
    pub simulate: simulate::Simulate,
    pub execute: execute::Execute,
}

impl Stages {
    pub fn new(dev_client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>) -> Self {
        Self {
            original: None,
            adjusted: None,
            current: DashboardState::default(),
            prepare: prepare::Prepare::default(),
            review: review::Review::default(),
            simulate: simulate::Simulate::default(),
            execute: execute::Execute::new(dev_client, None),
        }
    }

    pub fn guide(&self) -> Container<'static, Message> {
        match self.current {
            DashboardState::Empty => Container::new(Column::new()),
            DashboardState::Prepare => self.prepare.guide(Some(Message::Step)),
            DashboardState::Review => self.review.guide(),
            DashboardState::Simulate => self.simulate.guide(Some(Message::Step)),
            DashboardState::Execute => self.execute.guide(),
        }
    }

    pub fn step(&mut self) -> Command<Message> {
        match self.current.clone() {
            DashboardState::Empty => {
                // todo: figure out what happens here?
                Command::perform(async {}, |_| Message::Route(DashboardState::Prepare))
            }
            DashboardState::Prepare => {
                // Route to the review stage.
                Command::perform(async {}, |_| Message::Route(DashboardState::Review))
            }
            DashboardState::Review => {
                // Route to the simulate stage.
                Command::perform(async {}, |_| Message::Route(DashboardState::Simulate))
            }
            DashboardState::Simulate => {
                // Route to the execute stage.
                Command::perform(async {}, |_| Message::Route(DashboardState::Execute))
            }
            DashboardState::Execute => {
                // Route back to the empty page.
                Command::perform(async {}, |_| Message::Route(DashboardState::Empty))
            }
        }
    }

    pub fn arm_simulation(&mut self) -> Command<Message> {
        if self.original.is_none() {
            tracing::error!("Original portfolio is None. Bug!");
            return Command::none();
        }

        let builder = ArbiterInstanceManager::new();

        // // Compute the amount of steps given the time step size of 15 and the time
        // range provided by the user.
        // todo: this is temp value, but do above
        // let steps: usize = 1000;
        // Initial amount to deposit.
        // let deposit = parse_ether(0.01).unwrap();
        // initial price.
        // let price = parse_ether(1.0).unwrap();
        // Time between blocks, in seconds.
        // let timestep_size: u64 = 15;
        //
        // First add the coins. Make sure they are in the portfolio.
        // let coin_x: TokenData = portfolio
        // .positions
        // .0
        // .iter()
        // .find(|x| x.asset.symbol == "X")
        // .expect("no X coin found")
        // .clone()
        // .into();
        // let coin_y: TokenData = portfolio
        // .positions
        // .0
        // .iter()
        // .find(|x| x.asset.symbol == "Y")
        // .expect("no Y coin found")
        // .clone()
        // .into();
        // builder.config_builder.coins(coin_x, coin_y);
        //
        // Edit the lp agent.
        // builder.config_builder.deposit_x(deposit);
        // builder.config_builder.initial_price(price);
        //
        // Edit the seconds between blocks.
        // builder.config_builder.timestep_size(timestep_size);
        //
        // Edit the price and amount of steps.
        // todo: matching the dca/static.toml config right now, change later
        // builder.config_builder.price_changer.seed(1);
        // builder.config_builder.price_changer.num_steps(steps);
        // builder.config_builder.price_changer.num_paths(10);
        // builder.config_builder.price_changer.initial_price(price);
        // builder.config_builder.price_changer.t_0(0.0);
        // builder.config_builder.price_changer.t_n(0.1);
        // builder.config_builder.price_changer.drift(0.1);
        // builder.config_builder.price_changer.volatility(0.35);
        //
        // Edit the portfolio manager.
        // let fee_wad = parse_ether(0.003).unwrap();
        // let start_weight_wad = parse_ether(0.01).unwrap();
        // let end_weight_wad = parse_ether(0.99).unwrap();
        // builder.config_builder.portfolio_manager.fee(fee_wad);
        // builder
        // .config_builder
        // .portfolio_manager
        // .start_weight_x(start_weight_wad);
        // builder
        // .config_builder
        // .portfolio_manager
        // .end_weight_x(end_weight_wad);
        // builder
        // .config_builder
        // .portfolio_manager
        // .end_timestamp(14985);
        //
        // Edit the swapper.
        // let balance = parse_ether(1.0).unwrap();
        // builder.config_builder.swapper.num_swaps(12);
        // builder.config_builder.swapper.start_timestamp(15);
        // builder.config_builder.swapper.end_timestamp(15000);
        // builder.config_builder.swapper.initial_balance(balance);
        // builder.config_builder.swapper.swap_direction(false);
        //
        // if self.review.sealed.is_none() {
        // tracing::error!("Review form was not submitted!");
        // return Command::none();
        // }
        //
        // Edit the portfolio manager by applying the review form data provided by the
        // user.
        // let parameters: StrategyParameters = self.review.sealed.clone().unwrap();
        // builder
        // .config_builder
        // .portfolio_manager
        // .fee(parse_ether(parameters.fee_percentage).unwrap());
        //
        // todo: implement the start time in the portfolio manager too?
        // builder
        // .config_builder
        // .swapper
        // .start_timestamp(parameters.start_time_seconds.round() as u64);
        //
        // builder.config_builder.portfolio_manager.end_timestamp(
        // (parameters.start_time_seconds + parameters.duration_seconds).round() as u64,
        // );
        //
        // Finally, apply the weight changes to the portfolio manager.
        // if self.adjusted.is_none() {
        // tracing::error!("Weight changes were not computed!");
        // return Command::none();
        // }
        //
        // let adjusted = self.adjusted.clone().unwrap();
        //
        // let start_weight_x = portfolio
        // .positions
        // .0
        // .iter()
        // .filter(|x| x.asset.symbol == "X")
        // .filter(|x| x.weight.is_some())
        // .map(|x| x.weight.unwrap())
        // .next()
        // .unwrap();
        // builder
        // .config_builder
        // .portfolio_manager
        // .start_weight_x(parse_ether(start_weight_x).unwrap());
        //
        // let end_weight_x = adjusted
        // .positions
        // .0
        // .iter()
        // .filter(|x| x.asset.symbol == "X")
        // .filter(|x| x.weight.is_some())
        // .map(|x| x.weight.unwrap())
        // .last()
        // .unwrap();
        // builder
        // .config_builder
        // .portfolio_manager
        // .end_weight_x(parse_ether(end_weight_x).unwrap());
        //
        // let original_x_balance = portfolio
        // .positions
        // .0
        // .iter()
        // .filter(|x| x.asset.symbol == "X")
        // .filter(|x| x.balance.is_some())
        // .map(|x| x.balance.unwrap())
        // .next()
        // .unwrap();
        // builder
        // .config_builder
        // .deposit_x(parse_ether(original_x_balance).unwrap());

        Command::perform(async {}, |_| {
            Message::Simulate(simulate::Message::Armed(builder))
        })
    }
}

impl State for Stages {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Message> {
        Command::perform(async {}, |_| Message::Start(vec![]))
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Load(portfolio) => {
                self.original = Some(portfolio.clone());
                self.prepare = prepare::Prepare::new(portfolio.clone());
                self.execute.original = Some(portfolio.clone());
            }
            Message::SetAdjusted(portfolio) => {
                // Sets the adjusted portfolio.
                self.adjusted = portfolio.clone();

                let mut commands = vec![];

                // If we are setting Some portfolio and we are on the empty page, route to the
                // prepare page.
                if portfolio.is_some() && matches!(self.current, DashboardState::Empty) {
                    commands.push(Command::perform(async {}, |_| {
                        Message::Route(DashboardState::Prepare)
                    }));
                }

                // Propagates changes to the prepare stage since it renders the deltas.
                commands.push(
                    self.prepare
                        .update(prepare::Message::SetAdjusted(portfolio))
                        .map(|x| x.into()),
                );

                return Command::batch(commands);
            }
            Message::Reset => {
                self.current = DashboardState::Empty;
            }
            Message::Step => return self.step(),
            Message::Route(state) => {
                self.current = match state {
                    DashboardState::Empty => DashboardState::Empty,
                    DashboardState::Prepare => DashboardState::Prepare,
                    DashboardState::Review => DashboardState::Review,
                    DashboardState::Simulate => DashboardState::Simulate,
                    DashboardState::Execute => DashboardState::Execute,
                };
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
                    DashboardState::Review => {
                        matches!(message, review::Message::Form(review::FormMessage::Submit))
                    }
                    _ => false,
                };

                let mut commands = vec![];

                // todo: figure out proper order of operations here...
                // batch executes simultaneously, so whats the effect here?
                if let DashboardState::Review = &mut self.current {
                    commands.push(self.review.update(message.clone()).map(|x| x.into()));
                }

                if should_step {
                    commands.push(self.step());
                }

                return Command::batch(commands);
            }
            Message::Simulate(message) => {
                let should_arm = match &self.current {
                    DashboardState::Simulate => {
                        matches!(message, simulate::Message::Arm)
                    }
                    _ => false,
                };

                let mut commands = vec![];

                if should_arm {
                    commands.push(self.arm_simulation());
                }

                if let DashboardState::Simulate = &mut self.current {
                    commands.push(self.simulate.update(message.clone()).map(|x| x.into()));
                }

                return Command::batch(commands);
            }
            Message::Execute(message) => {
                let _should_execute = match &self.current {
                    DashboardState::Execute => {
                        matches!(message, execute::Message::Execute)
                    }
                    _ => false,
                };

                let mut commands = vec![];

                if let DashboardState::Execute = &mut self.current {
                    commands.push(self.execute.update(message.clone()).map(|x| x.into()));
                }

                return Command::batch(commands);
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let _routes = Row::new()
            .spacing(Sizes::Sm)
            .push(
                tab_button(
                    self.current == DashboardState::Prepare,
                    "Adjustments".to_string(),
                )
                .on_press(Message::Route(DashboardState::Prepare)),
            )
            .push(
                tab_button(self.current == DashboardState::Review, "Review".to_string())
                    .on_press(Message::Route(DashboardState::Review)),
            )
            .push(
                tab_button(
                    self.current == DashboardState::Simulate,
                    "Simulate".to_string(),
                )
                .on_press(Message::Route(DashboardState::Simulate)),
            )
            .push(
                tab_button(
                    self.current == DashboardState::Execute,
                    "Execute".to_string(),
                )
                .on_press(Message::Route(DashboardState::Execute)),
            );

        // Storing different stages in this enum allows us to easily switch between them
        // using view() and the MessageWrapper trait.
        let content = match &self.current {
            DashboardState::Empty => Column::new().into(),
            DashboardState::Prepare => self.prepare.view().map(|x| x.into()),
            DashboardState::Review => self.review.view().map(|x| x.into()),
            DashboardState::Simulate => self.simulate.view().map(|x| x.into()),
            DashboardState::Execute => self.execute.view().map(|x| x.into()),
        };

        // let mut nav = Column::new().spacing(Sizes::Md);
        //
        // nav = nav.push(routes).height(Length::FillPortion(1));
        // nav = nav
        // .push(Column::new().push(content))
        // .height(Length::FillPortion(3));
        // nav = nav
        // .push(self.guide().align_y(alignment::Vertical::Bottom))
        // .height(Length::FillPortion(1));

        Container::new(content).into()
    }
}
