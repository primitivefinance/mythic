use iced::widget::Container;
use iced::Fill;

use super::*;
use crate::components::system::label;

pub struct ExitPage {
    pub show_confirm: bool,
}

impl ExitPage {
    pub fn new(show_confirm: bool) -> Self {
        Self { show_confirm }
    }
}

impl From<ExitPage> for Page {
    fn from(screen: ExitPage) -> Self {
        Page::new(Box::new(screen))
    }
}

impl Lifecycle for ExitPage {
    type AppMessage = app::AppMessage;
    type ViewMessage = view::ViewMessage;

    fn load(&self) -> Task<Self::AppMessage> {
        Task::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Task<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::ViewMessage::Root(message)) => match message {
                view::RootMessage::ConfirmExit => {
                    self.show_confirm = false;
                    Task::perform(async { Ok::<(), ()>(()) }, |_| {
                        Self::AppMessage::View(view::ViewMessage::Root(
                            view::RootMessage::SaveAndExit,
                        ))
                    })
                }
                view::RootMessage::SaveAndExit => {
                    self.show_confirm = true;
                    Task::none()
                }
                _ => Task::none(),
            },

            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let content = match self.show_confirm {
            true => Column::new()
                .push(label("Are you sure you want to exit?").secondary().build())
                .push(
                    button(label("Yes, save and exit.").build())
                        .padding([10, 20])
                        .on_press(Self::ViewMessage::Root(view::RootMessage::ConfirmExit)),
                )
                .spacing(Sizes::Sm)
                .align_x(alignment::Alignment::Center),
            false => Column::new().push(label("Saving and exiting...").title2().build()),
        };

        Container::new(content)
            .center_x(Fill)
            .center_y(Fill)
            .width(Fill)
            .height(Fill)
            .into()
    }
}
