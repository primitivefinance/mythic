//! Empty is a default screen if no app is selected.

use std::collections::HashMap;

use iced::widget::Container;

use super::*;
use crate::components::tables::{asset_selection_table, dev_table, Asset};

pub struct DeveloperScreen {
    pub cache: Option<String>,
    pub selected: Option<String>,
    pub checkboxed: bool,
    pub assets: HashMap<String, Asset>,
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

        Self {
            cache: None,
            selected: None,
            checkboxed: false,
            assets: assets_map,
        }
    }
}

impl From<DeveloperScreen> for Screen {
    fn from(screen: DeveloperScreen) -> Self {
        Screen::new(Box::new(screen))
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
}

impl From<Message> for view::Message {
    fn from(msg: Message) -> Self {
        view::Message::Developer(msg)
    }
}

impl From<Message> for app::Message {
    fn from(msg: Message) -> Self {
        app::Message::View(view::Message::Developer(msg))
    }
}

impl State for DeveloperScreen {
    fn load(&self) -> Command<app::Message> {
        Command::none()
    }

    fn update(&mut self, message: app::Message) -> Command<app::Message> {
        match message {
            app::Message::View(view::Message::Developer(msg)) => match msg {
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
                    let mut asset = self
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
        let mut column = Column::new();
        column = column.push(h2("Developer".to_string()));

        let assets = self
            .assets
            .iter()
            .map(|asset| asset.1.clone())
            .collect::<Vec<Asset>>();

        let asset_selection: Container<'a, view::Message> =
            asset_selection_table(assets, |value| {
                Message::OnSelectAsset(value.unwrap_or_default()).into()
            });
        column = column.push(asset_selection);

        let current_state_row = Row::new()
            .spacing(Sizes::Lg)
            .push(text(format!(
                "Current input: {:?}",
                self.cache.as_ref().unwrap_or(&"None".to_string())
            )))
            .push(text(format!(
                "Selected: {:?}",
                self.selected.as_ref().unwrap_or(&"None".to_string())
            )))
            .push(text(format!("Checkbox: {:?}", self.checkboxed)));
        column = column.push(current_state_row);

        column = column.push(text("table"));
        let dev_tab = dev_table(self.cache.clone(), self.selected.clone(), self.checkboxed);
        column = column.push(dev_tab);

        view::app_layout(
            &view::Page::Developer,
            Container::new(column)
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .into()
    }
}
