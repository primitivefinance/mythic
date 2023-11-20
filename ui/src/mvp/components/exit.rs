use iced::widget::{component, Component};

use super::*;

pub fn create_exit_component<Message: Clone>(
    show_confirm: bool,
    on_exit: Message,
) -> ExitComponent<Message> {
    ExitComponent::new(show_confirm, on_exit)
}

/// Individual component for managing inputs with string values.
/// todo: better error handling and tracing
pub struct ExitComponent<Message: Clone> {
    show_confirm: bool,
    message: Message,
}

#[derive(Debug, Clone)]
pub enum ExitMessage {
    Confirm,
    Exit,
}

impl<Message: Clone> ExitComponent<Message> {
    pub fn new(show_confirm: bool, message: Message) -> Self {
        Self {
            show_confirm,
            message,
        }
    }
}

impl<Message: Clone> Component<Message, Renderer> for ExitComponent<Message> {
    type State = ();
    type Event = ExitMessage;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Self::Event::Confirm => {
                self.show_confirm = false;

                Some(self.message.clone())
            }
            Self::Event::Exit => {
                self.show_confirm = true;
                None
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        let exit = if self.show_confirm {
            Column::new()
                .push(tertiary_label("Are you sure you want to exit?".to_string()))
                .push(
                    button("Yes, exit now")
                        .padding([10, 20])
                        .on_press(Self::Event::Confirm),
                )
        } else {
            Column::new()
                .push(tertiary_label("Click the button to exit".to_string()))
                .push(button("Exit").padding([10, 20]).on_press(Self::Event::Exit))
        }
        .spacing(10)
        .align_items(alignment::Alignment::Center);

        exit.into()
    }
}

impl<'a, Event> From<ExitComponent<Event>> for Element<'a, Event, Renderer>
where
    Event: 'a + Clone,
{
    fn from(exit_component: ExitComponent<Event>) -> Self {
        component(exit_component).into()
    }
}
