//! Renders a title, a list of route-able pages, and bookmarks.
//! The sidebar is the main navigation for the app.

use iced::{widget::Space, Color};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{components::button::*, *};
use crate::components::system::label;

#[allow(dead_code)]
const SYMBOL: &str = "φ";
const TITLE: &str = "Excalibur";

/// Defines all the possible locations that can be directly routed to in the
/// app. For example, routing to a page will display that page. Routing to a
/// bookmark will route to a specific application state.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum Route {
    #[default]
    Empty,
    Page(Page),
    Bookmarks(Bookmarks),
    Open(Location),
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum Location {
    #[default]
    Empty,
    /// Routes to the portfolio dashboard with the given portfolio name.
    Portfolio(String),
}

impl MessageWrapper for Route {
    type ParentMessage = app::AppMessage;
}

/// For converting a `Route` into a `view::Message` that can be sent to the view
/// to update the UI.
impl MessageWrapperView for Route {
    type ParentMessage = view::ViewMessage;
}

impl From<Route> for <Route as MessageWrapper>::ParentMessage {
    fn from(msg: Route) -> Self {
        app::AppMessage::View(view::ViewMessage::Root(view::RootMessage::Route(msg)))
    }
}

impl From<Route> for <Route as MessageWrapperView>::ParentMessage {
    fn from(msg: Route) -> Self {
        view::ViewMessage::Root(view::RootMessage::Route(msg))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct Sidebar {
    pub state: Route,
    pub page: Page,
    pub bookmarks: Bookmarks,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            state: Route::Empty,
            page: Page::Empty,
            bookmarks: Bookmarks::new(),
        }
    }
}

impl Sidebar {
    /// Renders a section header with a label.
    #[allow(dead_code)]
    pub fn section<'a>(&self, header: String) -> Row<'a, view::ViewMessage> {
        Row::new()
            .push(Space::with_width(Length::Fixed(Sizes::Xs.into())))
            .push(
                Column::new()
                    .push(label(header).tertiary().build())
                    .align_items(alignment::Alignment::Center),
            )
            .padding(Sizes::Sm)
            .spacing(Sizes::Md)
            .width(Length::Fill)
    }

    /// Renders the inner column below the sidebar's header section.
    pub fn layout(&self) -> Element<'_, view::ViewMessage> {
        let mut column = Column::new();
        column = column.push(self.page.view().map(|x| x.into()));
        column
            .spacing(Sizes::Xs)
            .align_items(alignment::Alignment::Center)
            .into()
    }
}

impl controller::State for Sidebar {
    type AppMessage = Route;
    type ViewMessage = view::ViewMessage;

    fn update(&mut self, message: Route) -> Command<Route> {
        self.state = message.clone();

        match message {
            Route::Page(page) => {
                self.page = page;
                Command::none()
            }
            Route::Bookmarks(bookmark) => {
                self.bookmarks = bookmark;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    /// Renders the full sidebar.
    fn view(&self) -> Element<'_, Self::ViewMessage> {
        let title = Column::new()
            .push(
                Row::new()
                    .spacing(Sizes::Sm)
                    .align_items(alignment::Alignment::Center)
                    .push(label(TITLE).title3().branding().build()),
            )
            .padding(Sizes::Lg)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fill);

        Container::new(
            Column::new()
                .push(
                    Column::new().push(title).push(
                        Container::new(Column::new())
                            .width(Length::Fill)
                            .height(Length::Fixed(1.0))
                            .style(
                                ExcaliburContainer::default()
                                    .background_iced(Color::BLACK)
                                    .theme(),
                            ),
                    ),
                )
                .push(
                    Column::new()
                        .push(self.layout())
                        .spacing(Sizes::Lg)
                        .padding(Sizes::Xs),
                )
                .spacing(Sizes::Md),
        )
        .height(Length::Fill)
        .into()
    }
}

/// Defines all the possible pages that can be routed to in the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum Page {
    Empty,
    #[default]
    Portfolio,
    Settings,
    Exit,
}

