//! Implements the form for creating a portfolio.

use datatypes::portfolio::{coin::Coin, position::Position, Portfolio};

use super::*;
use crate::components::containers::CustomContainer;

/// Message type that passes forward the messages of this form.
type ParentMessage = create::Message;

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    NameChanged(Option<String>),
    TickerChanged(Option<String>),
    AssetSelected(usize),
    AssetPriceChanged(usize, Option<String>),
    AssetBalanceChanged(usize, Option<String>),
    Submit,
}

impl MessageWrapperView for Message {
    type ParentMessage = ParentMessage;
}

impl MessageWrapper for Message {
    type ParentMessage = ParentMessage;
}

/// Implements From for the parent message type.
/// This is the same as `impl From<Message> for ParentMessage` but
/// allows us to use the `ParentMessage` _type_ from the `MessageWrapper` trait,
/// instead of defining it directly.
impl From<Message> for <Message as MessageWrapper>::ParentMessage {
    fn from(message: Message) -> Self {
        Self::Form(message)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    pub name: Option<String>,
    pub ticker: Option<String>,
    pub assets: Vec<Asset>,
}

impl Form {
    pub fn new() -> Self {
        Self {
            name: None,
            ticker: None,
            assets: vec![],
        }
    }

    // Message types!
    // todo: why are these not used?
    pub type AppMessage = Message;
    pub type ViewMessage = form::Message;

    pub fn ready(&self) -> bool {
        self.name.is_some()
            && self.ticker.is_some()
            && !self
                .assets
                .iter()
                .filter(|x| x.selected)
                .collect::<Vec<_>>()
                .is_empty()
    }

    #[tracing::instrument(skip(self))]
    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }

    /// Formats the current form into a 2D table data vector.
    pub fn table_data(&self) -> Vec<Vec<String>> {
        let mut data = vec![];
        for asset in self.assets.iter().filter(|x| x.selected) {
            data.push(vec![
                asset.ticker.clone(),
                asset.price.clone().unwrap_or_default(),
                asset.balance.clone().unwrap_or_default(),
            ]);
        }
        data
    }

    pub fn submit(&mut self) -> Command<Self::AppMessage> {
        let assets = self
            .assets
            .iter()
            .filter(|asset| asset.selected)
            .map(|asset| {
                Position::new(
                    asset.coin.clone(),
                    Some(
                        asset
                            .price
                            .clone()
                            .unwrap_or_default()
                            .parse::<f64>()
                            .unwrap(),
                    ),
                    Some(
                        asset
                            .balance
                            .clone()
                            .unwrap_or_default()
                            .parse::<f64>()
                            .unwrap(),
                    ),
                    None,
                    None,
                )
            })
            .collect();
        let portfolio = Portfolio::new(
            self.name.clone().unwrap_or_default().to_lowercase(),
            self.ticker.clone().unwrap_or_default().to_uppercase(),
            assets,
        );

        // Don't overwrite if the name exists.
        if portfolio.file_path().exists() {
            tracing::error!(
                "Portfolio already exists: {:?}. Use a different name.",
                portfolio
            );
            return Command::none();
        }

        // Save to file.
        let res = portfolio.save();
        match res {
            Ok(_) => {
                tracing::debug!(
                    "Portfolio saved: {:?} to path {:?}",
                    portfolio,
                    portfolio.file_path()
                );
            }
            Err(e) => {
                tracing::error!("Failed to save portfolio: {:?}", e);
            }
        }

        Command::none()
    }

    pub fn headers(&self) -> Vec<String> {
        vec![
            "Ticker".to_string(),
            "Price".to_string(),
            "Balance".to_string(),
            "Select".to_string(),
        ]
    }

    pub fn cell_builder(&self) -> CellBuilder<Self::AppMessage> {
        CellBuilder::new()
    }

    pub fn row_builder(&self) -> RowBuilder<Self::AppMessage> {
        RowBuilder::new()
    }

    /// Builds the columns with this form's headers.
    pub fn column_builder(&self) -> ColumnBuilder<Self::AppMessage> {
        ColumnBuilder::new().headers(self.headers())
    }

    /// Styles the table builder with the following style:
    /// - Cells are padded internally and externally.
    /// - Spacing between the "stacked" rows is medium.
    pub fn table_builder(&self) -> TableBuilder<Self::AppMessage> {
        TableBuilder::new()
            .padding_cell_internal(Sizes::Xs)
            .padding_cell(Sizes::Sm)
            .padding_row(Sizes::Md)
            .spacing_col(Sizes::Md)
    }
}

impl State for Form {
    type ViewMessage = form::Message;
    type AppMessage = Message;

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Message::Empty => {}
            Message::NameChanged(name) => self.name = name,
            Message::TickerChanged(ticker) => self.ticker = ticker,
            Message::AssetSelected(index) => {
                self.assets[index].selected = !self.assets[index].selected
            }
            Message::AssetPriceChanged(index, price) => {
                tracing::trace!("Price changed: {}", price.clone().unwrap_or_default());
                self.assets[index].price = price
            }
            Message::AssetBalanceChanged(index, balance) => self.assets[index].balance = balance,
            Message::Submit => return self.submit().map(|x| x),
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let assets = self.assets.clone();
        let table =
            self.table_builder().column(
                self.column_builder().rows(
                    assets
                        .into_iter()
                        .enumerate()
                        .map(|(i, asset)| {
                            RowBuilder::new()
                                .style(|| {
                                    CustomContainer::theme(Some(iced::Background::Color(GRAY_500)))
                                })
                                .cell(CellBuilder::new().value(Some(asset.ticker)))
                                .cell(CellBuilder::new().value(asset.price))
                                .cell(CellBuilder::new().value(asset.balance).on_change(move |x| {
                                    Self::ViewMessage::AssetBalanceChanged(i, x)
                                }))
                                .cell(
                                    CellBuilder::new()
                                        .checked(Some(asset.selected))
                                        .on_checkbox(move |_x| form::Message::AssetSelected(i)),
                                )
                        })
                        .collect(),
                ),
            );

        let name_input = labeled_input(
            "Name".to_string(),
            self.name.clone(),
            "Portfolio name".to_string(),
            form::Message::NameChanged,
        );

        let ticker_input = labeled_input(
            "Ticker".to_string(),
            self.ticker.clone(),
            "Portfolio ticker".to_string(),
            form::Message::TickerChanged,
        );

        Container::new(
            Column::new()
                .spacing(Sizes::Lg)
                .push(
                    Row::new()
                        .spacing(Sizes::Lg)
                        .push(name_input.width(Length::FillPortion(2)))
                        .push(ticker_input.width(Length::FillPortion(2))),
                )
                .push(
                    Column::new()
                        .spacing(Sizes::Md)
                        .push(label(&"Assets").secondary().build())
                        .push(scrollable(table.build())),
                ),
        )
        .into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Asset {
    pub coin: Coin,
    pub ticker: String,
    pub price: Option<String>,
    pub balance: Option<String>,
    pub selected: bool,
}

impl Asset {
    pub fn new(coin: Coin, price: Option<String>) -> Self {
        let ticker = coin.symbol.clone();
        Self {
            coin,
            ticker,
            price,
            balance: None,
            selected: false,
        }
    }
}
