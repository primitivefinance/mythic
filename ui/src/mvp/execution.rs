use iced::{
    widget::{Column, Text},
    Command, Element, Subscription,
};

use super::{app::Message, state::State, view};

#[derive(Default)]
pub struct CraftingTransaction {
    pub to: String,
    pub amount: String,
}

#[derive(Default, Clone)]
pub enum TransactionSteps {
    #[default]
    Start,
    Review,
    Simulated,
    Confirm,
}

pub struct Execution {
    transaction: CraftingTransaction,
    step: TransactionSteps,
}

impl Execution {
    pub fn new() -> Self {
        Self {
            transaction: CraftingTransaction::default(),
            step: TransactionSteps::default(),
        }
    }
}

impl State for Execution {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Empty => Command::none(),
            Message::View(msg) => {
                match msg {
                    view::Message::Execution(e) => match e {
                        view::Execution::Next => {
                            self.step = match self.step {
                                TransactionSteps::Start => TransactionSteps::Review,
                                TransactionSteps::Review => TransactionSteps::Simulated,
                                TransactionSteps::Simulated => TransactionSteps::Confirm,
                                TransactionSteps::Confirm => TransactionSteps::Start,
                            };
                        }
                        _ => {}
                    },
                    _ => {}
                }

                Command::none()
            }
            Message::Simulation(_) => Command::none(),
            Message::Data(_) => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        view::app_layout(
            &view::Page::Execute,
            view::execution_view(self.step.clone()),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn load(&self) -> Command<Message> {
        Command::none()
    }
}
