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
    fn load(&self) -> Command<Message> {
        Command::none()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::View(view::Message::ConfirmExit) => {
                self.show_confirm = false;
                Command::perform(async { Ok::<(), ()>(()) }, |_| {
                    Message::View(view::Message::Exit)
                })
            }
            Message::View(view::Message::Exit) => {
                self.show_confirm = true;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        let content = match self.show_confirm {
            true => Column::new()
                .push(secondary_label(
                    "Are you sure you want to exit?".to_string(),
                ))
                .push(
                    button("Yes, save and exit.")
                        .padding([10, 20])
                        .on_press(view::Message::ConfirmExit),
                )
                .spacing(10)
                .align_items(alignment::Alignment::Center),
            false => Column::new().push(button("Save and exit.").on_press(view::Message::Exit)),
        };

        view::app_layout(
            &view::Page::Exit,
            Container::new(content)
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .into()
    }
}
