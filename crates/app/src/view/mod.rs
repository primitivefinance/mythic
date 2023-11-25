use std::collections::{BTreeMap, HashMap};

use api::contacts;
use ethers::utils::format_ether;
use iced::widget::{Column, Container, Row};
use iced_aw::Icon;
use simulation::agents::SubscribedData;

use self::{control::control_panel, monitor::labeled_data_cards, sidebar::window_directory};
use super::{
    components::{containers::*, *},
    screens::address_book::AddressBookDisplay,
    terminal::{StateSubscription, StateSubscriptionStore},
    tracer::AppEventLayer,
    *,
};

pub mod address_book;
pub mod agent;
pub mod control;
pub mod execute;
pub mod monitor;
pub mod sidebar;

/// Messages emitted from user interaction with the settings.
#[derive(Debug, Clone)]
pub enum Settings {
    ToggleRealtime,
    ToggleFirehoseVisibility,
}

/// Messages emitted from user interaction with event_data components.
#[derive(Debug, Clone)]
pub enum Data {
    // for debugging...
    LogTrace,
    // todo: this needs a refactor
    UpdateWatchedValue(StateSubscriptionStore),
    AppEvent,
}

/// Root message for the Terminal component.
#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    Empty,
    Exit,
    ConfirmExit,
    Simulation(control::Operation),
    Settings(Settings),
    Data(Data),
    Page(Page),
    Execution(Execution),
    AddressBook(AddressBookViewMessage),
    CopyToClipboard(String),
    Experimental,
    Developer(developer::Message),
    CreatePortfolio(portfolio::create::Message),
}

impl MessageWrapper for Message {
    type ParentMessage = app::Message;
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::View(message.into())
    }
}

#[derive(Debug, Clone)]
pub enum AddressBookViewMessage {
    Add,
    AddressChanged(Option<String>),
    LabelChanged(Option<String>),
    CategoryChanged(contacts::Category),
    ClassChanged(contacts::Class),
    Remove((contacts::Category, contacts::ContactKey)),
    ResetForm,
    RouteTo(contacts::Category),
    ChangeDisplay(AddressBookDisplay),
}

#[derive(Debug, Clone)]
pub enum Execution {
    Form(execution::form::FormMessage),
    Simulate,
    Execute,
    Results,
    Reset,
}

impl From<Execution> for view::Message {
    fn from(execution: Execution) -> Self {
        view::Message::Execution(execution)
    }
}

impl From<Execution> for app::Message {
    fn from(execution: Execution) -> Self {
        app::Message::View(execution.into())
    }
}

pub fn app_layout<'a, T: Into<Element<'a, Message>>>(
    window: &'a Page,
    content: T,
) -> Element<'a, Message> {
    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(page_menu(window))
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(screen_layout(window, content))
                    .width(Length::FillPortion(5)),
            ),
    )
    .style(BackgroundContainerTheme::theme())
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
        .padding(Sizes::Xl as u16)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default)]
pub enum Page {
    #[default]
    Empty,
    Terminal,
    Execute,
    AddressBook,
    Exit,
    Experimental,
    Developer,
}

impl Page {
    pub fn name(&self) -> String {
        match self {
            Page::Empty => "Select App".to_string(),
            Page::Terminal => "Terminal".to_string(),
            Page::Execute => "Execute".to_string(),
            Page::AddressBook => "Address Book".to_string(),
            Page::Exit => "Quit".to_string(),
            _ => "Experimental".to_string(),
        }
    }
}

// todo: this needs to be broken down into more components
// also developer page should be hidden, along with any other pages
// we should be able to start the app in dev mode that does some things to help
// development.
pub fn page_menu<'a>(menu: &Page) -> Container<'a, Message> {
    let name = "Excalibur".to_string();
    let title = Column::new()
        .push(with_font(h1(name)))
        .padding(Sizes::Lg as u16)
        .align_items(alignment::Alignment::Center)
        .width(Length::Fill);

    let is_dev_mode = std::env::var("DEV_MODE").is_ok();

    let mut windows = vec![
        (
            Icon::TerminalFill,
            "Terminal".to_string(),
            Message::Page(Page::Terminal),
            menu == &Page::Terminal,
        ),
        (
            Icon::Wallet,
            "Execute".to_string(),
            Message::Page(Page::Execute),
            menu == &Page::Execute,
        ),
        (
            Icon::ShieldShaded,
            "Address Book".to_string(),
            Message::Page(Page::AddressBook),
            menu == &Page::AddressBook,
        ),
        (
            Icon::Gear,
            "Experimental".to_string(),
            Message::Page(Page::Experimental),
            menu == &Page::Experimental,
        ),
        (
            Icon::X,
            "Quit".to_string(),
            Message::Page(Page::Exit),
            menu == &Page::Exit,
        ),
    ];

    if is_dev_mode {
        windows.push((
            Icon::Thermometer,
            "Developer".to_string(),
            Message::Page(Page::Developer),
            menu == &Page::Developer,
        ));
    }

    let apps = window_directory(windows);

    Container::new(
        Column::new()
            .push(
                Column::new().push(title).push(
                    Container::new(Column::new())
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0))
                        .style(ContainerBlackBg::theme()),
                ),
            )
            .push(
                Column::new()
                    .push(apps)
                    .spacing(Sizes::Lg as u16)
                    .padding(Sizes::Xs as u16),
            )
            .spacing(Sizes::Md as u16),
    )
    .style(SidebarContainer::theme())
    .height(Length::Fill)
}

