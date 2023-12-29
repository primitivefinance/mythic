//! Portfolio table which renders all positions and a form for editing deltas to
//! adjust the portfolio's positions.

use datatypes::portfolio::{position::Position, Portfolio};

use super::*;
use crate::components::system::{label, ExcaliburContainer};

pub mod form;
use form::*;

#[derive(Debug, Clone, Default)]
pub struct PortfolioTable {
    pub form: DeltaForm,
    pub original: Portfolio,
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Updates the underlying portfolio that is being rendered.
    Load(Portfolio),
    /// Updates the form within the table.
    DeltaForm(form::DeltaFormMessage),
    /// Clears the summary table and form.
    Clear,
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::UpdateTable(msg)
    }
}

/// Individual deltas of a given position.
/// todo: support price and market value deltas...
#[derive(Debug, Clone, Default)]
pub struct PositionDelta {
    pub id: usize,
    pub balance: Option<String>,
    pub weight: Option<String>,
    pub price: Option<String>,
}

impl PositionDelta {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.balance.is_none() && self.weight.is_none() && self.price.is_none()
    }
}

/// Closure for handling the form events.
type FormEvent = Box<dyn Fn(Option<String>) -> Message>;

impl PortfolioTable {
    pub fn new() -> Self {
        Self {
            form: DeltaForm::default(),
            original: Portfolio::default(),
        }
    }

    /// Gets the edited form fields as deltas that can be rendered by the
    /// summary table.
    /// todo: support more targets, price, and market value fields.
    #[allow(dead_code)]
    pub fn get_form_deltas(&self) -> Vec<PositionDelta> {
        self.original
            .positions
            .0
            .iter()
            .enumerate()
            .map(|(pos_index, _)| {
                // todo: support more fields to be changed!
                // let balance = self.form.balance.get(&pos_index).cloned();
                // let price = self.form.price.get(&pos_index).cloned();
                let weight = self.form.weight.get(&pos_index).cloned();
                PositionDelta {
                    id: pos_index,
                    balance: None,
                    weight,
                    price: None,
                }
            })
            .collect()
    }

    /// Renders the dual cell column for a target value and its delta input
    /// field.
    /// - pos_index is the index of the position in the portfolio's positions.
    /// - target is the target value to be displayed in the first cell.
    /// Why this I make this closure stuff so complicated?
    pub fn target_cell(&self, pos_index: usize, target: f64) -> Vec<CellBuilder<Message>> {
        // todo: support more targets
        let (value, on_change_msg) = (
            self.form.weight.get(&pos_index).cloned(),
            Box::new(move |x| form::DeltaFormMessage::Weight(pos_index, x).into()) as FormEvent,
        );

        vec![
            CellBuilder::new().value(Some(target.clone().to_string())),
            CellBuilder::new().value(value).on_change(on_change_msg),
        ]
    }

