//! Renders a title, a list of route-able pages, and bookmarks.

use iced::{widget::Space, Color};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{components::button::*, *};

const TITLE: &str = "Excalibur";

/// Defines all the possible locations that can be directly routed to in the
/// app. For example, routing to a page will display that page. Routing to a
/// bookmark will route to a specific application state.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub enum Route {
    #[default]
    Empty,
    Page(Page),
    Bookmark(Bookmarks),
}

impl MessageWrapper for Route {
    type ParentMessage = app::Message;
}

/// For converting a `Route` into a `view::Message` that can be sent to the view
/// to update the UI.
impl MessageWrapperView for Route {
    type ParentMessage = view::Message;
}

impl From<Route> for <Route as MessageWrapper>::ParentMessage {
    fn from(msg: Route) -> Self {
        app::Message::View(view::Message::Route(msg))
    }
}

impl From<Route> for <Route as MessageWrapperView>::ParentMessage {
    fn from(msg: Route) -> Self {
        view::Message::Route(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct Sidebar {
    pub page: Page,
    pub bookmarks: Bookmarks,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            page: Page::Empty,
            bookmarks: Bookmarks::new(),
        }
    }
}

impl Sidebar {
    pub type ViewMessage = view::Message;

    /// Renders a section header with a label.
    pub fn section<'a>(&self, label: String) -> Row<'a, Self::ViewMessage> {
        Row::new()
            .push(Space::with_width(Length::Fixed(Sizes::Xs.into())))
            .push(
                Column::new()
                    .push(secondary_label(label))
                    .align_items(alignment::Alignment::Center),
            )
            .padding(Sizes::Sm)
            .spacing(Sizes::Md)
            .width(Length::Fill)
    }

    /// Renders the inner column below the sidebar's header section.
    pub fn layout<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let mut column = Column::new();
        column = column.push(self.section("Apps".to_string()));
        column = column.push(self.page.view().map(|x| x.into()));
        column = column.push(self.section("Bookmarks".to_string()));
        column = column.push(self.bookmarks.view().map(|x| x.into()));

        column
            .spacing(Sizes::Xs as u16)
            .align_items(alignment::Alignment::Center)
            .into()
    }
}

impl screens::State for Sidebar {
    type AppMessage = Route;

    /// Renders the full sidebar.
    fn view<'a>(&'a self) -> Element<'a, Self::ViewMessage> {
        let title = Column::new()
            .push(with_font(h1(TITLE.to_string())))
            .padding(Sizes::Lg as u16)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fill);

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
                        .push(self.layout())
                        .spacing(Sizes::Lg as u16)
                        .padding(Sizes::Xs as u16),
                )
                .spacing(Sizes::Md as u16),
        )
        .style(SidebarContainer::theme())
        .height(Length::Fill)
        .into()
    }
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
    pub type Message = Route;

    /// Icon, Name, Message, Selected
    pub type PageTab = (Icon, String, Self::Message, bool);

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

    pub fn icon(&self) -> Icon {
        match self {
            Page::Empty => Icon::TerminalFill,
            Page::Terminal => Icon::TerminalFill,
            Page::Execute => Icon::Wallet,
            Page::AddressBook => Icon::ShieldShaded,
            Page::Exit => Icon::X,
            Page::Developer => Icon::Thermometer,
            _ => Icon::Gear,
        }
    }

    pub fn tab(&self, active: &Page) -> Self::PageTab {
        let name = self.name();
        let icon = self.icon();

        (icon, name, Self::Message::Page(*self), *self == *active)
    }

    pub fn tabs(active: &Page) -> Vec<Self::PageTab> {
        let mut all = vec![
            Page::Terminal.tab(active),
            Page::Execute.tab(active),
            Page::AddressBook.tab(active),
            Page::Experimental.tab(active),
            Page::Exit.tab(active),
        ];

        if std::env::var("DEV_MODE").is_ok() {
            all.push(Page::Developer.tab(active));
        }

        all
    }

    pub fn view<'a>(&self) -> Element<'a, Self::Message> {
        let mut column = Column::new();
        for (icon, name, msg, selected) in Self::tabs(self) {
            let style = match selected {
                true => route_button_style(SELECTED_CONTAINER_COLOR),
                false => route_button_style(Color::TRANSPARENT),
            };

            column = column.push(
                button(
                    Row::new()
                        .push(Space::with_width(Length::Fixed(Sizes::Xs as u32 as f32)))
                        .push(text(icon_to_char(icon)).font(ICON_FONT))
                        .push(text(name))
                        .spacing(Sizes::Md as u16),
                )
                .width(Length::Fill)
                .on_press(msg)
                .style(style.as_custom())
                .padding(Sizes::Sm as u16),
            );
        }

        column
            .spacing(Sizes::Xs)
            .align_items(alignment::Alignment::Center)
            .into()
    }
}

/// todo: implement bookmark editing and better route management.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct Bookmarks {
    bookmarks: Vec<String>,
}

impl Bookmarks {
    pub fn new() -> Self {
        Self {
            bookmarks: vec!["Portfolio".to_string()],
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Route> {
        Column::with_children(
            self.bookmarks
                .iter()
                .map(|x| {
                    button(
                        Row::new()
                            .push(Space::with_width(Length::Fixed(Sizes::Xs as u32 as f32)))
                            .push(text(icon_to_char(Icon::TerminalFill)).font(ICON_FONT))
                            .push(text(x))
                            .spacing(Sizes::Md as u16),
                    )
                    .width(Length::Fill)
                    .on_press(Route::Page(Page::Developer))
                    .style(route_button_style(Color::TRANSPARENT).as_custom())
                    .padding(Sizes::Sm as u16)
                    .into()
                })
                .collect::<Vec<Element<'a, Route>>>(),
        )
        .into()
    }
}
