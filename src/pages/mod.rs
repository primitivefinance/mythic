//! Traits for implementing sub-sections of the app with their own lifecycle and messages.

use super::*;

pub mod dashboard;
pub mod empty;
pub mod exit;
pub mod settings;

pub trait MessageWrapper: Sized {
    type ParentMessage: From<Self>;
}

pub trait MessageWrapperView: Sized {
    type ParentMessage: From<Self> + Clone;
}

pub trait Lifecycle {
    type ViewMessage: MessageWrapperView;

    type AppMessage: MessageWrapper;

    fn view(&self) -> Element<'_, Self::ViewMessage>;

    fn update(&mut self, _message: Self::AppMessage) -> Task<Self::AppMessage> {
        Task::none()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        Subscription::none()
    }

    fn load(&self) -> Task<Self::AppMessage> {
        Task::none()
    }

    fn exit(&mut self) -> Task<Self::AppMessage> {
        Task::none()
    }
}

type DynamicLifecycle =
    dyn Lifecycle<ViewMessage = view::ViewMessage, AppMessage = app::AppMessage>;

pub struct Page(pub Box<DynamicLifecycle>);

impl Page {
    pub fn new(state: Box<DynamicLifecycle>) -> Self {
        Self(state)
    }

    pub fn view(&self) -> Element<'_, <DynamicLifecycle as Lifecycle>::ViewMessage> {
        self.0.view()
    }

    pub fn update(
        &mut self,
        message: <DynamicLifecycle as Lifecycle>::AppMessage,
    ) -> Task<<DynamicLifecycle as Lifecycle>::AppMessage> {
        self.0.update(message)
    }

    pub fn subscription(&self) -> Subscription<<DynamicLifecycle as Lifecycle>::AppMessage> {
        self.0.subscription()
    }

    pub fn load(&self) -> Task<<DynamicLifecycle as Lifecycle>::AppMessage> {
        self.0.load()
    }

    pub fn exit(&mut self) -> Task<<DynamicLifecycle as Lifecycle>::AppMessage> {
        self.0.exit()
    }
}
