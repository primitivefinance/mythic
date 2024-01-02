use clients::dev::ProtocolPosition;

use super::*;
use crate::{
    components::system::{label, ExcaliburButton},
    middleware::Protocol,
};

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Execute,
    ExecuteResult(anyhow::Result<Option<TransactionReceipt>, Arc<anyhow::Error>>),
    FetchPosition,
    FetchPositionResult(anyhow::Result<ProtocolPosition, Arc<anyhow::Error>>),
    NewStrategyPosition(Portfolio),
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::Execute(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Execute {
    pub original: Option<Portfolio>,
    pub dev_client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
    pub tx_receipt: Option<TransactionReceipt>,
    pub on_submit: Option<super::Message>,
    pub position: Option<ProtocolPosition>,
}

impl Execute {
    pub fn new(
        dev_client: Option<Arc<ExcaliburMiddleware<Ws, LocalWallet>>>,
        original_portfolio: Option<Portfolio>,
    ) -> Self {
        Self {
            original: original_portfolio,
            dev_client,
            tx_receipt: None,
            on_submit: None,
            position: None,
        }
    }

    /// Returns an instructions element to guide the user.
    #[allow(dead_code)]
    pub fn guide(&self) -> Container<'static, super::Message> {
        instructions(
            vec![instruction_text(
                "Execute the portfolio adjustment strategy prepared before exiting".to_string(),
            )],
            Some("Exit".to_string()),
            None,
            self.on_submit.clone(),
        )
    }
    #[allow(dead_code)]
    pub fn view_tx_logs(&self) -> Container<'static, Message> {
        let mut content = Column::new()
            .spacing(Sizes::Lg)
            .push(label("Transaction Receipt").build());

        if let Some(tx_receipt) = &self.tx_receipt {
            let logs = tx_receipt.logs.clone();
            let logs = logs
                .iter()
                .map(|log| {
                    let mut log = log.clone();
                    log.topics = log.topics.iter().map(|_| H256::zero()).collect();
                    log
                })
                .collect::<Vec<Log>>();

            let logs = serde_json::to_string_pretty(&logs).unwrap();
            content = content.push(Text::new(logs));
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
    }
    #[allow(dead_code)]
    pub fn view_position(&self) -> Container<'static, Message> {
        let mut content = Column::new()
            .spacing(Sizes::Lg)
            .push(Text::new("Protocol Position").size(40));

        if let Some(position) = &self.position {
            let position = serde_json::to_string_pretty(&position).unwrap();
            content = content.push(Text::new(position));
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
    }
}

#[tracing::instrument(skip(dev_client), level = "trace", ret)]
async fn execute_create_position(
    dev_client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<Option<TransactionReceipt>> {
    let caller = dev_client.address().unwrap();
    let tx = dev_client
        .create_position(caller, 75.0, 1.0, 1.0, 1.0, 1.0)
        .await?;

    Ok(tx)
}

#[tracing::instrument(skip(dev_client), level = "trace", ret)]
async fn fetch_protocol_position(
    dev_client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<ProtocolPosition> {
    let _caller = dev_client.address().unwrap();
    let position = dev_client.get_position().await?;
    Ok(position)
}

impl State for Execute {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Execute => {
                tracing::debug!("Executing adjustment strategy.");
                let dev_client = self.dev_client.clone().unwrap();
                Command::perform(execute_create_position(dev_client), |res| {
                    Message::ExecuteResult(res.map_err(Arc::new))
                })
            }
            Message::ExecuteResult(result) => {
                tracing::debug!("Execute result: {:?}", result);
                match result {
                    Ok(tx_receipt) => {
                        self.tx_receipt = tx_receipt;
                        self.on_submit = Some(super::Message::Step);
                        let dev_client = self.dev_client.clone().unwrap();
                        return Command::perform(fetch_protocol_position(dev_client), |res| {
                            Message::FetchPositionResult(res.map_err(Arc::new))
                        });
                    }
                    Err(e) => {
                        tracing::error!("Execute error: {:?}", e);
                    }
                }
                Command::none()
            }
            Message::FetchPosition => {
                tracing::debug!("Fetching protocol position.");
                let dev_client = self.dev_client.clone().unwrap();
                Command::perform(fetch_protocol_position(dev_client), |res| {
                    Message::FetchPositionResult(res.map_err(Arc::new))
                })
            }
            Message::FetchPositionResult(result) => {
                tracing::debug!("Fetch position result: {:?}", result);
                match result {
                    Ok(position) => {
                        self.position = Some(position.clone());

                        // Use this information to build a new portfolio, and
                        // then emit a NewStrategyPosition message to have the
                        // portfolio dashboard update.
                        if let Some(_original) = &self.original {
                            // let portfolio = original.clone();
                            // let price_y_per_x = position
                            // .internal_price
                            // .map(|x| x.parse::<f64>().unwrap())
                            // .unwrap();
                            // let price_x_per_y = 1.0 / price_y_per_x;
                            //
                            // let mut strategy_position_x =
                            // portfolio.positions.0[0].clone();
                            // strategy_position_x.balance = position
                            // .balance_x
                            // .map(|x| x.parse::<f64>().unwrap())
                            // .clone();
                            //
                            // strategy_position_x.cost = Some(price_x_per_y);
                            //
                            // let mut strategy_position_y =
                            // portfolio.positions.0[1].clone();
                            // strategy_position_y.balance = position
                            // .balance_y
                            // .map(|y| y.parse::<f64>().unwrap())
                            // .clone();
                            // strategy_position_y.cost = Some(price_y_per_x);
                            //
                            // let positions =
                            // Positions::new(vec![strategy_position_x,
                            // strategy_position_y]);
                            //
                            // let mut strategy_portfolio = portfolio.clone();
                            // strategy_portfolio.positions = positions;
                            //
                            // return Command::perform(async {}, move |_| {
                            // Message::NewStrategyPosition(strategy_portfolio)
                            // });
                        }
                    }
                    Err(e) => {
                        tracing::error!("Fetch position error: {:?}", e);
                    }
                }
                Command::none()
            }
            Message::NewStrategyPosition(portfolio) => {
                tracing::debug!("New strategy portfolio: {:?}", portfolio);
                Command::none()
            }
            Message::Empty => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::ViewMessage> {
        Card::build_container(
            Column::new()
                .width(Length::Fill)
                .push(
                    Container::new(
                        Column::new()
                            .push(label("Instructions").secondary().build())
                            .push(
                                label("Create a position to get started.")
                                    .billions()
                                    .build(),
                            )
                            .width(Length::Fill)
                            .padding(Sizes::Lg),
                    )
                    .width(Length::Fill),
                )
                .push(
                    ExcaliburContainer::default()
                        .background(ExcaliburColor::Background2)
                        .border_radius([0.0, 0.0, Sizes::Sm.into(), Sizes::Sm.into()].into())
                        .build(
                            ExcaliburButton::new()
                                .primary()
                                .build(label("Create position").build())
                                .padding([8, 16, 8, 16])
                                .on_press(Message::Execute),
                        )
                        .padding(Sizes::Lg)
                        .center_x()
                        .center_y()
                        .width(Length::Fill),
                ),
        )
        .width(Length::Fill)
        .into()
    }
}
