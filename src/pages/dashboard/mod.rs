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

const PANE_ID_COLOR_UNFOCUSED: iced::Color = iced::Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: iced::Color = iced::Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

impl From<Dashboard> for Page {
    fn from(screen: Dashboard) -> Self {
        Page::new(Box::new(screen))
    }
}

impl Lifecycle for Dashboard {
    type AppMessage = app::AppMessage;
    type ViewMessage = view::ViewMessage;

    fn load(&self) -> Task<Self::AppMessage> {
        Task::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Task<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::ViewMessage::Dashboard(message)) => match message {
                Message::Empty => Task::none(),
                Message::Split(axis, pane) => {
                    let result = self
                        .panes
                        .split(axis, pane, pane::Pane::new(self.panes_created));

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;

                    Task::none()
                }
                Message::SplitFocused(axis) => {
                    if let Some(pane) = self.focus {
                        let result =
                            self.panes
                                .split(axis, pane, pane::Pane::new(self.panes_created));

                        if let Some((pane, _)) = result {
                            self.focus = Some(pane);
                        }

                        self.panes_created += 1;
                    }

                    Task::none()
                }
                Message::FocusAdjacent(direction) => {
                    if let Some(pane) = self.focus {
                        if let Some(adjacent) = self.panes.adjacent(pane, direction) {
                            self.focus = Some(adjacent);
                        }
                    }
                    Task::none()
                }
                Message::Clicked(pane) => {
                    self.focus = Some(pane);
                    Task::none()
                }
                Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                    self.panes.resize(split, ratio);
                    Task::none()
                }
                Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                    self.panes.drop(pane, target);
                    Task::none()
                }
                Message::TogglePin(pane) => {
                    if let Some(pane::Pane { is_pinned, .. }) = self.panes.get_mut(pane) {
                        *is_pinned = !*is_pinned;
                    }
                    Task::none()
                }
                Message::Maximize(pane) => {
                    self.panes.maximize(pane);
                    Task::none()
                }
                Message::Restore => {
                    self.panes.restore();
                    Task::none()
                }
                Message::Close(pane) => {
                    if let Some((_, sibling)) = self.panes.close(pane) {
                        self.focus = Some(sibling);
                    }
                    Task::none()
                }
                Message::CloseFocused => {
                    if let Some(pane) = self.focus {
                        if let Some(pane::Pane { is_pinned, .. }) = self.panes.get(pane) {
                            if !is_pinned {
                                if let Some((_, sibling)) = self.panes.close(pane) {
                                    self.focus = Some(sibling);
                                }
                            }
                        }
                    }

                    Task::none()
                }
                _ => Task::none(),
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let grid: PaneGrid<'_, Self::ViewMessage> =
            PaneGrid::new(&self.panes, |id, pane, is_maximized| {
                let is_focused = focus == Some(id);

                let pin_button: iced::widget::Button<'_, Self::ViewMessage> = button(
                    iced::widget::text(if pane.is_pinned { "unpin" } else { "pin" }).size(14),
                )
                .on_press(Message::TogglePin(id).into())
                .padding(3);

                let title = Row::new()
                    .push(pin_button)
                    .push("Pane")
                    .push(text(pane.id.to_string()).color(if is_focused {
                        PANE_ID_COLOR_FOCUSED
                    } else {
                        PANE_ID_COLOR_UNFOCUSED
                    }))
                    .spacing(5);

                let title_bar = pane_grid::TitleBar::new(title)
                    .controls(pane_grid::Controls::dynamic(
                        pane::view_controls(id, total_panes, pane.is_pinned, is_maximized)
                            .map(|x| x.into()),
                        button(text("X").size(14))
                            .style(button::danger)
                            .padding(3)
                            .on_press_maybe(if total_panes > 1 && !pane.is_pinned {
                                Some(Message::Close(id).into())
                            } else {
                                None
                            }),
                    ))
                    .padding(10)
                    .style(if is_focused {
                        style::title_bar_focused
                    } else {
                        style::title_bar_active
                    });

                pane_grid::Content::new(responsive(move |size| {
                    pane::view_content(id, total_panes, pane.is_pinned, size).map(|x| x.into())
                }))
                .title_bar(title_bar)
                .style(if is_focused {
                    style::pane_focused
                } else {
                    style::pane_active
                })
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .on_click(|x| Message::Clicked(x).into())
            .on_drag(|x| Message::Dragged(x).into())
            .on_resize(10, |x| Message::Resized(x).into());

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

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
