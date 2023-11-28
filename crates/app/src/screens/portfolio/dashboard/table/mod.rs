//! Portfolio table which renders all positions and a form for editing deltas to
//! adjust the portfolio's positions.

use profiles::portfolios::{Portfolio, Position, Targetable};

use self::summary::DeltaSummary;
use super::*;

pub mod form;
pub mod summary;
use form::*;

#[derive(Debug, Clone, Default)]
pub struct PortfolioTable {
    pub form: DeltaForm,
    pub positions: Vec<Position>,
    pub summary: Option<DeltaSummary>,
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Updates the underlying portfolio that is being rendered.
    Portfolio(Portfolio),
    /// Updates the form within the table.
    DeltaForm(form::DeltaFormMessage),
    /// Refreshes the summary table with the edited deltas.
    Ready,
    /// Clears the summary table and form.
    Clear,
    /// Refreshes the summary table sibling component.
    /// note: This is not rendered by this component's `view` method.
    Summary(summary::Message),
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::PortfolioTable(msg)
    }
}

/// Individual deltas of a given position.
/// todo: support price and market value deltas...
#[derive(Debug, Clone, Default)]
pub struct PositionDelta {
    pub balance: Option<String>,
    pub targets: Vec<Option<Targetable>>,
}

impl PortfolioTable {
    pub type AppMessage = Message;

    pub fn new() -> Self {
        Self {
            form: DeltaForm::default(),
            positions: Vec::new(),
            summary: None,
        }
    }

    /// If adjustments have been prepared and the summary table is being
    /// rendered.
    pub fn prepared(&self) -> bool {
        self.summary.is_some()
    }

    /// Closure for handling the form events.
    type FormEvent = Box<dyn Fn(Option<String>) -> Self::AppMessage>;

    /// Gets the edited form fields as deltas that can be rendered by the
    /// summary table.
    /// todo: support more targets, price, and market value fields.
    pub fn get_form_deltas(&self) -> Vec<PositionDelta> {
        self.positions
            .iter()
            .enumerate()
            .map(|(pos_index, position)| {
                let balance = self.form.balance.get(&pos_index).cloned();
                let targets =
                    position
                        .clone()
                        .targets
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|target| matches!(target, Targetable::Weight(_)))
                        .map(|_target| {
                            self.form.weight.get(&pos_index).cloned().map(|x| Targetable::from_string(Targetable::Weight(0.0), x))
                        })
                        .collect::<Vec<Option<Targetable>>>();

                PositionDelta { balance, targets }
            })
            .collect()
    }

    /// todo: investigate if this empty element will cause any problems...
    pub fn summary_table(&self) -> Element<'_, Self::AppMessage> {
        match self.summary.as_ref() {
            Some(summary) => summary.view().map(|x| x.into()),
            None => iced::widget::Space::new(0.0, 0.0).into(),
        }
    }

    /// Renders the dual cell column for a target value and its delta input
    /// field.
    /// - pos_index is the index of the position in the portfolio's positions.
    /// - target is the target value to be displayed in the first cell.
    /// Why this I make this closure stuff so complicated?
    pub fn target_cell(
        &self,
        pos_index: usize,
        target: Targetable,
    ) -> Vec<CellBuilder<Self::AppMessage>> {
        // todo: support more targets
        let (value, on_change_msg) = match target {
            Targetable::Weight(_x) => (
                self.form.weight.get(&pos_index).cloned(),
                Box::new(move |x| form::DeltaFormMessage::Weight(pos_index, x).into())
                    as Self::FormEvent,
            ),
            _ => (
                None,
                Box::new(|_x| form::DeltaFormMessage::Empty.into()) as Self::FormEvent,
            ),
        };

        vec![
            CellBuilder::new().value(Some(target.clone().to_string())),
            CellBuilder::new()
                .value(value)
                .on_change(on_change_msg)
                .style(|| CustomContainer::theme(Some(iced::Background::Color(GRAY_400)))),
        ]
    }

    /// Gets the cell builders for each target in the position.
    pub fn cell_builders_targets<'a>(
        &'a self,
        position: &'a Position,
        pos_index: usize,
    ) -> Vec<CellBuilder<Self::AppMessage>> {
        position
            .clone()
            .targets
            .unwrap_or_default()
            .into_iter()
            .filter(|target| matches!(target, Targetable::Weight(_)))
            .flat_map(|target| self.target_cell(pos_index, target).into_iter())
            .collect::<Vec<CellBuilder<Self::AppMessage>>>()
    }

    /// Gets the builders for each cell in the table.
    pub fn cell_builders<'a>(
        &'a self,
        position: &'a Position,
        pos_index: usize,
    ) -> Vec<CellBuilder<Self::AppMessage>> {
        vec![
            CellBuilder::new().value(Some(position.asset.symbol.clone())),
            CellBuilder::new().value(position.cost.map(|x| x.to_string())),
            CellBuilder::new()
                .value(self.form.price.get(&pos_index).cloned())
                .on_change(move |x| form::DeltaFormMessage::Price(pos_index, x).into())
                .style(|| CustomContainer::theme(Some(iced::Background::Color(GRAY_400)))),
            CellBuilder::new().value(position.balance.map(|x| x.to_string())),
            CellBuilder::new()
                .value(self.form.balance.get(&pos_index).cloned())
                .on_change(move |x| form::DeltaFormMessage::Balance(pos_index, x).into())
                .style(|| CustomContainer::theme(Some(iced::Background::Color(GRAY_400)))),
            CellBuilder::new().value(position.cost.map(|x| x.to_string())),
            CellBuilder::new()
                .value(self.form.market_value.get(&pos_index).cloned())
                .on_change(move |x| form::DeltaFormMessage::MarketValue(pos_index, x).into())
                .style(|| CustomContainer::theme(Some(iced::Background::Color(GRAY_400)))),
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

    pub fn position_table(&self) -> TableBuilder<Self::AppMessage> {
        TableBuilder::new()
            .padding_cell(Sizes::Md)
            .padding_cell_internal(Sizes::Xs)
            .column(
                ColumnBuilder::new().headers(Self::headers()).rows(
                    self.positions
                        .iter()
                        .enumerate()
                        .map(|(pos_index, position)| {
                            let targets_cells = self.cell_builders_targets(position, pos_index);
                            let field_cells = self.cell_builders(position, pos_index);
                            let all_cells = field_cells.into_iter().chain(targets_cells).collect();

                            RowBuilder::new().cells(all_cells).style(|| {
                                CustomContainer::theme(Some(iced::Background::Color(GRAY_500)))
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

    fn update(&mut self, msg: Self::AppMessage) -> Command<Self::AppMessage> {
        let mut cmd = Command::none();

        match msg {
            Self::AppMessage::Portfolio(portfolio) => {
                self.positions = portfolio.positions;
            }
            Self::AppMessage::DeltaForm(msg) => {
                cmd = self.form.update(msg).map(Self::AppMessage::DeltaForm);
            }
            Self::AppMessage::Ready => {
                self.summary = Some(DeltaSummary::new().deltas(self.get_form_deltas()))
            }
            Self::AppMessage::Clear => {
                self.form.clear();
                self.summary = None;
            }
            Self::AppMessage::Summary(msg) => match self.summary.as_mut() {
                Some(summary) => {
                    cmd = summary.update(msg).map(Self::AppMessage::Summary);
                }
                None => {}
            },
            Self::AppMessage::Empty => {}
            _ => {}
        }

        cmd
    }

    /// Renders the constructed table.
    fn view(&self) -> Element<'_, Self::ViewMessage> {
        self.position_table().build().into()
    }
}
