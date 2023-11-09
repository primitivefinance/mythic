use iced::{
    alignment,
    widget::{button, column, component, container, row, text, Component},
    Color, Element, Length, Renderer,
};

use super::footer::{self, Footer};
use super::styles;

#[derive(Debug, Clone)]
pub enum Event {
    Debug(String),
    AddWorkspace(String),
}

pub struct Sidebar {
    pub workspaces: Vec<String>,
    active_workspace: usize,
    footer: footer::Footer,
}

impl Sidebar {
    pub fn new(footer: Footer) -> Self {
        Self {
            workspaces: vec!["Workspace 1".to_string(), "Workspace 2".to_string()],
            active_workspace: 0,
            footer,
        }
    }

    pub fn get_active_workspace(&self) -> &String {
        &self.workspaces[self.active_workspace]
    }

    pub fn set_active_workspace(&mut self, index: usize) {
        if index < self.workspaces.len() {
            self.active_workspace = index;
        }
    }
}

impl<'a, Message> Component<Message, Renderer> for Sidebar
where
    Message: 'a,
{
    type Event = Event;
    type State = ();

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        // Set the active workspace
        match event {
            Event::Debug(workspace) => {
                for (i, w) in self.workspaces.clone().into_iter().enumerate() {
                    if w == workspace {
                        self.active_workspace = i;
                    }
                }
            }
            Event::AddWorkspace(workspace) => {
                // todo: need a more robust way to manage workspaces obviously...
                if !self.workspaces.contains(&workspace) {
                    self.workspaces.push(workspace);
                }
            }
        }

        None
    }

    fn view(&self, _: &Self::State) -> Element<Event, Renderer> {
        let mut workspaces = column![text(format!("Workspaces"))
            .size(16)
            .height(Length::Fixed(45.0))
            .vertical_alignment(alignment::Vertical::Center)]
        .width(Length::Fill)
        .align_items(alignment::Alignment::Center)
        .spacing(4)
        .padding(8);

        for (i, workspace) in self.workspaces.clone().into_iter().enumerate() {
            let black_rectangle_indicator = container(column![])
                .width(Length::Fixed(4.0))
                .height(Length::Fill)
                .style(Indicator::theme());

            let mut button_content = row![];

            if i == self.active_workspace {
                button_content = button_content.push(black_rectangle_indicator);
            }

            button_content = button_content.push(
                text(workspace.clone())
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill),
            );

            // Style the button with the Workspace button if the workspace is active
            let mut button = button(button_content)
                .height(Length::Fixed(60.0))
                .width(Length::Fill)
                .on_press(Event::Debug(workspace.clone()));

            if i == self.active_workspace {
                button = button.style(iced::theme::Button::Custom(Box::new(WorkspaceButton {})));
            } else {
                // Use the transparent button styling
                button = button.style(iced::theme::Button::Secondary);
            }

            workspaces = workspaces.push(button);
        }

        let add_workspace_button = button(row![text("New Workspace")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)])
        .on_press(Event::AddWorkspace("New Workspace".to_string()))
        .padding(10)
        .style(iced::theme::Button::Custom(Box::new(AddWorkspaceButton {})));

        let footer = self
            .footer
            .view()
            .map(|_| Event::Debug("footer press".into()));

        let mut sidebar_content = column![];

        // Sidebar has 80% of its height dedicated to workspace management, and the
        // remainder to the footer.
        sidebar_content = sidebar_content.push(
            column![workspaces, add_workspace_button]
                .height(Length::FillPortion(8))
                .align_items(alignment::Alignment::Center),
        );
        sidebar_content = sidebar_content.push(footer);

        container(sidebar_content.align_items(alignment::Alignment::Center))
            .style(styles::background::BackgroundContainer::theme())
            .width(Length::Fixed(200.0))
            .height(Length::Fill)
            .into()
    }
}

impl<'a, Message> From<Sidebar> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(sidebar: Sidebar) -> Self {
        component(sidebar).into()
    }
}

pub struct Indicator;

impl iced::widget::container::StyleSheet for Indicator {
    type Style = iced::Theme;

    fn appearance(&self, _: &<Self as container::StyleSheet>::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::BLACK)),
            ..Default::default()
        }
    }
}

impl Indicator {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Indicator))
    }
}

pub const BUTTON_BG: Color = Color::from_rgb(
    0xD9 as f32 / 255.0,
    0xD9 as f32 / 255.0,
    0xD9 as f32 / 255.0,
);

pub struct WorkspaceButton;

impl iced::widget::button::StyleSheet for WorkspaceButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::BLACK,
            background: Some(iced::Background::Color(BUTTON_BG)),
            border_radius: 5.0.into(),
            ..Default::default()
        }
    }
}

pub struct AddWorkspaceButton;

impl iced::widget::button::StyleSheet for AddWorkspaceButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::BLACK,
            border_radius: 5.0.into(),
            border_width: 2.0.into(),
            border_color: BUTTON_BG,
            ..Default::default()
        }
    }
}
