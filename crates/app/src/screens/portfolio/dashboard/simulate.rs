use super::*;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Submit,
}

impl MessageWrapperView for Message {
    type ParentMessage = view::Message;
}

impl MessageWrapper for Message {
    type ParentMessage = dashboard::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Simulated(message)
    }
}

impl From<Message> for <Message as MessageWrapperView>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Developer(developer::Message::Dash(message.into()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {}

#[derive(Debug, Clone, Default)]
pub struct Simulate {
    form: Form,
    metrics: Vec<String>,
}

impl Simulate {
    pub fn new() -> Self {
        Self {
            form: Form::default(),
            metrics: vec!["metric 1".to_string(), "metric 2".to_string()],
        }
    }
}

impl State for Simulate {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Empty => {}
            Message::Submit => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let mut content = Column::new()
            .push(Text::new("Simulate").size(40))
            .push(Text::new("Table here").size(20));

        content = content.push(
            Column::new()
                .spacing(Sizes::Md)
                .push(
                    Row::with_children(
                        self.metrics
                            .iter()
                            .map(|x| Card::new(label_item(x.to_string())).into())
                            .collect::<Vec<Element<'a, Message>>>(),
                    )
                    .spacing(Sizes::Md),
                )
                .push(
                    Row::new()
                        .push(
                            Column::new()
                                .width(Length::FillPortion(3))
                                .push(label_item("Simulation Results".to_string()))
                                .push(
                                    Row::with_children(
                                        vec![
                                            Card::new(label_item("results 1".to_string())).into(),
                                            Card::new(label_item("results 2".to_string())).into(),
                                        ]
                                        .into_iter()
                                        .collect::<Vec<Element<'a, Message>>>(),
                                    )
                                    .spacing(Sizes::Md),
                                ),
                        )
                        .push(
                            instructions(
                                vec![label_item("Execute the Adjustment".to_string())],
                                None,
                                None,
                                Some(Message::Submit),
                            )
                            .width(Length::FillPortion(1)),
                        ),
                ),
        );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
