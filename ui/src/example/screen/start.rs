//! Renders the start screen for an application, with a banner image and start
//! button. Also renders the version info and any relevant system information.

use iced::{
    alignment::{self, Alignment},
    widget::{button, column, component, text, Component},
    Element, Length, Renderer,
};

use super::*;

/// A view for the first screen of the application.
pub struct StartScreen<Msg> {
    on_start: Box<dyn Fn() -> Msg>,
}

/// Events handled by the start screen view.
#[derive(Clone)]
pub enum Event {
    Deploy,
}

/// Takes a specific message to emit for this view, which is triggered by the
/// start button in this view.
impl<Msg> StartScreen<Msg> {
    pub fn new(on_press: impl Fn() -> Msg + 'static) -> Self {
        Self {
            on_start: Box::new(on_press),
        }
    }
}

/// Implementation of the iced component.
impl<Msg> Component<Msg, Renderer> for StartScreen<Msg> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Msg> {
        match event {
            Event::Deploy => Some((self.on_start)()),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .height(40)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .on_press(on_press)
        };

        // Layout should be:
        // Banner centered
        // Start button centered under banner
        // Footer has system info and version info
        // with spacing between each element
        let banner = banner::banner(400);

        let start_button = button("Start Excalibur", Event::Deploy);

        let footer = text(format!("Version: {}", env!("CARGO_PKG_VERSION"),))
            .width(Length::Fill)
            .height(Length::Fixed(30.0))
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center);

        column!()
            .push(banner)
            .push(start_button)
            .push(footer)
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }
}

impl<'a, Msg> From<StartScreen<Msg>> for Element<'a, Msg, Renderer>
where
    Msg: 'a,
{
    fn from(start: StartScreen<Msg>) -> Self {
        component(start).into()
    }
}
