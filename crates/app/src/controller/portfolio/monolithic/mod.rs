pub mod create;
mod inventory;
mod metrics;
mod view;

use datatypes::portfolio::coin::Coin;

use self::{
    create::LiquidityTypes,
    metrics::Metrics,
    view::{MonolithicPresenter, MonolithicView},
};
use super::{dashboard::stages::review::Times, *};
use crate::model::portfolio::AlloyAddress;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Allocate,
    Form(FormMessage),
    SelectPosition(AlloyAddress),
}

#[derive(Debug, Clone, Default)]
pub enum FormMessage {
    #[default]
    Empty,
    Amount(Option<String>),
    Asset(Coin),
    Quote(Coin),
    Duration(Times),
    EndPrice(Option<String>),
    Liquidity(LiquidityTypes),
    Submit,
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(msg: Message) -> Self {
        super::Message::Monolithic(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Monolithic {
    model: Model,
    presenter: MonolithicPresenter,
    create: create::Form,
    allocate: bool,
    view_position: Option<AlloyAddress>,
}

impl Monolithic {
    pub fn new(model: Model) -> Self {
        let presenter = MonolithicPresenter::new(model.clone());
        Self {
            model,
            presenter,
            create: create::Form::new(),
            allocate: false,
            view_position: None,
        }
    }
}

impl State for Monolithic {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::Empty => Command::none(),
            Self::AppMessage::SelectPosition(address) => {
                self.view_position = Some(address);
                Command::none()
            }
            Self::AppMessage::Allocate => {
                self.allocate = true;
                Command::none()
            }
            Self::AppMessage::Form(form_message) => match form_message {
                FormMessage::Empty => Command::none(),
                FormMessage::Submit => {
                    self.allocate = false;
                    Command::none()
                }
                FormMessage::Amount(amount) => {
                    self.create.amount = amount;
                    Command::none()
                }
                FormMessage::Asset(asset) => {
                    self.create.chosen_asset = Some(asset);
                    Command::none()
                }
                FormMessage::Quote(quote) => {
                    self.create.chosen_quote = Some(quote);
                    Command::none()
                }
                FormMessage::Duration(duration) => {
                    self.create.duration = Some(duration);
                    Command::none()
                }
                FormMessage::EndPrice(end_price) => {
                    self.create.end_price = end_price;
                    Command::none()
                }
                FormMessage::Liquidity(liquidity) => {
                    self.create.liquidity = Some(liquidity);
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Self::ViewMessage> {
        let (positions, logos) = self.presenter.get_positions();
        let mut content = Column::new().spacing(Sizes::Md);
        content = content.push(MonolithicView::new().layout(
            positions,
            logos,
            Some(Message::Allocate),
            |x| Message::SelectPosition(x),
            |x| Message::Form(FormMessage::Amount(x)),
        ));

        if self.allocate {
            content = content.push(
                self.create
                    .view::<FormMessage>(
                        FormMessage::Submit,
                        |x| FormMessage::Amount(x),
                        |x| FormMessage::Asset(x),
                        |x| FormMessage::Quote(x),
                        |x| FormMessage::Duration(x),
                        |x| FormMessage::EndPrice(x),
                        |x| FormMessage::Liquidity(x),
                    )
                    .map(|x| Message::Form(x)),
            );
        } else {
            if let Some(address) = self.view_position {
                let (title, external_price, aum, health) = self.presenter.get_metrics(address);

                content = content.push(Metrics::layout(
                    title,
                    external_price,
                    label("USD"),
                    aum,
                    health,
                ));
            }
        }

        scrollable(
            Container::new(content)
                .max_height(5000.0)
                .padding(Sizes::Xl),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        Subscription::none()
    }
}
