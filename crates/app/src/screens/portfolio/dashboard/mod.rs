//! Renders a view of the portfolio's positions and strategies.

pub mod stages;
pub mod table;

use std::collections::HashMap;

use profiles::portfolios::Portfolio;
use stages::Stages;

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
    let path = name.map(Portfolio::file_path_with_name);
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
    /// Triggered when a user wants to review the proposed adjustments.
    Prepare,
    /// Triggered action from the main button on the dashboard.
    Submit,
    /// Triggered when the user clicks the review adjustments button.
    Stage(stages::Message),
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

    pub fn render_header(&self) -> Element<'_, Self::AppMessage> {
        Column::new()
            .spacing(Sizes::Md)
            .push(h1("Dashboard".to_string()).size(TitleSize::Xl))
            .push(
                Row::new()
                    .spacing(Sizes::Md)
                    .push(label_item("Positions".to_string()).size(TitleSize::Sm))
                    .push(
                        action_button("Review Adjustments".to_string()).on_press(Message::Prepare),
                    ),
            )
            .into()
    }

    pub fn render_table(&self) -> Element<'_, Self::AppMessage> {
        self.table.view().map(|x| x.into())
    }

    pub fn render_stages(&self) -> Element<'_, Self::AppMessage> {
        match self.stage.current {
            DashboardState::Empty => {
                // Triggers the `Step` message on the stages component.
                let submit = match self.table.prepared() {
                    true => Some(Self::AppMessage::Submit),
                    false => None,
                };

                let instruct: Element<'_, Self::AppMessage> = instructions(
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

                Row::new()
                    .spacing(Sizes::Lg)
                    .push(self.table.summary_table().map(|x| x.into()))
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
            None => Some("Main".to_string()),
        };

        let mut commands = vec![];
        commands.push(Command::perform(load_portfolio(name), Message::Load));

        // todo: does this even work for the children components?
        commands.push(self.stage.load().map(|x| x.into()));

        Command::batch(commands)
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        match message {
            Message::Load(Ok(portfolio)) => {
                self.portfolio = Some(portfolio.clone());

                let mut commands: Vec<Command<Self::AppMessage>> = vec![];

                commands.push(
                    self.stage
                        .update(stages::Message::LoadPortfolio(portfolio.clone()))
                        .map(|x| x.into()),
                );

                commands.push(
                    self.table
                        .update(table::Message::Portfolio(portfolio.clone()))
                        .map(|x| x.into()),
                );

                return Command::batch(commands);
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load portfolio: {:?}", e);
            }
            Message::PortfolioTable(message) => {
                let cmd = self.table.update(message).map(|x| x.into());
                return cmd;
            }
            Message::Stage(stage) => {
                return self.stage.update(stage).map(|x| x.into());
            }
            // Renders the summary of adjustments table.
            Message::Prepare => {
                return self.table.update(table::Message::Ready).map(|x| x.into());
            }
            // Triggers the staging process...
            Message::Submit => {
                tracing::trace!("Reviewing...");

                return self.stage.update(stages::Message::Step).map(|x| x.into());
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Lg);
        content = content.push(self.render_header().map(|x| x.into()));
        content = content.push(self.render_table().map(|x| x.into()));
        content = content.push(self.render_stages().map(|x| x.into()));

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
