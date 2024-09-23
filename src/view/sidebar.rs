use iced::{widget::Space, Color};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{components::button::*, *};
use crate::components::system::label;

#[allow(dead_code)]
const TITLE: &str = "Mythic";

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum Route {
    #[default]
    Empty,
    Page(Page),
    Open(Location),
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum Location {
    #[default]
    Empty,
    Portfolio(String),
}

impl MessageWrapper for Route {
    type ParentMessage = app::AppMessage;
}

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
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            state: Route::Empty,
            page: Page::Empty,
        }
    }
}

impl Sidebar {
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

    pub fn layout(&self) -> Element<'_, view::ViewMessage> {
        let mut column = Column::new();
        column = column.push(self.page.view().map(|x| x.into()));
        column
            .spacing(Sizes::Xs)
            .align_items(alignment::Alignment::Center)
            .into()
    }
}

impl pages::State for Sidebar {
    type AppMessage = Route;
    type ViewMessage = view::ViewMessage;

    fn update(&mut self, message: Route) -> Command<Route> {
        self.state = message.clone();

        match message {
            Route::Page(page) => {
                self.page = page;
                Command::none()
            }
            _ => Command::none(),
        }
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub enum Page {
    Empty,
    #[default]
    Portfolio,
    Settings,
    Exit,
}

pub type PageTab = (Icon, String, Route, bool);

impl Page {
    pub fn name(&self) -> String {
        match self {
            Page::Empty => "Select App".to_string(),
            Page::Portfolio => "Portfolio".to_string(),
            Page::Settings => "Settings".to_string(),
            Page::Exit => "Exit".to_string(),
        }
    }

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
