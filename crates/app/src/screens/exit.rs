//! Empty is a default screen if no app is selected.

use iced::widget::Container;

use super::*;

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
    type AppMessage = app::Message;
    type ViewMessage = view::Message;

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    // todo: clean up the message piping in this...
    fn update(&mut self, message: Self::AppMessage) -> Command<Self::AppMessage> {
        match message {
            Self::AppMessage::View(view::Message::ConfirmExit) => {
                self.show_confirm = false;
                Command::perform(async { Ok::<(), ()>(()) }, |_| {
                    Self::AppMessage::View(view::Message::Exit)
                })
            }
            Self::AppMessage::View(view::Message::Exit) => {
                self.show_confirm = true;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let content = match self.show_confirm {
            true => Column::new()
                .push(secondary_label(
                    "Are you sure you want to exit?".to_string(),
                ))
                .push(
                    button("Yes, save and exit.")
                        .padding([10, 20])
                        .on_press(Self::ViewMessage::ConfirmExit),
                )
                .spacing(10)
                .align_items(alignment::Alignment::Center),
            false => Column::new().push(button("Save and exit.").on_press(Self::ViewMessage::Exit)),
        };

        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
