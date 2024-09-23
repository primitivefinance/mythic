use super::*;

pub mod empty;
pub mod exit;
pub mod portfolio;
pub mod settings;

pub trait MessageWrapper: Sized {
    type ParentMessage: From<Self>;
}

pub trait MessageWrapperView: Sized {
    type ParentMessage: From<Self> + Clone;
}

pub trait State {
    type ViewMessage: MessageWrapperView;

    type AppMessage: MessageWrapper;

    fn view(&self) -> Element<'_, Self::ViewMessage>;

    fn update(&mut self, _message: Self::AppMessage) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::AppMessage> {
        Subscription::none()
    }

    fn load(&self) -> Command<Self::AppMessage> {
        Command::none()
    }

    fn exit(&mut self) -> Command<Self::AppMessage> {
        Command::none()
    }
}

type WindowScreen = dyn State<ViewMessage = view::ViewMessage, AppMessage = app::AppMessage>;

pub struct Screen(pub Box<WindowScreen>);

impl Screen {
    pub fn new(state: Box<WindowScreen>) -> Self {
        Self(state)
    }

    pub fn view(&self) -> Element<'_, <WindowScreen as State>::ViewMessage> {
        self.0.view()
    }

    pub fn update(
        &mut self,
        message: <WindowScreen as State>::AppMessage,
    ) -> Command<<WindowScreen as State>::AppMessage> {
        self.0.update(message)
    }

    pub fn subscription(&self) -> Subscription<<WindowScreen as State>::AppMessage> {
        self.0.subscription()
    }

    pub fn load(&self) -> Command<<WindowScreen as State>::AppMessage> {
        self.0.load()
    }

    pub fn exit(&mut self) -> Command<<WindowScreen as State>::AppMessage> {
        self.0.exit()
    }
}
