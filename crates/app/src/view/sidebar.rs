use iced::{widget::Space, Color};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{components::button::*, view::Message, *};

pub fn layout<'a>(pages: &Page) -> Container<'a, view::Message> {
    let name = "Excalibur".to_string();
    let title = Column::new()
        .push(with_font(h1(name)))
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
                    .push(pages.view())
                    .spacing(Sizes::Lg as u16)
                    .padding(Sizes::Xs as u16),
            )
            .spacing(Sizes::Md as u16),
    )
    .style(SidebarContainer::theme())
    .height(Length::Fill)
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

/// Icon, Name, Message, Selected
type PageTab = (Icon, String, Message, bool);

impl Page {
    pub type Message = view::Message;

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

    pub fn tab(&self, active: &Page) -> PageTab {
        let name = self.name();
        let icon = self.icon();

        (icon, name, Message::Page(*self), *self == *active)
    }

    pub fn tabs(active: &Page) -> Vec<PageTab> {
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
        let windows = Page::tabs(self);
        let mut column = Column::new()
            .push(
                Row::new()
                    .push(Space::with_width(Length::Fixed(Sizes::Xs as u32 as f32)))
                    .push(
                        Column::new()
                            .push(secondary_label("Apps".to_string()))
                            .align_items(alignment::Alignment::Center),
                    )
                    .padding(Sizes::Sm as u16)
                    .spacing(Sizes::Md as u16)
                    .width(Length::Fill),
            )
            .spacing(Sizes::Xs as u16)
            .align_items(alignment::Alignment::Center);

        for (icon, name, msg, selected) in windows {
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

        column.into()
    }
}
