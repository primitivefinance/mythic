//! Empty is a default screen if no app is selected.

use std::collections::HashMap;

use iced::widget::Container;

use super::{
    portfolio::create::{self, CreatePortfolio},
    *,
};
use crate::components::tables::{asset_selection_table, dev_table, Asset};

pub struct DeveloperScreen {
    pub cache: Option<String>,
    pub selected: Option<String>,
    pub checkboxed: bool,
    pub assets: HashMap<String, Asset>,
    pub create_screen: CreatePortfolio,
}

impl From<DeveloperScreen> for Screen {
    fn from(screen: DeveloperScreen) -> Self {
        Screen::new(Box::new(screen))
    }
}

impl DeveloperScreen {
    pub fn new() -> Self {
        let assets = vec![
            Asset {
                ticker: "AAPL".to_string(),
                price: 100.0,
                selected: false,
            },
            Asset {
                ticker: "TSLA".to_string(),
                price: 200.0,
                selected: false,
            },
        ];

        let mut assets_map = HashMap::new();
        for asset in assets {
            assets_map.insert(asset.ticker.clone(), asset);
        }

        let create_screen = CreatePortfolio::default();

        Self {
            cache: None,
            selected: None,
            checkboxed: false,
            assets: assets_map,
            create_screen,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    OnChange(Option<String>),
    OnSubmit,
    OnSelect(String),
    OnCheckbox(bool),
    OnSelectAsset(String),
    Create(create::Message),
}

impl From<Message> for view::Message {
    fn from(msg: Message) -> Self {
        view::Message::Developer(msg)
    }
}

impl From<create::Message> for Message {
    fn from(msg: create::Message) -> Self {
        Message::Create(msg)
    }
}

impl From<Message> for app::Message {
    fn from(msg: Message) -> Self {
        app::Message::View(view::Message::Developer(msg))
    }
}

impl State for DeveloperScreen {
    fn load(&self) -> Command<app::Message> {
        self.create_screen.load()
    }

    fn update(&mut self, message: app::Message) -> Command<app::Message> {
        match message {
            app::Message::View(view::Message::Developer(msg)) => match msg {
                Message::Create(message) => return self.create_screen.update(message),
                Message::OnChange(value) => {
                    self.cache = value;
                }
                Message::OnSubmit => {
                    tracing::debug!("submit");
                }
                Message::OnSelect(value) => {
                    self.selected = Some(value);
                }
                Message::OnCheckbox(value) => {
                    self.checkboxed = value;
                }
                Message::OnSelectAsset(ticker) => {
                    let asset = self
                        .assets
                        .iter_mut()
                        .find(|asset| asset.1.ticker.contains(&ticker))
                        .unwrap();
                    asset.1.selected = !asset.1.selected;
                }
                _ => {}
            },
            _ => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let column = self.create_screen.view();

        Container::new(column)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
