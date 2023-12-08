use std::collections::HashMap;

use ethers::abi::Token;
use iced::widget::{Column, Container, Row};

use self::sidebar::Page;
use super::{
    components::{containers::*, *},
    tracer::AppEventLayer,
    *,
};
use crate::screens::State;

pub mod sidebar;

/// Root message for the Terminal component.
#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    // Exit application
    Exit,
    // Confirm exit application
    ConfirmExit,
    // Route to a new page.
    Route(sidebar::Route),
    // Copy to clipboard.
    CopyToClipboard(String),
    // Specific window messages.
    Portfolio(portfolio::Message),
    Settings(settings::Message),
    Experimental(experimental::Message),
}

impl MessageWrapperView for Message {
    type ParentMessage = Message;
}

impl MessageWrapper for Message {
    type ParentMessage = app::Message;
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::View(message)
    }
}

const BG_2: iced::Color = iced::Color::from_rgb(
    0x04 as f32 / 255.0,
    0x04 as f32 / 255.0,
    0x04 as f32 / 255.0,
);

pub fn app_layout<'a, T: Into<Element<'a, Message>>>(
    menu: &'a sidebar::Sidebar,
    content: T,
) -> Element<'a, Message> {
    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(menu.view())
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(screen_layout(&menu.page, content))
                    .width(Length::FillPortion(5)),
            ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

/// For rendering content inside a screen that implements [`State`].
pub fn screen_layout<'a, T: Into<Element<'a, Message>>>(
    window: &'a Page,
    content: T,
) -> Element<'a, Message> {
    Container::new(screen_window(window, content))
        .center_x()
        .center_y()
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(Sizes::Xl)
        .style(CustomContainer::theme(Some(BG_2.into())))
        .into()
}

#[derive(Debug, Clone)]
pub struct SubscribedData {
    pub name: String,
    pub data: Token,
}

impl SubscribedData {
    pub fn new(name: String, data: Token) -> Self {
        Self { name, data }
    }
}

#[derive(Debug, Clone)]
pub struct StateSubscription {
    pub logs: Vec<SubscribedData>,
    pub label: String,
    pub category: AppEventLayer,
    pub id: u64,
}

pub type StateSubscriptionStore = HashMap<u64, HashMap<String, StateSubscription>>;

/// Renders a grid of agents cards, with a maximum amount of cards per row.
pub fn agent_card_grid<'a, Message>(
    data: Vec<Vec<(String, String)>>,
    max: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new();
    let mut row = Row::new().spacing(Sizes::Lg as u16);
    let mut i = 0;
    for card in data.into_iter() {
        row = row.push(labeled_data_cards(card, 4));
        i += 1;
        if i == max {
            content = content.push(row);
            row = Row::new().spacing(Sizes::Lg as u16);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(Sizes::Lg as u16).into()
}

/// Renders a single piece of labeled data in a container with a panel
/// background and padding.
pub fn labeled_data_card<'a, Message>(
    label: String,
    data: String,
    _max_width: u16,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new()
        .push(labeled_data(label, data))
        .width(Length::Fixed(100.0));
    content = content.spacing(Sizes::Sm as u16);
    Card::new(container(content))
        .padding(Sizes::Md as u16)
        .into()
}

/// Renders a group of labeled data cards in a row with a maximum amount of
/// elements per row. If the total amount of elements exceeds the maximum, it
/// will push a new row inside the column. There is a group label rendered above
/// the rows.
pub fn labeled_data_cards<'a, Message>(
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = Column::new();

    let mut row = Row::new().spacing(Sizes::Sm as u16);
    let mut i = 0;
    for (label, data) in data {
        row = row.push(labeled_data_card(label, data, 200));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(Sizes::Sm as u16);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(16).into()
}
