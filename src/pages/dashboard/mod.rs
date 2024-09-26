use app::SharedState;
use iced::widget::{
    pane_grid::{self, PaneGrid},
    responsive, Column, Container,
};

use super::*;
use crate::components::system::ExcaliburContainer;

use crate::components::panes;

#[derive(Debug, Clone, Copy, Default)]
pub enum Message {
    #[default]
    Empty,

    Panes(panes::Message),
}

pub struct Dashboard {
    panes: pane_grid::State<panes::Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,

    shared_state: SharedState,
}

impl Dashboard {
    pub fn new(shared_state: SharedState) -> Self {
        let (panes, _) = pane_grid::State::new(panes::Pane::new(0));

        Self {
            panes,
            panes_created: 1,
            focus: None,
            shared_state,
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
                Message::Panes(message) => match message {
                    panes::Message::Open(pane_type) => {
                        if let Some(focus) = self.focus {
                            let new_pane =
                                panes::Pane::new_with_type(self.panes_created, pane_type);
                            let result =
                                self.panes.split(pane_grid::Axis::Vertical, focus, new_pane);

                            if let Some((pane, _)) = result {
                                self.focus = Some(pane);
                            }

                            self.panes_created += 1;
                        }
                        Task::none()
                    }
                    panes::Message::Split(axis, pane) => {
                        let result =
                            self.panes
                                .split(axis, pane, panes::Pane::new(self.panes_created));

                        if let Some((pane, _)) = result {
                            self.focus = Some(pane);
                        }

                        self.panes_created += 1;

                        Task::none()
                    }
                    panes::Message::SplitFocused(axis) => {
                        if let Some(pane) = self.focus {
                            let result =
                                self.panes
                                    .split(axis, pane, panes::Pane::new(self.panes_created));

                            if let Some((pane, _)) = result {
                                self.focus = Some(pane);
                            }

                            self.panes_created += 1;
                        }

                        Task::none()
                    }
                    panes::Message::FocusAdjacent(direction) => {
                        if let Some(pane) = self.focus {
                            if let Some(adjacent) = self.panes.adjacent(pane, direction) {
                                self.focus = Some(adjacent);
                            }
                        }
                        Task::none()
                    }
                    panes::Message::Clicked(pane) => {
                        self.focus = Some(pane);
                        Task::none()
                    }
                    panes::Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                        self.panes.resize(split, ratio);
                        Task::none()
                    }
                    panes::Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                        self.panes.drop(pane, target);
                        Task::none()
                    }
                    panes::Message::TogglePin(pane) => {
                        if let Some(panes::Pane { is_pinned, .. }) = self.panes.get_mut(pane) {
                            *is_pinned = !*is_pinned;
                        }
                        Task::none()
                    }
                    panes::Message::Maximize(pane) => {
                        self.panes.maximize(pane);
                        Task::none()
                    }
                    panes::Message::Restore => {
                        self.panes.restore();
                        Task::none()
                    }
                    panes::Message::Close(pane) => {
                        if let Some((_, sibling)) = self.panes.close(pane) {
                            self.focus = Some(sibling);
                        }
                        Task::none()
                    }
                    panes::Message::CloseFocused => {
                        if let Some(pane) = self.focus {
                            if let Some(panes::Pane { is_pinned, .. }) = self.panes.get(pane) {
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
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let state = self.shared_state.read().unwrap();
        tracing::debug!("State: {:?}", state);

        let block_number = state.get::<u64>("block_number").unwrap_or_default();
        tracing::debug!("Block number: {}", block_number);

        let grid: PaneGrid<'_, Self::ViewMessage> =
            PaneGrid::new(&self.panes, |id, pane, is_maximized| {
                let is_focused = focus == Some(id);

                let pin_button: iced::widget::Button<'_, Self::ViewMessage> = button(
                    iced::widget::text(if pane.is_pinned { "unpin" } else { "pin" }).size(14),
                )
                .on_press(panes::Message::TogglePin(id).into())
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
                        panes::basic::view_controls(id, total_panes, pane.is_pinned, is_maximized)
                            .map(|x| x.into()),
                        button(text("X").size(14))
                            .style(button::danger)
                            .padding(3)
                            .on_press_maybe(if total_panes > 1 && !pane.is_pinned {
                                Some(panes::Message::Close(id).into())
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

                pane_grid::Content::new(responsive(move |size| match pane.pane_type {
                    panes::PaneType::Basic => {
                        panes::basic::view_content(id, total_panes, pane.is_pinned, size)
                            .map(|x| x.into())
                    }
                    panes::PaneType::Blocks => panes::blocks::view_blocks(
                        id,
                        total_panes,
                        pane.is_pinned,
                        size,
                        block_number,
                    )
                    .map(|x| x.into()),
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
            .on_click(|x| panes::Message::Clicked(x).into())
            .on_drag(|x| panes::Message::Dragged(x).into())
            .on_resize(10, |x| panes::Message::Resized(x).into());

        ExcaliburContainer::default()
            .build(grid)
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

impl From<panes::Message> for Message {
    fn from(message: panes::Message) -> Self {
        Self::Panes(message)
    }
}

impl From<panes::Message> for view::ViewMessage {
    fn from(message: panes::Message) -> Self {
        Self::Dashboard(message.into())
    }
}

mod style {
    use crate::components::system::{ExcaliburColor, LabelColors};
    use iced::widget::container;
    use iced::{Border, Theme};

    use super::GRAY_600;

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(ExcaliburColor::Label(LabelColors::Secondary).color()),
            background: Some(ExcaliburColor::Background4.into()),
            border: Border {
                width: 2.0,
                color: ExcaliburColor::Custom(GRAY_600).into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(ExcaliburColor::Label(LabelColors::Primary).color()),
            background: Some(ExcaliburColor::Card.into()),
            border: Border {
                width: 2.0,
                color: ExcaliburColor::Custom(GRAY_600).into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(ExcaliburColor::Background2.into()),
            border: Border {
                width: 2.0,
                color: ExcaliburColor::Custom(GRAY_600).into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(ExcaliburColor::Background2.into()),
            border: Border {
                width: 2.0,
                color: ExcaliburColor::Custom(GRAY_600).into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
