use super::*;

pub struct SearchBar {
    query: String,
}

pub struct SearchBarScreen {
    search_bar: SearchBar,
}

impl SearchBarScreen {
    pub fn new() -> Self {
        Self {
            search_bar: SearchBar {
                query: String::new(),
            },
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        self.update(message)
    }
    pub fn view<'a>(&'a self) -> Element<'a, view::Message> {
        self.view()
    }
}

impl From<SearchBarScreen> for Screen {
    fn from(screen: SearchBarScreen) -> Self {
        Screen::new(Box::new(screen))
    }
}

impl State for SearchBarScreen {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::View(msg) => match msg {
                view::Message::SearchInputChanged(query) => {
                    self.search_bar.query = query;
                    Command::none()
                }
                view::Message::Empty => todo!(),
                view::Message::Exit => todo!(),
                view::Message::ConfirmExit => todo!(),
                view::Message::Simulation(_) => todo!(),
                view::Message::Settings(_) => todo!(),
                view::Message::Data(_) => todo!(),
                view::Message::Page(_) => todo!(),
                view::Message::Execution(_) => todo!(),
                view::Message::AddressBook(_) => todo!(),
                view::Message::CopyToClipboard(_) => todo!(),
            },
            _ => Command::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, view::Message> {
        view::app_layout(&view::Page::Explore, view::search::simple_view()).into()
    }

    // Implement other methods...
}
