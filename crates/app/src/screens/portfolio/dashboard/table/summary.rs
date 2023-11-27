//! Renders a summary of the deltas applied to the table's
//! form.

use super::*;

/// Renders a summary of the edited deltas to the portfolio.
#[derive(Debug, Clone, Default)]
pub struct DeltaSummary {
    pub on: bool,
    pub deltas: Vec<PositionDelta>,
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    /// Toggles the summary on or off via stateful `self.on`.
    /// This got added after running into issues trying to call view() on the
    /// summary which was stored as Option<Summary>.
    /// However, in the table there is a separate function from view that
    /// handles this case for us. So we are not using toggle, but it's going
    /// to stay here for reference or future use.
    Toggle,
    // todo: add copy to clipboard or something here.
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::Summary(msg)
    }
}

impl DeltaSummary {
    pub type ViewMessage = Message;

    pub fn new() -> Self {
        Self {
            on: true,
            deltas: vec![PositionDelta::default()],
        }
    }

    pub fn deltas(self, deltas: Vec<PositionDelta>) -> Self {
        Self { deltas, ..self }
    }

    pub fn headers() -> Vec<String> {
        vec!["Field".to_string(), "Delta".to_string()]
    }

    pub fn table(&self, index: usize) -> TableBuilder<Self::ViewMessage> {
        let position = self
            .deltas
            .get(index)
            .unwrap_or(&PositionDelta::default())
            .clone();

        let mut data: Vec<(String, String)> = Vec::new();
        if let Some(balance) = position.balance.clone() {
            data.push(("Balance".to_string(), balance));
        }

        for (i, target) in position.targets.iter().enumerate() {
            if let Some(target) = target.clone() {
                data.push((format!("{:?}", target), target.to_string()));
            }
        }

        key_value_table(Self::headers(), data)
    }
}

impl State for DeltaSummary {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Toggle => {
                self.on = !self.on;
            }
            Message::Empty => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        // Render nothing if the summary is not on.
        if !self.on {
            return Column::new().into();
        }

        let mut rows: Vec<Element<'a, Self::ViewMessage>> = vec![];

        for (i, pos) in self.deltas.iter().enumerate() {
            rows.push(
                Column::new()
                    .spacing(Sizes::Sm)
                    .push(key_value_row("Name".to_string(), format!("Position {}", i)))
                    .push(self.table(i).build())
                    .into(),
            );
        }

        Column::new()
            .push(label_item("Adjustments Overview".to_string()))
            .push(Row::with_children(rows).spacing(Sizes::Lg))
            .width(Length::FillPortion(3))
            .into()
    }
}
