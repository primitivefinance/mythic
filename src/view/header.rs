//! Header implements a section of the app for the logo, navigation and profile.

use iced::Color;

use crate::components::system::label;

use super::navigation::{NavEvent, Navigation};
use super::*;

const TITLE: &str = "Mythic";

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct Header {
    pub last_event: NavEvent,
    pub current_nav: Navigation,
}

impl Header {
    pub fn new() -> Self {
        Self {
            last_event: NavEvent::Empty,
            current_nav: Navigation::Empty,
        }
    }
}

impl Header {
    pub fn view_nav_items(&self) -> Element<'_, view::ViewMessage> {
        let mut column = Column::new();
        column = column.push(self.current_nav.view().map(|x| x.into()));
        column
            .spacing(Sizes::Xs)
            .align_x(alignment::Alignment::Center)
            .into()
    }
}

impl Lifecycle for Header {
    type AppMessage = NavEvent;
    type ViewMessage = view::ViewMessage;

    fn update(&mut self, message: NavEvent) -> Task<NavEvent> {
        self.last_event = message.clone();

        match message {
            NavEvent::Navigate(to) => {
                self.current_nav = to;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        /* let title = Column::new()
        .push(
            Row::new()
                .spacing(Sizes::Sm)
                .align_y(alignment::Alignment::Center)
                .push(label(TITLE).title3().branding().build()),
        )
        .padding(Sizes::Lg)
        .align_x(alignment::Alignment::Center)
        .width(Length::Fill); */

        Container::new(
            Column::new()
                .push(
                    Column::new().push(
                        Container::new(Column::new())
                            .width(Length::Fill)
                            .height(Length::Fixed(1.0))
                            .style(move |_theme| {
                                ExcaliburContainer::default()
                                    .background_iced(Color::BLACK)
                                    .theme()
                            }),
                    ),
                )
                .push(
                    Column::new()
                        .push(self.view_nav_items())
                        .spacing(Sizes::Lg)
                        .padding(Sizes::Xs),
                )
                .spacing(Sizes::Md),
        )
        .height(Length::Fill)
        .into()
    }
}
