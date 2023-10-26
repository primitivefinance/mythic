use iced::alignment::{self, Alignment};
use iced::widget::{button, component, row, text, Component};
use iced::{Element, Length, Renderer};
pub struct Counter<Message> {
    value: Option<u32>,
    on_change: Box<dyn Fn(Option<u32>) -> Message>,
}

pub fn counter_state<Message>(
    value: Option<u32>,
    on_change: impl Fn(Option<u32>) -> Message + 'static,
) -> Counter<Message> {
    Counter::new(value, on_change)
}

impl<Message> Counter<Message> {
    pub fn new(value: Option<u32>, on_change: impl Fn(Option<u32>) -> Message + 'static) -> Self {
        Self {
            value,
            on_change: Box::new(on_change),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Increment,
    Decrement,
    InputChanged(String),
}

impl<Message> Component<Message, Renderer> for Counter<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
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

impl<'a, Message> From<Counter<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(counter: Counter<Message>) -> Self {
        component(counter).into()
    }
}
