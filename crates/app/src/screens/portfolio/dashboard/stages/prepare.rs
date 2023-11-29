//! Stage for confirming the desired portfolio adjustments before continuing to
//! review and simulate.

use profiles::portfolios::{Portfolio, Position};

use super::*;
use crate::screens::{
    portfolio::dashboard::table::PositionDelta, MessageWrapper, MessageWrapperView,
};

/// This encapsulates the adjustments to the portfolio that are being made.
#[derive(Debug, Clone, Default)]
pub struct PreparePayload {
    /// Original immutable portfolio.
    pub original: Portfolio,
    /// State for the adjusted portfolio.
    pub adjusted: Portfolio,
    /// Vector of all positions that have been updated.
    pub altered: Vec<usize>,
}

impl PreparePayload {
    pub fn new(original: Portfolio) -> Self {
        Self {
            original: original.clone(),
            adjusted: original.clone(),
            altered: Vec::new(),
        }
    }

    /// Applies a weight adjustment to the adjusted portfolio's position.
    pub fn adjust_weight(&mut self, pos_index: usize, delta: f64) {
        if !self.altered.contains(&pos_index) {
            self.altered.push(pos_index);
        }

        let mut position = self.adjusted.positions[pos_index].clone();
        let weight = position.weight.unwrap_or_default();
        position.weight = Some(weight + delta);
        self.adjusted.positions[pos_index] = position;
    }

    /// Applies a cost adjustment to the adjusted portfolio's position.
    pub fn adjust_cost(&mut self, pos_index: usize, cost: f64) {
        if !self.altered.contains(&pos_index) {
            self.altered.push(pos_index);
        }

        let mut position = self.adjusted.positions[pos_index].clone();
        position.cost = Some(cost);
        self.adjusted.positions[pos_index] = position;
    }

    pub fn adjust_balance(&mut self, pos_index: usize, balance: f64) {
        if !self.altered.contains(&pos_index) {
            self.altered.push(pos_index);
        }

        let mut position = self.adjusted.positions[pos_index].clone();
        position.balance = Some(balance);
        self.adjusted.positions[pos_index] = position;
    }

    pub fn get_adjustments(&self) -> Vec<Adjustment> {
        let mut adjustments = Vec::new();

        if self.altered.is_empty() {
            return adjustments;
        }

        for &pos_index in &self.altered {
            let original_position = self.original.positions[pos_index].clone();
            let adjusted_position = self.adjusted.positions[pos_index].clone();
            let adjustment = Adjustment::new(original_position, adjusted_position);
            adjustments.push(adjustment);
        }

        adjustments
    }

    pub fn get_balance_delta(&self) -> f64 {
        let adjustments = self.get_adjustments();
        adjustments
            .iter()
            .map(|adjustment| adjustment.get_balance_delta())
            .sum()
    }

    pub fn get_cost_delta(&self) -> f64 {
        let adjustments = self.get_adjustments();
        adjustments
            .iter()
            .map(|adjustment| adjustment.get_cost_delta())
            .sum()
    }

    pub fn get_market_value_delta(&self) -> f64 {
        let adjustments = self.get_adjustments();
        adjustments
            .iter()
            .map(|adjustment| adjustment.get_market_value_delta())
            .sum()
    }

