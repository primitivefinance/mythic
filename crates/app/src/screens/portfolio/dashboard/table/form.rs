//! Portfolio position form for editing position deltas.
use super::*;

/// Form for editing individual position deltas.
/// Maps the position's index in the portfolio's positions to the delta,
/// for each respective position field.
#[derive(Debug, Clone, Default)]
pub struct DeltaForm {
    pub price: HashMap<usize, String>,
    pub balance: HashMap<usize, String>,
    pub market_value: HashMap<usize, String>,
    pub weight: HashMap<usize, String>,
}

#[derive(Debug, Clone, Default)]
pub enum DeltaFormMessage {
    #[default]
    Empty,
    Price(usize, Option<String>),
    Balance(usize, Option<String>),
    MarketValue(usize, Option<String>),
    Weight(usize, Option<String>),
}

impl MessageWrapper for DeltaFormMessage {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for DeltaFormMessage {
    type ParentMessage = super::Message;
}

impl From<DeltaFormMessage> for <DeltaFormMessage as MessageWrapper>::ParentMessage {
    fn from(msg: DeltaFormMessage) -> Self {
        super::Message::DeltaForm(msg)
    }
}

impl DeltaForm {
    pub fn clear(&mut self) {
        self.price.clear();
        self.balance.clear();
        self.market_value.clear();
        self.weight.clear();
    }
}

impl State for DeltaForm {
    type AppMessage = DeltaFormMessage;
    type ViewMessage = DeltaFormMessage;

    fn update(&mut self, msg: Self::AppMessage) -> Command<Self::AppMessage> {
        match msg {
            DeltaFormMessage::Price(index, price) => {
                tracing::trace!("Price changed: {}", price.clone().unwrap_or_default());
                self.price.insert(index, price.unwrap_or_default());
            }
            DeltaFormMessage::Balance(index, balance) => {
                tracing::trace!("Balance changed: {}", balance.clone().unwrap_or_default());
                self.balance.insert(index, balance.unwrap_or_default());
            }
            DeltaFormMessage::MarketValue(index, market_value) => {
                tracing::trace!(
                    "Market value changed: {}",
                    market_value.clone().unwrap_or_default()
                );
                self.market_value
                    .insert(index, market_value.unwrap_or_default());
            }
            DeltaFormMessage::Weight(index, weight) => {
                tracing::trace!(
                    "Weight changed: {} for position {}",
                    weight.clone().unwrap_or_default(),
                    index
                );
                self.weight.insert(index, weight.unwrap_or_default());
            }
            _ => {}
        }
        Command::none()
    }

    /// todo: right now this form just holds the state + update functionality,
    /// but does not render the table.
    /// todo: make sure this empty column doesn't mess any layouts up.
    fn view(&self) -> Element<Self::AppMessage> {
        Column::new().into()
    }
}
