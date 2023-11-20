//! Cached state of user inputs and derived values for
//! using the Execution app.

use iced_aw::Icon;

use super::{processing::StorageDiffs, utils::empty_async, *};
use crate::mvp::view::execute::{steps_group, submit_group};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransactionSteps {
    #[default]
    Start,
    Simulated,
    Executed,
    Confirmed,
}

impl TransactionSteps {
    pub fn next(&self) -> Self {
        match self {
            Self::Start => Self::Simulated,
            Self::Simulated => Self::Executed,
            Self::Executed => Self::Confirmed,
            Self::Confirmed => Self::Start,
        }
    }

    pub fn get_cta(&self) -> String {
        match self {
            TransactionSteps::Start => "Review".to_string(),
            TransactionSteps::Simulated => "Execute".to_string(),
            TransactionSteps::Executed => "Confirming...".to_string(),
            TransactionSteps::Confirmed => "Exit".to_string(),
        }
    }

    pub fn get_instructions(&self) -> String {
        match self {
            TransactionSteps::Start => "Construct a transaction and then review it.".to_string(),
            TransactionSteps::Simulated => "Review simulated results then execute.".to_string(),
            TransactionSteps::Executed => "Wait for transaction to confirm.".to_string(),
            TransactionSteps::Confirmed => "Transaction confirmed. Exit to restart.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Loading {
    #[default]
    Empty,
    Simulation,
    Execution,
}

impl Loading {
    pub fn in_progress(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Simulation => true,
            Self::Execution => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Progress {
    // Current step to display.
    pub current: TransactionSteps,
    // Highest step reached.
    pub checkpoint: TransactionSteps,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            current: TransactionSteps::Start,
            checkpoint: TransactionSteps::Start,
        }
    }

    /// Checks if we are currently at the checkpoint for deciding to progress or
    /// not.
    pub fn validate(&self) -> bool {
        self.current == self.checkpoint
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Fields {
    pub to: Option<String>,
    pub from: Option<String>,
    pub target: Option<String>,
    pub amount: Option<String>,
}

impl Fields {
    pub fn new() -> Self {
        Self {
            to: None,
            from: None,
            target: None,
            amount: None,
        }
    }

    /// todo: leverage this validate more?
    pub fn validate(&self) -> bool {
        !self.to.is_none()
            && !self.from.is_none()
            && !self.target.is_none()
            && !self.amount.is_none()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Form {
    pub fields: Fields,
    pub progress: Progress,
    pub loading: Loading,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FormMessage {
    ChangeTo(String),
    ChangeFrom(String),
    ChangeTarget(String),
    ChangeAmount(Option<String>),
    ChangeFeedback(String),
    ChangeLoading(Loading),
    ChangeCheckpoint(TransactionSteps),
    ChangeCurrentStep(TransactionSteps),
    RouteToStep(TransactionSteps),
    Simulate,
    CompleteSimulate,
    Execute,
    CompleteExecute,
    Confirm,
    Continue,
    Reset,
}

impl Form {
    pub fn new() -> Self {
        Self {
            fields: Fields::default(),
            progress: Progress::default(),
            loading: Loading::Empty,
            feedback: None,
        }
    }

    pub fn view<'a>(
        &self,
        from_contacts: &ContactList,
        to_contacts: &ContactList,
        target_contacts: &ContactList,
        sim_results: Option<StorageDiffs>,
        exec_results: Option<StorageDiffs>,
    ) -> Element<'a, view::Message> {
        let content = match self.progress.current.clone() {
            TransactionSteps::Start => view::execute::starting(
                from_contacts,
                self.fields.from.clone(),
                to_contacts,
                self.fields.to.clone(),
                target_contacts,
                self.fields.target.clone(),
                self.fields.amount.clone(),
            ),
            TransactionSteps::Simulated => view::execute::simulated(
                self.fields.to.clone(),
                self.fields.target.clone(),
                self.fields.amount.clone(),
                sim_results,
            ),
            TransactionSteps::Executed => view::execute::executed(
                self.fields.to.clone(),
                self.fields.target.clone(),
                self.fields.amount.clone(),
            ),
            TransactionSteps::Confirmed => view::execute::confirmed(
                self.fields.to.clone(),
                self.fields.target.clone(),
                self.fields.amount.clone(),
                exec_results,
                1,
            ),
        };

        let steps = self.get_actions();
        let action = self.on_submit();

        let steps_card = steps_group(steps);
        let submit_card = submit_group(action, self.feedback.clone(), self.progress.checkpoint);

        let column_1: Vec<Element<'a, view::Message>> = content;
        let column_2: Vec<Element<'a, view::Message>> = vec![steps_card.into(), submit_card.into()];

        components::dual_column(column_1, column_2).into()
    }

    // Fetches the appropriate submit message, does not submit.
    pub fn on_submit(&self) -> view::Message {
        match self.progress.checkpoint {
            TransactionSteps::Start if self.fields.validate() && self.progress.validate() => {
                view::Execution::Simulate.into()
            }
            TransactionSteps::Simulated
                if self.progress.validate() && !self.loading.in_progress() =>
            {
                view::Execution::Execute.into()
            }
            TransactionSteps::Executed
                if self.progress.validate() && !self.loading.in_progress() =>
            {
                view::Execution::Results.into()
            }
            TransactionSteps::Confirmed
                if self.progress.validate() && !self.loading.in_progress() =>
            {
                view::Execution::Reset.into()
            }
            _ => view::Message::Empty,
        }
    }

    pub fn reset(&mut self) {
        self.fields = Fields::default();
        self.progress = Progress::default();
        self.loading = Loading::Empty;
        self.feedback = None;
    }

    pub fn update(&mut self, message: FormMessage) -> Command<app::Message> {
        match message {
            FormMessage::ChangeTo(to) => {
                self.fields.to = Some(to);
                Command::none()
            }
            FormMessage::ChangeFrom(from) => {
                self.fields.from = Some(from);
                Command::none()
            }
            FormMessage::ChangeTarget(target) => {
                self.fields.target = Some(target);
                Command::none()
            }
            FormMessage::ChangeAmount(amount) => {
                self.fields.amount = amount;
                Command::none()
            }
            FormMessage::ChangeFeedback(feedback) => {
                self.feedback = Some(feedback);
                Command::none()
            }
            FormMessage::ChangeLoading(loading) => {
                self.loading = loading;
                Command::none()
            }
            FormMessage::ChangeCheckpoint(checkpoint) => {
                self.progress.checkpoint = checkpoint;
                Command::none()
            }
            FormMessage::ChangeCurrentStep(current) => {
                self.progress.current = current;
                Command::none()
            }
            FormMessage::RouteToStep(step) => {
                // Only route if the step has been reached.
                if step <= self.progress.checkpoint {
                    self.progress.current = step;
                }

                Command::none()
            }
            FormMessage::Continue => {
                // Progress to the next step.
                let next_step = self.progress.current.next();

                // If the next step is start, emit the Reset event.
                if next_step == TransactionSteps::Start {
                    return Command::perform(empty_async(), |_| view::Execution::Reset.into());
                }

                // If we get past our checkpoint, update it.
                if next_step > self.progress.checkpoint {
                    self.progress.checkpoint = next_step;
                }

                return Command::perform(empty_async(), move |_| {
                    view::Execution::Form(FormMessage::RouteToStep(next_step)).into()
                });
            }
            FormMessage::Simulate => {
                // Return early if our checkpoint is past this step.
                // todo: this validates simulate from updating, but how do validate something in
                // the screen from updating based on this?
                if self.progress.checkpoint > TransactionSteps::Simulated {
                    return Command::none();
                }

                self.loading = Loading::Simulation;
                self.feedback = Some("Simulation in progress...".to_string());

                Command::none()
            }
            FormMessage::Execute => {
                if self.progress.checkpoint > TransactionSteps::Executed {
                    return Command::none();
                }

                self.loading = Loading::Execution;
                self.feedback = Some("Sending transaction...".to_string());

                // Continue
                Command::perform(empty_async(), move |_| {
                    view::Execution::Form(FormMessage::Continue).into()
                })
            }
            FormMessage::CompleteSimulate => {
                self.loading = Loading::Empty;
                self.feedback = Some("Simulation complete!".to_string());

                // Continue
                Command::perform(empty_async(), move |_| {
                    view::Execution::Form(FormMessage::Continue).into()
                })
            }
            FormMessage::CompleteExecute => {
                self.loading = Loading::Empty;
                self.feedback = Some("Transaction sent!".to_string());

                // Continue
                Command::perform(empty_async(), move |_| {
                    view::Execution::Form(FormMessage::Continue).into()
                })
            }
            FormMessage::Confirm => {
                self.progress.current = TransactionSteps::Confirmed;
                Command::none()
            }
            FormMessage::Reset => {
                self.reset();
                Command::none()
            }
        }
    }
}

/// Gets a method to return a set of actions that can be rendered
/// in a simple button list.
pub trait ActionsList {
    type Components;

    fn get_actions(&self) -> Vec<Self::Components>;
}

impl ActionsList for Form {
    type Components = (Icon, String, view::Message, bool);

    fn get_actions(&self) -> Vec<Self::Components> {
        let mut actions: Vec<Self::Components> = vec![];
        let checkpoint = self.progress.checkpoint;
        let current = self.progress.current;

        actions.push((
            Icon::PencilSquare,
            "Build".to_string(),
            if checkpoint >= TransactionSteps::Start {
                FormMessage::RouteToStep(TransactionSteps::Start).into()
            } else {
                view::Message::Empty
            },
            current == TransactionSteps::Start,
        ));

        actions.push((
            Icon::Sim,
            "Simulate".to_string(),
            if checkpoint >= TransactionSteps::Simulated {
                FormMessage::RouteToStep(TransactionSteps::Simulated).into()
            } else {
                view::Message::Empty
            },
            current == TransactionSteps::Simulated,
        ));
        actions.push((
            Icon::CursorFill,
            "Execute".to_string(),
            if checkpoint >= TransactionSteps::Executed {
                FormMessage::RouteToStep(TransactionSteps::Executed).into()
            } else {
                view::Message::Empty
            },
            current == TransactionSteps::Executed,
        ));
        actions.push((
            Icon::CheckCircleFill,
            "Confirm".to_string(),
            if checkpoint >= TransactionSteps::Confirmed {
                FormMessage::RouteToStep(TransactionSteps::Confirmed).into()
            } else {
                view::Message::Empty
            },
            current == TransactionSteps::Confirmed,
        ));

        actions
    }
}

impl From<FormMessage> for view::Message {
    fn from(message: FormMessage) -> Self {
        view::Message::Execution(view::Execution::Form(message))
    }
}
