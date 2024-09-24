use iced::widget::{
    pane_grid::{self, PaneGrid},
    responsive, Column, Container,
};

use super::*;
use crate::components::system::label;

pub mod pane;

#[derive(Debug, Clone, Copy, Default)]
pub enum Message {
    #[default]
    Empty,

    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
}

pub struct Dashboard {
    panes: pane_grid::State<pane::Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

impl Dashboard {
    pub fn new() -> Self {
        let (panes, _) = pane_grid::State::new(pane::Pane::new(0));

        Self {
            panes,
            panes_created: 1,
            focus: None,
        }
    }
}

impl From<Dashboard> for Page {
    fn from(screen: Dashboard) -> Self {
        Page::new(Box::new(screen))
    }
}

impl Lifecycle for Dashboard {
    type AppMessage = app::AppMessage;
    type ViewMessage = view::ViewMessage;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::ViewMessage::Dashboard(message)) => match message {
                Message::Empty => Command::none(),
                Message::Split(axis, pane) => {
                    let result = self
                        .panes
                        .split(axis, &pane, pane::Pane::new(self.panes_created));

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;

                    Command::none()
                }
                Message::TogglePin(pane) => {
                    if let Some(pane::Pane { is_pinned, .. }) = self.panes.get_mut(&pane) {
                        *is_pinned = !*is_pinned;
                    }

                    Command::none()
                }
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let grid: PaneGrid<'_, Self::ViewMessage> =
            PaneGrid::new(&self.panes, |id, pane, is_maximized| {
                let is_focused = focus == Some(id);

                let pin_button: iced::widget::Button<'_, Self::ViewMessage> =
                    button(text(if pane.is_pinned { "unpin" } else { "pin" }).size(14))
                        .on_press(Message::TogglePin(id).into())
                        .padding(3);

                let title = Row::new()
                    .push(pin_button)
                    .push(text(format!("Pane {}", "1".to_string())))
                    .spacing(5);

                let title_bar = pane_grid::TitleBar::new(title).controls(
                    pane::view_controls(id, total_panes, pane.is_pinned, is_maximized)
                        .map(|x| x.into()),
                );

                pane_grid::Content::new(responsive(move |size| {
                    pane::view_content(id, total_panes, pane.is_pinned, size).map(|x| x.into())
                }))
                .title_bar(title_bar)
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);

        Container::new(grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        Subscription::none()
    }
}

impl From<Message> for app::RootMessage {
    fn from(message: Message) -> Self {
        Self::View(view::ViewMessage::Dashboard(message))
    }
}

impl From<Message> for view::ViewMessage {
    fn from(message: Message) -> Self {
        Self::Dashboard(message)
    }
}
