//! Signers are any entity that can sign and execute transactions.
//! These signers can be used within the app.

use alloy_primitives::Address;
use anyhow::Error;
use clients::ledger::{types::DerivationType, *};

use super::*;
use crate::components::{
    system::{label, ExcaliburButton},
    tables::{builder::TableBuilder, cells, columns::ColumnBuilder, rows::RowBuilder},
};

#[derive(Debug, Default, Clone)]
pub enum Message {
    #[default]
    NotConnected,
    Connected(Result<(Arc<LedgerClient>, Address), Arc<Error>>),
    ConnectLedger,
}

impl MessageWrapper for Message {
    type ParentMessage = super::Message;
}

impl MessageWrapperView for Message {
    type ParentMessage = super::Message;
}

impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Signers(message)
    }
}

pub enum SignerManagement {
    NotConnected,
    Connected(Arc<LedgerClient>, Address),
    Connecting,
    Error,
}
pub async fn connect_to_ledger() -> Result<(Arc<LedgerClient>, Address), Arc<Error>> {
    let ledger = Arc::new(LedgerClient::new_connection(DerivationType::LedgerLive(0)).await?);
    let address = ledger.get_address().await.unwrap();
    tracing::info!("Address: {:?}", address);
    Ok((ledger, address))
}

impl SignerManagement {
    pub fn new() -> Self {
        Self::NotConnected
    }

    pub fn _signer_table(&self) -> TableBuilder<Message> {
        TableBuilder::new().padding_cell(Sizes::Md).column(
            ColumnBuilder::new()
                .headers(vec!["Name".to_string(), "Address".to_string()])
                .rows(vec![RowBuilder::new()
                    .cell(cells::CellBuilder::new().value(Some("Wallet Name".to_string())))
                    .cell(
                        cells::CellBuilder::new().value(Some("0xb0b".to_string())),
                    )]),
        )
    }

    pub fn singer_view() -> Row<'static, Message> {
        let row_1 = Row::new()
            .spacing(Sizes::Sm)
            .push(label("labeled_name_input").build());
        let row_2 = Row::new().spacing(Sizes::Sm).push(
            Column::new()
                .spacing(Sizes::Md)
                .push(label("Instructions").build())
                .width(Length::FillPortion(2)),
        );
        Row::new().push(Column::new().push(row_1).push(row_2).spacing(Sizes::Md))
    }
}

impl State for SignerManagement {
    type AppMessage = Message;
    type ViewMessage = Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        tracing::info!("Update message: {:?}", message);
        match message {
            Message::Connected(Ok(res)) => {
                tracing::info!("Connected to ledger");
                *self = SignerManagement::Connected(res.0, res.1);
                Command::none()
            }
            Message::Connected(Err(_err)) => {
                tracing::error!("Error connecting to ledger");
                *self = SignerManagement::Error;
                Command::none()
            }
            Message::ConnectLedger => {
                *self = SignerManagement::Connecting;
                Command::perform(connect_to_ledger(), Message::Connected) // hangs here
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let mut content = Column::new().spacing(Sizes::Lg).padding(Sizes::Lg);

        let mut upper_half = Column::new().spacing(Sizes::Md).push(
            label("Manage Signer Settings")
                .title2()
                .primary()
                .middle()
                .build(),
        );

        let mut lower_half = Column::new().spacing(Sizes::Md);

        let (upper_content, lower_content) = match self {
            SignerManagement::NotConnected => (
                Row::new().spacing(Sizes::Md).push(
                    ExcaliburButton::new()
                        .primary()
                        .build(label("Connect Ledger Device").build())
                        .padding(Sizes::Sm)
                        .on_press(Message::ConnectLedger),
                ),
                Row::<Message>::new(),
            ),
            SignerManagement::Connecting => (
                Row::new().spacing(Sizes::Md).push(
                    ExcaliburButton::new()
                        .primary()
                        .build(label("Connecting").build())
                        .padding(Sizes::Sm)
                        .on_press(Message::ConnectLedger),
                ),
                Row::<Message>::new(),
            ),
            SignerManagement::Connected(_ledger, _address) => (
                Row::new().spacing(Sizes::Md).push(
                    ExcaliburButton::new()
                        .primary()
                        .build(label("Connected").build())
                        .padding(Sizes::Sm)
                        .on_press(Message::ConnectLedger),
                ),
                SignerManagement::singer_view(),
            ),
            SignerManagement::Error => (
                Row::new().spacing(Sizes::Md).push(
                    ExcaliburButton::new()
                        .primary()
                        .build(label("Error connecting, try again").build())
                        .padding(Sizes::Sm)
                        .on_press(Message::ConnectLedger),
                ),
                Row::<Message>::new().spacing(Sizes::Md),
            ),
        };
        upper_half = upper_half.push(upper_content);
        lower_half = lower_half.push(lower_content);
        content = content.push(upper_half);
        content = content.push(lower_half);
        Container::new(content)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
