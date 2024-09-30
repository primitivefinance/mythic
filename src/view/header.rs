//! Header implements a section of the app for the logo, navigation and profile.

use iced::Alignment::{Center, End};
use iced::{Fill, Length};
use system::{ExcaliburColor, LabelColors};

use crate::components::system::label;

use super::navigation::{NavEvent, Navigation};
use super::*;

pub const TITLE_BAR_HEIGHT: u16 = 48;
pub const APPLICATION_TITLE: &str = "Mythic";

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
        ExcaliburContainer::default()
            .middle_top()
            .sharp()
            .build(
                Row::new()
                    .push(
                        label(APPLICATION_TITLE).headline().branding().build(),
                    )
                    .push(
                        Row::from_vec(
                            Navigation::items(&self.current_nav)
                                .into_iter()
                                .map(|(_icon, name, msg, selected)| {
                                    let style = match selected {
                                        true => |_theme: &Theme, _status: iced::widget::button::Status| iced::widget::button::Style {
                                            background: Some(
                                                ExcaliburColor::Transparent.color().into(),
                                            ),
                                            text_color: ExcaliburColor::Label(LabelColors::Primary)
                                                .color(),
                                            ..Default::default()
                                        },
                                        false => |_theme: &Theme, _status: iced::widget::button::Status| iced::widget::button::Style {
                                            background: Some(
                                                ExcaliburColor::Transparent.color().into(),
                                            ),
                                            text_color: ExcaliburColor::Label(
                                                LabelColors::Tertiary,
                                            )
                                            .color(),
                                            ..Default::default()
                                        },
                                    };

                                    button(iced::widget::text(name))
                                        .on_press(msg)
                                        .style(style)
                                        .padding(Sizes::Xs)
                                        .into()
                                })
                                .collect::<Vec<Element<'_, NavEvent>>>()
                                .into_iter()
                                .map(|x| x.map(|y| y.into()))
                                .collect(),
                        )
                        .width(Length::FillPortion(1))
                        .spacing(Sizes::Sm)
                        .align_y(Center),
                    )
                    .push(Row::new().width(Length::FillPortion(2)))
                    .push(
                        Column::new()
                            .align_x(End)
                            .push(button("profile").padding(Sizes::Xs).style(button::primary))
                            .width(Length::FillPortion(1)),
                    )
                    .spacing(Sizes::Lg)
                    .align_y(Center)
                    .width(Fill)
                    .height(Fill)
            )
            .padding(iced::Padding {
                top: Sizes::Sm.into(),
                right: Sizes::Md.into(),
                bottom: Sizes::Sm.into(),
                left: Sizes::Md.into(),
            })
            .height(TITLE_BAR_HEIGHT)
            .width(Fill)
            .into()
    }
}
