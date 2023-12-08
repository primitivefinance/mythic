//! Stage for confirming the desired portfolio adjustments before continuing to
//! review and simulate.

use datatypes::portfolio::{position::Position, Portfolio};
use serde::{Deserialize, Serialize};

use super::*;
use crate::{
    components::system::label,
    screens::{MessageWrapper, MessageWrapperView},
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    SetAdjusted(Option<Portfolio>),
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
    pub original: Portfolio,
    pub adjusted: Option<Portfolio>,
    pub adjustments: Vec<Adjustment>,
}

impl Prepare {
    pub fn new(original: Portfolio) -> Self {
        Self {
            original,
            adjusted: None,
            adjustments: Vec::new(),
        }
    }

    /// Setting the adjusted portfolio will unlock the methods in this stage.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn set_adjusted(&mut self, adjusted: Option<Portfolio>) {
        self.adjusted = adjusted;
        self.adjustments = self.get_adjustments();
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub fn get_adjustments(&self) -> Vec<Adjustment> {
        let original = self.original.clone();
        let mut adjustments = Vec::new();

        tracing::debug!("Adjusted is some? {}", self.adjusted.is_some());

        if let Some(adjusted) = &self.adjusted {
            let original_positions = original.positions.0;
            let adjusted_positions = adjusted.positions.0.clone();

            tracing::debug!("original_positions: {:?}", original_positions);

            for (i, original_position) in original_positions.iter().enumerate() {
                let adjusted_position = adjusted_positions[i].clone();

                // This filters any positions that have no changed, however,
                // all positions should change given a single change because
                // the algorithm is designed to adjust all positions.
                tracing::debug!(
                    "positions equal ? {}",
                    *original_position == adjusted_position
                );
                if *original_position != adjusted_position {
                    let adjustment = Adjustment::new(original_position.clone(), adjusted_position);
                    adjustments.push(adjustment);
                }
            }
        }

        adjustments
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
            Message::SetAdjusted(adjusted) => {
                self.set_adjusted(adjusted);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        key_value_table(
            vec![
                "Position".to_string(),
                "Weight % / Market Value $".to_string(),
            ],
            self.adjustments
                .iter()
                .map(|adjustment| {
                    let weight_delta = adjustment.get_weight_delta();
                    let market_value_delta = adjustment.get_market_value_delta();

                    tracing::info!(
                        "weight_delta: {}, market_value_delta: {}",
                        weight_delta,
                        market_value_delta
                    );

                    let balance_color = match market_value_delta {
                        x if x > f64::EPSILON => GREEN_400,
                        x if x < -f64::EPSILON => RED_400,
                        _ => GRAY_800,
                    };

                    (
                        label(&adjustment.adjusted.asset.name.clone())
                            .secondary()
                            .build(),
                        label(&format!(
                            "{:.2}% / ${:.2}",
                            weight_delta * 100.0,
                            market_value_delta
                        ))
                        .highlight()
                        .build(),
                    )
                })
                .collect(),
        )
        .into()
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
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
