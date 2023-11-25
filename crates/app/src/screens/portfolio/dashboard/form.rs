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

impl DeltaForm {
    pub fn clear(&mut self) {
        self.price.clear();
        self.balance.clear();
        self.market_value.clear();
        self.weight.clear();
    }
}
