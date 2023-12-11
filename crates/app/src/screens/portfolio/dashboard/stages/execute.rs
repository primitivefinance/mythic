use clients::dev::ProtocolPosition;

use super::*;
use crate::components::system::{label, ExcaliburButton};

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
    pub dev_client: Option<DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>>>,
    pub tx_receipt: Option<TransactionReceipt>,
    pub on_submit: Option<super::Message>,
    pub position: Option<ProtocolPosition>,
}

impl Execute {
    pub fn new(
        dev_client: Option<DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>>>,
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

    pub fn view_tx_logs(&self) -> Container<'static, Message> {
        let mut content = Column::new()
            .spacing(Sizes::Lg)
            .push(label(&"Transaction Receipt").build());

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
            .into()
    }

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
            .into()
    }
}

#[tracing::instrument(skip(dev_client), level = "trace", ret)]
async fn execute_create_position(
    dev_client: DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>>,
) -> anyhow::Result<Option<TransactionReceipt>> {
    let caller = dev_client.protocol.client.address();
    let tx = dev_client
        .create_position(caller, 75.0, 1.0, 1.0, 1.0, 1.0)
        .await?;

    Ok(tx)
}

#[tracing::instrument(skip(dev_client), level = "trace", ret)]
async fn fetch_protocol_position(
    dev_client: DevClient<SignerMiddleware<Provider<Ws>, LocalWallet>>,
) -> anyhow::Result<ProtocolPosition> {
    let caller = dev_client.protocol.client.address();
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
                        if let Some(original) = &self.original {
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
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::ViewMessage> {
        let mut content = Column::new()
            .spacing(Sizes::Lg)
            .push(label(&"Execute Adjustment").title2().build());

        if let Some(dev_client) = &self.dev_client {
            content = content.push(
                ExcaliburButton::new()
                    .danger()
                    .build(label(&"Execute").build())
                    .padding(Sizes::Md)
                    .on_press(Message::Execute),
            )
        };

        let mut result_content = Row::new().spacing(Sizes::Lg);

        if self.tx_receipt.is_some() {
            result_content = result_content.push(self.view_tx_logs());
        }

        if self.position.is_some() {
            result_content = result_content.push(self.view_position());
        }

        content = content.push(result_content);

        Container::new(content).into()
    }
}
