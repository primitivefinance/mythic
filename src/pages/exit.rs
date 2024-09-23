use iced::widget::Container;

use super::*;
use crate::components::system::label;

pub struct ExitScreen {
    pub show_confirm: bool,
}

impl ExitScreen {
    pub fn new(show_confirm: bool) -> Self {
        Self { show_confirm }
    }
}

impl From<ExitScreen> for Screen {
    fn from(screen: ExitScreen) -> Self {
        Screen::new(Box::new(screen))
    }
}

impl State for ExitScreen {
    type AppMessage = app::AppMessage;
    type ViewMessage = view::ViewMessage;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::ViewMessage::Root(message)) => match message {
                view::RootMessage::ConfirmExit => {
                    self.show_confirm = false;
                    Command::perform(async { Ok::<(), ()>(()) }, |_| {
                        Self::AppMessage::View(view::ViewMessage::Root(
                            view::RootMessage::SaveAndExit,
                        ))
                    })
                }
                view::RootMessage::SaveAndExit => {
                    self.show_confirm = true;
                    Command::none()
                }
                _ => Command::none(),
            },

            _ => Command::none(),
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
                .align_items(alignment::Alignment::Center),
            false => Column::new().push(label("Saving and exiting...").title2().build()),
        };

        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
