use super::*;

#[derive(Debug, Clone, Default)]
pub struct Form {}

#[derive(Debug, Clone, Default)]
pub struct Simulate {
    form: Form,
    metrics: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Submit,
}

impl Simulate {
    pub fn new() -> Self {
        Self {
            form: Form::default(),
            metrics: vec!["metric 1".to_string(), "metric 2".to_string()],
        }
    }

    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::Empty => {}
            Message::Submit => {}
        }

        Command::none()
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
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