/// Icon, Name, Message, Selected
pub type PageTab = (Icon, String, Route, bool);

impl Page {
    /// Returns the name of the page.
    pub fn name(&self) -> String {
        match self {
            Page::Empty => "Select App".to_string(),
            Page::Portfolio => "Portfolio".to_string(),
            Page::Settings => "Settings".to_string(),
            Page::Exit => "Exit".to_string(),
        }
    }

    /// Returns the icon of the page.
    pub fn icon(&self) -> Icon {
        match self {
            Page::Empty => Icon::TerminalFill,
            Page::Portfolio => Icon::Wallet,
            Page::Settings => Icon::Gear,
            Page::Exit => Icon::X,
        }
    }

    pub fn tab(&self, active: &Page) -> PageTab {
        let name = self.name();
        let icon = self.icon();

        (icon, name, Route::Page(*self), *self == *active)
    }

    pub fn tabs(active: &Page) -> Vec<PageTab> {
        let all = vec![
            Page::Portfolio.tab(active),
            Page::Settings.tab(active),
            Page::Exit.tab(active),
        ];

        all
    }

    /// Generates the view for the sidebar navigation.
    /// It creates a column of buttons, each representing a page in the
    /// application. For each page, it creates a button with an icon, name,
    /// and a message that is sent when the button is pressed. If the page
    /// is currently selected, the button is styled differently to indicate
    /// this. The function returns an Element that can be displayed in the
    /// user interface.
    pub fn view<'a>(&self) -> Element<'a, Route> {
        let mut column = Column::new();
        for (icon, name, msg, selected) in Self::tabs(self) {
            let style = match selected {
                true => route_button_style(Color::TRANSPARENT)
                    .hovered()
                    .background(Some(Color::TRANSPARENT.into())),
                false => route_button_style(Color::TRANSPARENT),
            };

            let mut app_name = label(name);

            if !selected {
                app_name = app_name.secondary();
            }

            column = column.push(
                button(
                    Row::new()
                        .push(Space::with_width(Length::Fixed(Sizes::Xs.into())))
                        .push(label(icon_to_char(icon)).icon().build())
                        .push(app_name.build())
                        .spacing(Sizes::Md),
                )
                .width(Length::Fill)
                .on_press(msg)
                .style(style.as_custom())
                .padding(Sizes::Sm),
            );
        }

        column
            .spacing(Sizes::Xs)
            .align_items(alignment::Alignment::Center)
            .into()
    }
}

/// todo: implement bookmark editing and better route management.
/// todo: currently not used.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub struct Bookmarks {
    current: String,
    bookmarks: Vec<String>,
}

impl Bookmarks {
    // todo: this is hacky, but it works for now.
    pub const PORTFOLIO_URL: &'static str = "portfolio";
    pub const PORTFOLIO_EXTENSION: &'static str = ".portfolio.json";

    pub fn new() -> Self {
        let bookmarks = vec!["Main.portfolio.json".to_string()];
        Self {
            current: bookmarks[0].clone(),
            bookmarks,
        }
    }

    pub fn bookmark_route(url: &String) -> Route {
        match url {
            x if x.contains(Self::PORTFOLIO_URL) => Route::Open(Location::Portfolio(
                x.replace(Self::PORTFOLIO_EXTENSION, ""),
            )),
            _ => Route::Empty,
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Route> {
        Column::with_children(
            self.bookmarks
                .iter()
                .map(|x| {
                    button(
                        Row::new()
                            .push(Space::with_width(Length::Fixed(Sizes::Xs.into())))
                            .push(text(icon_to_char(Icon::TerminalFill)).font(ICON_FONT))
                            .push(text(x))
                            .spacing(Sizes::Md),
                    )
                    .width(Length::Fill)
                    .on_press(Self::bookmark_route(x))
                    .style(route_button_style(Color::TRANSPARENT).as_custom())
                    .padding(Sizes::Sm)
                    .into()
                })
                .collect::<Vec<Element<'a, Route>>>(),
        )
        .into()
    }
}