    /// Gets the cell builders for each target in the position.
    pub fn cell_builders_targets<'a>(
        &'a self,
        position: &'a Position,
        pos_index: usize,
    ) -> Vec<CellBuilder<Message>> {
        self.target_cell(pos_index, position.weight.unwrap_or_default().into())
    }

    /// Gets the builders for each cell in the table.
    pub fn cell_builders<'a>(
        &'a self,
        position: &'a Position,
        pos_index: usize,
    ) -> Vec<CellBuilder<Message>> {
        let price_current = position.cost.unwrap_or_default();
        let balance_current = position.balance.unwrap_or_default();
        let market_value_current = price_current * balance_current;

        let price_adjusted = self
            .form
            .price
            .get(&pos_index)
            .map(|x| x.parse::<f64>().unwrap_or_default())
            .unwrap_or_default();

        let balance_adjusted = self
            .form
            .balance
            .get(&pos_index)
            .map(|x| x.parse::<f64>().unwrap_or_default())
            .unwrap_or_default();

        let market_value_adjusted = self
            .form
            .market_value
            .get(&pos_index)
            .map(|x| x.parse::<f64>().unwrap_or_default())
            .unwrap_or_default();

        let balance_delta = balance_adjusted - balance_current;
        let market_value_delta = market_value_adjusted - market_value_current;

        let price_label = if price_adjusted == 0.0 {
            "-".to_string()
        } else {
            format!(
                "({:.2}) {:.2}",
                price_adjusted - price_current,
                price_adjusted
            )
        };

        let balance_label = if balance_adjusted == 0.0 {
            "-".to_string()
        } else {
            format!("({:.2}) {:.2}", balance_delta, balance_adjusted)
        };

        let market_value_label = if market_value_adjusted == 0.0 {
            "-".to_string()
        } else {
            format!("({:.2}) {:.2}", market_value_delta, market_value_adjusted)
        };

        let float_epsilon = 0.0001;
        // todo: unused variables
        let _balance_color = if balance_label == "-" {
            GRAY_800
        } else {
            match balance_delta {
                x if x == 0.0 => GRAY_800,
                x if x > float_epsilon => GREEN_400,
                x if x < -float_epsilon => RED_400,
                _ => GRAY_800,
            }
        };
        // todo: unused variables
        let _market_value_color = if market_value_label == "-" {
            GRAY_800
        } else {
            match market_value_delta {
                x if x == 0.0 => GRAY_800,
                x if x > float_epsilon => GREEN_400,
                x if x < -float_epsilon => RED_400,
                _ => GRAY_800,
            }
        };

        vec![
            CellBuilder::new().value(Some(position.asset.symbol.clone())),
            CellBuilder::new().value(position.cost.map(|x| format!("{:.2}", x))),
            CellBuilder::new().child(label(price_label).secondary().build()),
            CellBuilder::new().value(position.balance.map(|x| format!("{:.2}", x))),
            CellBuilder::new().child(label(&balance_label).secondary().build()),
            CellBuilder::new().value(position.cost.map(|x| format!("{:.2}", x))),
            CellBuilder::new().child(label(&market_value_label).secondary().build()),
        ]
    }

    pub fn headers() -> Vec<String> {
        vec![
            "Ticker".to_string(),
            "Price".to_string(),
            "Delta".to_string(),
            "Balance".to_string(),
            "Delta".to_string(),
            "Market Value".to_string(),
            "Delta".to_string(),
            "Weight".to_string(),
            "Delta".to_string(),
        ]
    }

    pub fn position_table(&self) -> TableBuilder<Message> {
        TableBuilder::new()
            .padding_cell(Sizes::Md)
            .padding_cell_internal(Sizes::Xs)
            .column(
                ColumnBuilder::new().headers(Self::headers()).rows(
                    self.original
                        .positions
                        .0
                        .iter()
                        .enumerate()
                        .map(|(pos_index, position)| {
                            let targets_cells = self.cell_builders_targets(position, pos_index);
                            let field_cells = self.cell_builders(position, pos_index);
                            let all_cells = field_cells.into_iter().chain(targets_cells).collect();

                            let row_background = match pos_index % 2 == 0 {
                                true => TABLE_ROW_1,
                                false => TABLE_ROW_2,
                            };

                            RowBuilder::new().cells(all_cells).style(move || {
                                ExcaliburContainer::default()
                                    .background_iced(row_background)
                                    .theme()
                            })
                        })
                        .collect(),
                ),
            )
    }
}

impl State for PortfolioTable {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, msg: Message) -> Command<Message> {
        let mut cmd = Command::none();

        match msg {
            Message::Load(portfolio) => {
                tracing::debug!("Loading portfolio: {:?}", portfolio.name);
                self.original = portfolio.clone();
            }
            Message::DeltaForm(msg) => {
                cmd = self.form.update(msg).map(Message::DeltaForm);
            }
            Message::Clear => {
                self.form.clear();
            }
            Message::Empty => {}
        }

        cmd
    }

    /// Renders the constructed table.
    fn view(&self) -> Element<'_, Self::ViewMessage> {
        self.position_table().build().into()
    }
}