    pub fn get_weight_delta(&self) -> f64 {
        let adjustments = self.get_adjustments();
        adjustments
            .iter()
            .map(|adjustment| adjustment.get_weight_delta())
            .sum()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Adjustment {
    pub original: Position,
    pub adjusted: Position,
}

impl Adjustment {
    pub fn new(original: Position, adjusted: Position) -> Self {
        Self { original, adjusted }
    }

    pub fn get_balance_delta(&self) -> f64 {
        self.adjusted.balance.unwrap_or(0.0) - self.original.balance.unwrap_or(0.0)
    }

    pub fn get_cost_delta(&self) -> f64 {
        self.adjusted.cost.unwrap_or(0.0) - self.original.cost.unwrap_or(0.0)
    }

    pub fn get_market_value_delta(&self) -> f64 {
        let original_bal = self.original.balance.unwrap_or(0.0);
        let adjusted_bal = self.adjusted.balance.unwrap_or(0.0);
        let original_cost = self.original.cost.unwrap_or(0.0);
        let adjusted_cost = self.adjusted.cost.unwrap_or(0.0);

        let original_market_value = original_bal * original_cost;
        let adjusted_market_value = adjusted_bal * adjusted_cost;

        adjusted_market_value - original_market_value
    }

    pub fn get_weight_delta(&self) -> f64 {
        let original = self.original.weight.unwrap_or_default();
        let adjusted = self.adjusted.weight.unwrap_or_default();
        f64::from(adjusted) - f64::from(original)
    }
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Submit,
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Prepare(message)
    }
}

/// Stage for preparing the adjustments.
#[derive(Debug, Clone, Default)]
pub struct Prepare {
    pub payload: PreparePayload,
}

/// Cursor made this algorithm to adjust weights in a portfolio of weights that
/// sum to 1.
/// todo: need to disable the "dependent" inputs.
/// todo: need this to go in a much better place than here, like api or
/// something.
#[tracing::instrument(ret)]
pub fn adjust_weights_algorithm(items: &mut Vec<f64>, changes: Vec<(usize, f64)>) -> Vec<f64> {
    // Create a hashmap for quick lookup
    let changes: std::collections::HashMap<usize, f64> = changes.into_iter().collect();

    // Calculate the total change
    let total_change: f64 = changes.values().sum();

    // Adjust the weights
    for (i, weight) in items.iter_mut().enumerate() {
        if let Some(&change) = changes.get(&i) {
            if change != 0.0 {
                *weight += change;
            } else {
                // Adjust the weight proportionally to maintain the total weight
                *weight *= (1.0 - total_change);
            }
        } else {
            // Adjust the weight proportionally to maintain the total weight
            *weight *= (1.0 - total_change);
        }

        // Round the weight to the nearest 2 decimal places
        *weight = (*weight * 100.0).round() / 100.0;
    }

    items.clone()
}

impl Prepare {
    pub fn new(original: Portfolio, deltas: Vec<PositionDelta>) -> Self {
        let mut payload = PreparePayload::new(original);

        for delta in deltas {
            let pos_index = delta.id;
            let weight = delta
                .weight
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or_default();
            let cost = delta
                .price
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or_default();
            let balance = delta
                .balance
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or_default();

            payload.adjust_weight(pos_index, weight);
            payload.adjust_cost(pos_index, cost);
            // todo: adjust weight should handle this probably.
            payload.adjust_balance(pos_index, balance);
        }

        Self { payload }
    }

    /// Returns an instructions element to guide the user.
    pub fn guide(&self, on_submit: Option<super::Message>) -> Container<'static, super::Message> {
        instructions(
            vec![instruction_text(
                "Review the proposed adjustments before continuing to select an adjustment strategy."
                    .to_string(),
            )],
            Some("Select Adjustment Strategy".to_string()),
            None,
            on_submit,
        )
    }
}

impl State for Prepare {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Empty => Command::none(),
            Message::Submit => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let adjustments = self.payload.get_adjustments();

        key_value_table(
            vec![
                "Position".to_string(),
                "Weight % / Market Value $".to_string(),
            ],
            adjustments
                .iter()
                .map(|adjustment| {
                    let weight_delta = adjustment.get_weight_delta();
                    let market_value_delta = adjustment.get_market_value_delta();

                    tracing::info!(
                        "weight_delta: {}, market_value_delta: {}",
                        weight_delta,
                        market_value_delta
                    );

                    (
                        adjustment.adjusted.asset.name.clone(),
                        format!("{:.2}% / ${:.2}", weight_delta * 100.0, market_value_delta),
                    )
                })
                .collect(),
        )
        .into()
    }
}
