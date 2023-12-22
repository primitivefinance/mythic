//! Implements the create portfolio screen.

pub mod form;

use datatypes::portfolio::coin_list::CoinList;

use super::*;
use crate::components::{
    system::label,
    tables::{builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, rows::RowBuilder},
};

type ParentMessage = super::Message;

#[derive(Debug, Default, Clone)]
pub enum Message {
    #[default]
    Empty,
    Form(form::Message),
    Load(anyhow::Result<CoinList, Arc<anyhow::Error>>),
    Submit,
}

impl MessageWrapperView for Message {
    type ParentMessage = ParentMessage;
}

impl MessageWrapper for Message {
    type ParentMessage = ParentMessage;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Create(message)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CreatePortfolio {
    form: form::Form,
    coinlist: CoinList,
}

impl CreatePortfolio {
    pub fn new(profile: UserProfile) -> Self {
        Self {
            form: form::Form::new(),
            coinlist: profile.coins,
        }
    }

    pub fn ready(&self) -> bool {
        self.form.ready()
    }
}

impl State for CreatePortfolio {
    type ViewMessage = Message;
    type AppMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        tracing::trace!("Message: {:?}", message);
        match message {
            Message::Load(Ok(coinlist)) => {
                tracing::trace!("Loaded coinlist: {:?}", coinlist);
                // Load the coin list and also build the form by creating the assets.
                self.coinlist = coinlist;

                // todo: fetch balance + price?
                // this adds the loaded coins into the form's list.
                // in order to do this, we need to fetch the balance and price for each coin
                // which afaik both need to be sync.
                for token in self.coinlist.tokens.iter().cloned() {
                    self.form
                        .add_asset(form::Asset::new(token, Some(format!("{}", 20.0))));
                }
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load coinlist: {:?}", e);
            }
            Message::Form(message) => return self.form.update(message).map(|x| x.into()),
            Message::Submit => return self.form.submit().map(|x| x.into()),
            // TODO: resave user profiles in the submit command here ^^^.
            Message::Empty => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let column_1: Vec<Element<'a, Self::ViewMessage>> = vec![
            label("Create Portfolio").title2().build().into(),
            self.form.view().map(|x| x.into()),
        ];

        let action = match self.ready() {
            true => Some(Message::Submit),
            false => None,
        };

        let instruct: Element<'a, Message> = instructions(
            vec![instruction_text(
                "Create a new portfolio by filling out the form.".to_string(),
            )],
            Some("Create Portfolio".to_string()),
            None,
            action,
        )
        .max_width(ByteScale::Xl5)
        .into();

        let column_2: Vec<Element<'a, Self::ViewMessage>> = vec![
            key_value_row(
                "Name".to_string(),
                self.form.name.clone().unwrap_or("n/a".to_string()),
            )
            .into(),
            key_value_row(
                "Ticker".to_string(),
                self.form.ticker.clone().unwrap_or("n/a".to_string()),
            )
            .into(),
            instruct.map(|x| x),
        ];

        Container::new(
            DualColumn::new()
                .columns(column_1, column_2)
                .spacing(Sizes::Lg)
                .padding(Sizes::Lg.into())
                .build()
                .spacing(Sizes::Xl2),
        )
        .align_y(alignment::Vertical::Top)
        .center_x()
        .max_height(ByteScale::Xl7)
        .max_width(ByteScale::Xl7.between(&ByteScale::Xl8))
        .padding(Sizes::Xl)
        .into()
    }
}
