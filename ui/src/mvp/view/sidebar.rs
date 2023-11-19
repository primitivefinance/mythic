use iced::{widget::Space, Color};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use super::{components::button::*, view::Message, *};

pub fn window_directory<'a>(windows: Vec<(Icon, String, Message, bool)>) -> Element<'a, Message> {
    let mut column = Column::new()
        .push(
            Row::new()
                .push(Space::with_width(Length::Fixed(Sizes::Xs as u32 as f32)))
                .push(
                    Column::new()
                        .push(text_label("Apps".to_string()))
                        .align_items(alignment::Alignment::Center),
                )
                .padding(Sizes::Sm as u16)
                .spacing(Sizes::Md as u16)
                .width(Length::Fill),
        )
        .spacing(Sizes::Sm as u16)
        .align_items(alignment::Alignment::Center);

    for (icon, name, msg, selected) in windows {
        let style = match selected {
            true => route_button_style(TABLE_COLUMN_BG_COLOR),
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
