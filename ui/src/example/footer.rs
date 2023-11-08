//! Simple footer component that has system and version info for Excalibur.

use std::collections::{HashMap, VecDeque};

use iced::{
    alignment,
    widget::{column, container, row, text, Row},
    Color, Element, Length, Renderer,
};

#[derive(Debug, Clone)]
pub enum Message {
    #[allow(dead_code)]
    Debug,
}

pub struct Footer {
    pub info: HashMap<String, String>,
}

impl Footer {
    pub fn view<'a>(&self) -> Element<'a, Message, Renderer> {
        let mut content: Vec<Element<_>> = vec![];

        let max_items = 2;

        for (key, value) in &self.info {
            let text = text(format!("{}: {}", key, value))
                .size(10)
                .style(iced::theme::Text::Color(DIMMED_TEXT));
            content.push(text.into());
        }

        container(create_columns(content, max_items))
            .width(Length::Fill)
            .height(Length::Fixed(45.0))
            .align_x(alignment::Horizontal::Left)
            .style(super::styles::background::BackgroundContainer::theme())
            .into()
    }
}

/// Hacky way to avoid a lot of elements in a column from clipping and instead
/// fill a row.
pub fn create_columns<'a, Message: 'a + Clone>(
    data: Vec<Element<'a, Message>>,
    max_items_per_column: usize,
) -> Row<'a, Message> {
    let mut row = row![]
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(alignment::Alignment::Center);
    let mut queue: VecDeque<_> = data.into_iter().collect();

    while !queue.is_empty() {
        let mut column = column![]
            .spacing(2)
            .padding(4)
            .align_items(alignment::Alignment::Start)
            .height(Length::Fill);
        for _ in 0..max_items_per_column {
            if let Some(item) = queue.pop_front() {
                column = column.push(item);
            }
        }
        row = row.push(column);
    }

    row
}

pub const DIMMED_TEXT: Color = Color::from_rgb(
    0x6D as f32 / 255.0,
    0x70 as f32 / 255.0,
    0x77 as f32 / 255.0,
);

use std::{env, process::Command};

pub struct FooterBuilder {
    info: HashMap<String, String>,
}

impl FooterBuilder {
    pub fn new() -> Self {
        Self {
            info: HashMap::new(),
        }
    }

    pub fn add_crate_info(mut self) -> Self {
        let name = env!("CARGO_PKG_NAME").to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();
        self.info.insert("Crate Name".to_string(), name);
        self.info.insert("Crate Version".to_string(), version);
        self
    }

    pub fn add_git_commit(mut self) -> Self {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .expect("Failed to execute git command");
        let commit = String::from_utf8(output.stdout).expect("Not UTF-8");
        self.info.insert("Git Commit".to_string(), commit);
        self
    }

    pub fn add_system_info(mut self) -> Self {
        let system_info = env::consts::OS.to_string();
        self.info.insert("System Info".to_string(), system_info);
        self
    }

    pub fn build(self) -> Footer {
        Footer { info: self.info }
    }
}
