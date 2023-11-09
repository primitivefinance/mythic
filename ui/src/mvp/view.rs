use super::{column, *};

#[derive(Debug, Clone)]
pub enum Message {
    Empty,
    LogTrace,
}

pub fn app_layout<'a, T: Into<Element<'a, Message>>>(content: T) -> Element<'a, Message> {
    container(row![
        Space::with_width(Length::FillPortion(1)),
        column![container(column![content.into()].width(Length::Fill))
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)]
        .width(Length::FillPortion(8)),
        Space::with_width(Length::FillPortion(1))
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

pub fn terminal_view<'a>(logs: Vec<String>) -> Element<'a, Message> {
    let mut content = Column::new().push(Text::new("Terminal").size(28));
    content = content.push(firehose_view(logs.clone()));
    content.spacing(16).into()
}

pub fn firehose_view<'a>(logs: Vec<String>) -> Element<'a, Message> {
    let mut content = Column::new().push(Text::new("Firehose").size(28));

    let firehose = logs.iter().rev().fold(column![].spacing(2), |column, log| {
        column.push(firehose_log(log.clone()))
    });
    let firehose_content = container(scrollable(firehose))
        .style(FirehoseContainer::theme())
        .height(Length::Fixed(500.0))
        .max_width(Length::Fixed(750.0))
        .padding(16);

    let mut actions = row![].width(Length::Fill).height(Length::Fill).spacing(8);
    actions = actions
        .push(button(text("Start")).on_press(Message::Empty))
        .push(button(text("Stop")).on_press(Message::Empty))
        .push(button(text("Spawn")).on_press(Message::Empty))
        .push(button(text("Log debug trace")).on_press(Message::LogTrace));

    content = content.push(firehose_content).push(actions);
    content.spacing(16).into()
}

pub fn firehose_log<'a>(log: String) -> Element<'a, Message> {
    let firehose_element = text(log)
        .style(BLACK)
        .size(12)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Left);

    container(firehose_element)
        .style(FirehoseTrace::theme())
        .width(Length::Fill)
        .padding(4)
        .into()
}

pub struct FirehoseContainer;

impl container::StyleSheet for FirehoseContainer {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(LEVEL_2_COLOR)),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl FirehoseContainer {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(FirehoseContainer))
    }
}

pub struct FirehoseTrace;

impl container::StyleSheet for FirehoseTrace {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(LEVEL_4_COLOR)),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
            ..Default::default()
        }
    }
}

impl FirehoseTrace {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(FirehoseTrace))
    }
}