pub fn terminal_layout<'a>(
    realtime: bool,
    state_data: StateSubscriptionStore,
) -> Element<'a, Message> {
    let cloned = state_data.clone();
    let control_view = control_panel(realtime);
    let state_view = state_render(cloned);
    let content_row = Row::new()
        .push(Column::new().push(state_view))
        .width(Length::Fill);

    Column::new()
        .push(
            Card::new(container(control_view))
                .padding(Sizes::Md as u16)
                .width(Length::Fill),
        )
        .push(content_row)
        .spacing(Sizes::Lg as u16)
        .padding(Sizes::Xl as u16)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

type SubscriptionViewWrapper = BTreeMap<u64, BTreeMap<String, Vec<(String, String)>>>;

// todo: lot of logic to handle in here, maybe we parse it further up in the
// app?
fn state_render<'a>(state_data: StateSubscriptionStore) -> Element<'a, Message> {
    let mut agent_data: SubscriptionViewWrapper = BTreeMap::new();
    let mut monitored_data: SubscriptionViewWrapper = BTreeMap::new();

    let empty_map = BTreeMap::new();

    let cloned: StateSubscriptionStore = state_data.clone();

    for (_i, (world_id, world_data)) in cloned.into_iter().enumerate() {
        // todo: handle rendering for multiple worlds, should probably be grouped.

        let cloned_world: HashMap<String, StateSubscription> = world_data.clone();

        // Exit early if world data is empty
        if cloned_world.is_empty() {
            continue;
        }

        for (agent_name, agent) in cloned_world.into_iter() {
            let label = agent.label.clone();
            let category: AppEventLayer = agent.category.clone();
            let logs: Vec<SubscribedData> = agent.logs.clone();

            // Insert the agent label if it has non empty state subscriptions
            if agent.logs.is_empty() {
                continue;
            }

            match category {
                AppEventLayer::Agent => {
                    // Each agent should have one tuple element of (name, label)
                    agent_data
                        .entry(world_id)
                        .or_insert(empty_map.clone())
                        .entry(agent_name.clone())
                        .or_insert(vec![])
                        .push(("name".to_string(), label.clone()));
                    for log in logs {
                        let name = log.name.clone();
                        let value = log.data.clone();

                        // todo: this can easily be 0...
                        let value_uint = value.into_uint().unwrap_or_default();
                        // todo: this is hardcoded parsing... it could become wrong easily.
                        let formatted = format_ether(value_uint).parse::<f64>().unwrap_or_default();
                        // truncated
                        let truncated = format!("{:.2}", formatted);

                        agent_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_insert(vec![])
                            .push((name, truncated));
                    }
                }
                _ => {
                    for log in logs {
                        let name = format!("{}", log.name);
                        let value = log.data.clone();
                        // todo: this can easily be 0...
                        let value_uint = value.into_uint().unwrap_or_default();
                        // todo: this is hardcoded parsing... it could become wrong easily.
                        let formatted = format_ether(value_uint).parse::<f64>().unwrap_or_default();
                        // truncated
                        let truncated = format!("{:.2}", formatted);

                        monitored_data
                            .entry(world_id)
                            .or_insert(empty_map.clone())
                            .entry(agent_name.clone())
                            .or_insert(vec![])
                            .push((name, truncated));
                    }
                }
            }
        }
    }

    let mut agent_groups = Column::new().spacing(16);

    for (_world_id, world_data) in agent_data.into_iter() {
        let mut agent_cards = Vec::new();
        for (_agent_name, agent) in world_data.into_iter() {
            agent_cards.push(agent);
        }

        agent_groups = agent_groups.push(agent_card_grid(agent_cards, 4));
    }

    let mut monitored_groups = Column::new().spacing(Sizes::Md as u16);

    for (_world_id, world_data) in monitored_data.into_iter() {
        let mut monitored_cards = Vec::new();
        for (_agent_name, agent) in world_data.into_iter() {
            monitored_cards.push(agent);
        }

        let first_elemn = monitored_cards.clone()[0].clone();
        monitored_groups = monitored_groups.push(labeled_data_cards(first_elemn, 6));
    }

    let agent_groups_title = data_item("Agents".to_string()).size(36);

    scrollable(
        Column::new()
            .push(data_item("Protocol".to_string()).size(36))
            .push(monitored_groups)
            .push(agent_groups_title)
            .push(agent_groups)
            .spacing(Sizes::Md as u16)
            .width(Length::Fill),
    )
    .into()
}

/// Renders a grid of agents cards, with a maximum amount of cards per row.
fn agent_card_grid<'a>(data: Vec<Vec<(String, String)>>, max: usize) -> Element<'a, Message> {
    let mut content = Column::new();
    let mut row = Row::new().spacing(Sizes::Lg as u16);
    let mut i = 0;
    for card in data.into_iter() {
        row = row.push(agent::agent_card(card, false));
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

#[allow(dead_code)]
fn mock_agent_card() -> Element<'static, Message> {
    agent::agent_card(
        vec![
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
        ],
        false,
    )
}

#[allow(dead_code)]
fn mock_monitor_group() -> Element<'static, Message> {
    labeled_data_cards(
        vec![
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
            ("name".to_string(), "agent".to_string()),
        ],
        3,
    )
}
