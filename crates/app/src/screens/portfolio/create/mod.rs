//! Implements the create portfolio screen.

pub mod form;

use profiles::coins::CoinList;

use super::*;
use crate::components::tables::{
    builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder, rows::RowBuilder,
};

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
    pub fn load(&self) -> Command<app::Message> {
        Command::perform(load_coinlist(), |x| {
            // todo: fix this to point to its own message for its own screen.
            view::Message::Developer(developer::Message::Create(Message::Load(x))).into()
        })
    }

    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        tracing::trace!("Message: {:?}", message);
        match message {
            Message::Load(Ok(coinlist)) => {
                tracing::trace!("Loaded coinlist: {:?}", coinlist);
                // Load the coin list and also build the form by creating the assets.
                self.coinlist = coinlist;

                // todo: fetch balance + price?
                // this adds the loaded coins into the form's list.
                for token in self.coinlist.tokens.iter() {
                    self.form
                        .add_asset(form::Asset::new(token.name.clone(), 20.0));
                }
            }
            Message::Load(Err(e)) => {
                tracing::error!("Failed to load coinlist: {:?}", e);
            }
            Message::Form(message) => return self.form.update(message),
            Message::Submit => return self.form.submit(),
            Message::Empty => {}
            _ => {}
        }

        Command::none()
    }

    pub fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let column_1: Vec<Element<'a, view::Message>> = vec![
            h2("Create Portfolio".to_string())
                .font(FONT_SEMIBOLD)
                .into(),
            self.form
                .view()
                .map(|x| view::Message::Developer(developer::Message::Create(Message::Form(x)))),
        ];
        // todo: add to state
        let ready = false;
        let action = match ready {
            true => Some(Message::Submit),
            false => None,
        };
        let column_2: Vec<Element<'a, view::Message>> = vec![
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
            instructions(
                vec![instruction_text(
                    "Create a new portfolio by filling out the form.".to_string(),
                )],
                Some("Create Portfolio".to_string()),
                None,
                action,
            )
            .map(|x| view::Message::Developer(developer::Message::Create(x))),
        ];

        dual_column(column_1, column_2).into()
    }
}
