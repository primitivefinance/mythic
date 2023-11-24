//! Empty is a default screen if no app is selected.

use iced::widget::Container;

use super::*;
use crate::components::tables::dev_table;

pub struct DeveloperScreen {
    pub cache: Option<String>,
    pub selected: Option<String>,
    pub checkboxed: bool,
}

impl DeveloperScreen {
    pub fn new() -> Self {
        Self {
            cache: None,
            selected: None,
            checkboxed: false,
        }
    }
}

impl From<DeveloperScreen> for Screen {
    fn from(screen: DeveloperScreen) -> Self {
        Screen::new(Box::new(screen))
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Empty,
    OnChange(Option<String>),
    OnSubmit,
    OnSelect(String),
    OnCheckbox(bool),
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
                _ => {}
            },
            _ => {}
        }

        Command::none()
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let mut column = Column::new();
        column = column.push(h2("Developer".to_string()));

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
