//! Implements the create portfolio screen.

pub mod form;

use profiles::coins::CoinList;

use super::*;
use crate::components::tables::{
    builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, rows::RowBuilder,
};

type ParentMessage = developer::Message;

#[derive(Debug, Clone, Default)]
pub struct CreatePortfolio {
    form: form::Form,
    coinlist: CoinList,
}

#[derive(Debug, Default, Clone)]
pub enum Message {
    #[default]
    Empty,
    Form(form::Message),
    Load(anyhow::Result<CoinList, Arc<anyhow::Error>>),
    Submit,
}

impl From<Message> for ParentMessage {
    fn from(message: Message) -> Self {
        ParentMessage::Create(message)
    }
}

impl From<Message> for view::Message {
    fn from(message: Message) -> Self {
        view::Message::CreatePortfolio(message)
    }
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        // This is very important, if we just did `message.into()` it would cause
        // a stack overflow if we used it as a message to return in a command in this
        // component and if we used in a child component.
        let view_msg: view::Message = message.into();
        view_msg.into()
    }
}

#[tracing::instrument(ret)]
async fn load_coinlist() -> anyhow::Result<CoinList, Arc<anyhow::Error>> {
    let coinlist = CoinList::load(None);
    let coinlist = match coinlist {
        Ok(coinlist) => coinlist,
        Err(e) => {
            tracing::error!("Failed to load coinlist: {:?}", e);
            return Err(Arc::new(e));
        }
    };

    Ok(coinlist)
}

impl CreatePortfolio {
    pub fn ready(&self) -> bool {
        self.form.ready()
    }
}

impl State for CreatePortfolio {
    type ViewMessage = view::Message;
    type AppMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::perform(load_coinlist(), |x| {
            // todo: fix this to point to its own message for its own screen.
            Message::Load(x).into()
        })
    }

    fn update(&mut self, message: Message) -> Command<Self::AppMessage> {
        tracing::trace!("Message: {:?}", message);
        match message {
            Message::Load(Ok(coinlist)) => {
                tracing::trace!("Loaded coinlist: {:?}", coinlist);
                // Load the coin list and also build the form by creating the assets.
                self.coinlist = coinlist;

                // todo: fetch balance + price?
                // this adds the loaded coins into the form's list.
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
            Message::Empty => {}
            _ => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let column_1: Vec<Element<'a, Self::ViewMessage>> = vec![
            h2("Create Portfolio".to_string())
                .font(FONT_SEMIBOLD)
                .into(),
            self.form.view().map(|x| {
                Self::ViewMessage::Developer(developer::Message::Create(Message::Form(x)))
            }),
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
            static_table(
                "Summary".to_string(),
                vec![
                    "Ticker".to_string(),
                    "Price".to_string(),
                    "Balance".to_string(),
                ],
                self.form.table_data(),
            )
            .into(),
            instruct.map(|x| developer::Message::Create(x).into()),
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
