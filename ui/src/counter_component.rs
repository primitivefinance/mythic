use iced::{
    alignment::{self, Alignment},
    widget::{button, component, row, text, Component},
    Element, Length, Renderer,
};

use std::sync::Arc;

type HandlerFn<Msg> = Arc<Box<dyn Fn(Option<u32>) -> Msg + Send + Sync + 'static>>;

#[derive(Clone)]
pub struct Counter<Msg> {
    value: Option<u32>,
    on_change: HandlerFn<Msg>,
}

pub fn counter_state<Msg>(
    value: Option<u32>,
    on_change: impl Fn(Option<u32>) -> Msg + Send + Sync + 'static,
) -> Counter<Msg> {
    Counter::new(value, on_change)
}

impl<Msg> Counter<Msg> {
    pub fn new(
        value: Option<u32>,
        on_change: impl Fn(Option<u32>) -> Msg + Send + Sync + 'static,
    ) -> Self {
        Self {
            value,
            on_change: Arc::new(Box::new(on_change)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Increment,
    Decrement,
    InputChanged(String),
}

impl<Msg> Component<Msg, Renderer> for Counter<Msg> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Msg> {
        match event {
            Event::Increment => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_add(1),
            ))),
            Event::Decrement => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_sub(1),
            ))),
            Event::InputChanged(value) => {
                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    value.parse().ok().map(Some).map(self.on_change.as_ref())
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .width(40)
            .height(40)
            .on_press(on_press)
        };

        row![
            button("-", Event::Decrement),
            button("+", Event::Increment),
            button("refresh", Event::InputChanged("1".to_string()))
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .into()
    }
}

impl<'a, Msg> From<Counter<Msg>> for Element<'a, Msg, Renderer>
where
    Msg: 'a,
{
    fn from(counter: Counter<Msg>) -> Self {
        component(counter).into()
    }
}
