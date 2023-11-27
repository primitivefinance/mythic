//! Handles the different stages for interacting with a portfolio, from just
//! viewing it to executing adjustments.

pub mod execute;
pub mod review;
pub mod simulate;

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

#[derive(Debug, Clone, Default)]
pub struct Stages {
    pub current: DashboardState,
}

impl Stages {
    pub type AppMessage = Message;

    pub fn new() -> Self {
        Self {
            current: DashboardState::Empty,
        }
    }

    pub fn step(&mut self) -> Command<Self::AppMessage> {
        match &self.current {
            DashboardState::Empty => {
                self.current = DashboardState::Review(review::ReviewAdjustment::default());
            }
            DashboardState::Review(state) => {
                self.current = DashboardState::Simulate(simulate::Simulate::default());
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

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
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

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
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
