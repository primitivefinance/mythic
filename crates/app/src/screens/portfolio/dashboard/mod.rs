//! Renders a view of the portfolio's positions and strategies.

pub mod review;
pub mod simulate;
pub mod table;

use std::collections::HashMap;

use profiles::portfolios::{Portfolio, Targetable};

use self::{review::ReviewAdjustment, simulate::Simulate, table::PortfolioTable};
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
    let path = match name {
        Some(name) => Some(Portfolio::file_path_with_name(name)),
        None => None,
    };
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
    /// Triggered action from the main button on the dashboard.
    Submit,
    /// Triggered when the user clicks the review adjustments button.
    ReviewAdjustment,
    /// Triggered after the first submit to transition review adjustments.
    Review(review::Message),
    /// Triggered after the user submits adjustments for simulation.
    Simulated(simulate::Message),
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
    portfolio: Option<Portfolio>,
    table: PortfolioTable,
    ready: bool,
    state: DashboardState,
    loaded_from: Option<String>,
}

impl From<DashboardWrapper> for Screen {
    fn from(dashboard: DashboardWrapper) -> Self {
        Screen::new(Box::new(dashboard))
    }
}

#[derive(Debug, Clone, Default)]
pub enum DashboardState {
    #[default]
    Empty,
    /// State of reviewing the portfolio's edited fields.
    Propose,
    /// State of reviewing the portfolio adjustment transaction.
    Review(ReviewAdjustment),
    /// State of simulating the portfolio adjustment transaction.
    Simulate(Simulate),
    /// State of executing the portfolio adjustment transaction.
    Execute,
}

impl DashboardState {
    pub fn clear(&mut self) {
        *self = DashboardState::Empty;
    }

    pub fn ready(&self) -> bool {
        match self {
            DashboardState::Propose => true,
            _ => false,
        }
    }
}

impl Dashboard {
    pub type AppMessage = Message;
    pub type ViewMessage = view::Message;

    pub fn new(name: Option<String>) -> Self {
        // Try loading the portfolio from the name.

        Self {
            portfolio: None,
            table: PortfolioTable::new(),
            ready: false,
            state: DashboardState::default(),
            loaded_from: name,
        }
    }

    pub fn loaded(&self) -> bool {
        self.portfolio.is_some()
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

        Command::perform(load_portfolio(name), |x| Message::Load(x).into())
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                self.portfolio = Some(portfolio.clone());
                return self
                    .table
                    .update(table::Message::Portfolio(portfolio.clone()))
                    .map(|x| x.into());
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load portfolio: {:?}", e);
            }
            Message::PortfolioTable(message) => {
                let cmd = self.table.update(message).map(|x| x.into());
                return cmd;
            }
            Message::Submit => {
                tracing::trace!("Reviewing...");

                self.state = DashboardState::Review(ReviewAdjustment::default());
            }
            Message::ReviewAdjustment => {
                // Make it ready to review.
                self.ready = true;
                return self.table.update(table::Message::Ready).map(|x| x.into());
            }
            Message::Review(review::Message::Form(review::FormMessage::Submit)) => {
                tracing::trace!("Simulating...");

                let cmd = match &mut self.state {
                    DashboardState::Review(review) => review
                        .update(review::Message::Form(review::FormMessage::Submit))
                        .map(|x| x.into()),
                    _ => Command::none(),
                };

                self.state = DashboardState::Simulate(Simulate::default());

                return cmd;
            }
            Message::Review(message) => {
                let cmd = match &mut self.state {
                    DashboardState::Review(review) => review.update(message).map(|x| x.into()),
                    _ => Command::none(),
                };

                return cmd;
            }
            Message::Simulated(simulate::Message::Submit) => {
                tracing::trace!("Executing adjustment...");
                self.state.clear();
                self.ready = false;
                return self.table.update(table::Message::Clear).map(|x| x.into());
            }
            _ => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let table: Element<'a, Message> = self.table.view().map(|x| x.into());

        let submit = match self.ready {
            true => Some(Message::Submit),
            false => None,
        };

        let instruct: Element<'a, Message> = instructions(
            vec![
                instruction_text("Edit the deltas for each position.".to_string()),
                instruction_text(
                    "Deltas are used to calculate the portfolio's metrics.".to_string(),
                ),
            ],
            Some("Edit Deltas".to_string()),
            None,
            submit,
        )
        .into();

        let mut sub_section = Row::new().spacing(Sizes::Lg);

        sub_section = sub_section.push(self.table.summary_table().map(|x| {
            view::Message::Developer(developer::Message::Dash(Message::PortfolioTable(x)))
        }));

        sub_section = sub_section.push(
            Column::new()
                .align_items(alignment::Alignment::End)
                .push(instruct.map(|x| view::Message::Developer(developer::Message::Dash(x))))
                .width(Length::FillPortion(1)),
        );

        let summarize: Element<'a, Message> = action_button("Review Adjustments".to_string())
            .on_press(Message::ReviewAdjustment)
            .into();

        let mut content = Column::new()
            .spacing(20)
            .push(h1("Dashboard".to_string()).size(TitleSize::Xl))
            .push(
                Row::new()
                    .push(label_item("Positions".to_string()).size(TitleSize::Sm))
                    .push(summarize.map(|x| view::Message::Developer(developer::Message::Dash(x)))),
            )
            .push(table.map(|x| view::Message::Developer(developer::Message::Dash(x))))
            .push(sub_section);

        match &self.state {
            DashboardState::Review(review) => {
                content = content.push(review.view().map(|x| {
                    view::Message::Developer(developer::Message::Dash(Message::Review(x)))
                }));
            }
            DashboardState::Simulate(simulate) => {
                content = content.push(simulate.view().map(|x| {
                    view::Message::Developer(developer::Message::Dash(Message::Simulated(x)))
                }));
            }
            _ => {}
        }

        Container::new(content)
            .align_y(alignment::Vertical::Top)
            .center_x()
            .max_height(ByteScale::Xl7)
            .max_width(ByteScale::Xl7.between(&ByteScale::Xl8))
            .padding(Sizes::Xl)
            .into()
    }
}

/// Wrapper around the dashboard so it can be used directly from the root app.
pub struct DashboardWrapper {
    pub dashboard: Dashboard,
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
        return cmd.map(|x| x.into());
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

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        self.dashboard.view().map(|x| x.into())
    }
}
