use std::time::Duration;

use serde::{Deserialize, Serialize};
use tracing::Span;

use super::{
    controller::{empty::EmptyScreen, exit::ExitScreen, Screen},
    *,
};
use crate::{
    components::system::{label, ExcaliburColor},
    controller::{portfolio::PortfolioRoot, settings::SettingsScreen, State},
    middleware::ExcaliburMiddleware,
    model::{
        contacts::{self, ContactValue},
        rpcs::RPCValue,
        user::Saveable,
    },
    view::sidebar::Sidebar,
};

pub fn app_span() -> Span {
    tracing::debug_span!("App")
}

#[derive(Debug, Default)]
pub enum AppMessage {
    #[default]
    Empty,
    Load,
    QuitReady,
    View(view::ViewMessage),
    UpdateUser(UserProfileMessage),
    SwitchWindow(view::sidebar::Route),
    ModelSyncResult(Result<Model, Arc<anyhow::Error>>),
}

#[derive(Debug)]
pub enum UserProfileMessage {
    SaveAnvilSnapshot(anyhow::Result<AnvilSave>),
    AddAddress(String, Address, contacts::Category),
    RemoveAddress(String, contacts::Category),
    ClearAddresses(contacts::Category),
    AddRPC(RPCValue),
    RemoveRPC(String),
    ClearRPCs,
}

pub type RootMessage = AppMessage;
pub type RootViewMessage = view::ViewMessage;

impl MessageWrapper for AppMessage {
    type ParentMessage = AppMessage;
}

impl From<UserProfileMessage> for AppMessage {
    fn from(message: UserProfileMessage) -> Self {
        Self::UpdateUser(message)
    }
}

pub struct Windows {
    pub screen: Screen,
    pub sidebar: Sidebar,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: EmptyScreen::new().into(),
            sidebar: Sidebar::new(),
        }
    }
}

impl Windows {
    pub fn new(screen: Screen, sidebar: Sidebar) -> Self {
        Self { screen, sidebar }
    }
}

pub struct App {
    pub client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    pub model: Model,
    pub windows: Windows,

    pub app_clock: AppClock,
}

impl App {
    pub fn new(
        model: Model,
        client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    ) -> (Self, Command<AppMessage>) {
        let dashboard = PortfolioRoot::new(Some(client.clone()), model.clone()).into();
        let mut sidebar = Sidebar::new();
        sidebar.page = view::sidebar::Page::Portfolio;
        (
            Self {
                client,
                model,
                windows: Windows::new(dashboard, sidebar),
                app_clock: AppClock::new(),
            },
            Command::perform(async {}, |_| AppMessage::Load),
        )
    }

    pub fn load(&mut self) -> Command<AppMessage> {
        let cmds = vec![
            self.windows.sidebar.load().map(|x| x.into()),
            self.windows.screen.load().map(|x| x),
        ];
        Command::batch(cmds)
    }

    pub fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        self.app_clock.update();

        app_span().in_scope(|| match message {
            AppMessage::Load => self.load(),
            AppMessage::QuitReady => Command::none(),
            AppMessage::ModelSyncResult(Ok(model)) => {
                self.model = model.clone();

                self.windows
                    .screen
                    .update(AppMessage::ModelSyncResult(Ok(model)))
            }
            AppMessage::ModelSyncResult(Err(e)) => {
                tracing::error!(
                    "Critical failure - sync model threw an error while updating: {:?}",
                    e
                );
                Command::none()
            }
            AppMessage::UpdateUser(msg) => self.update_user(msg),
            AppMessage::View(view::ViewMessage::Root(msg)) => match msg {
                view::RootMessage::ModelSyncRequest => self.sync_model(),
                view::RootMessage::Route(route) => self.switch_window(&route),
                view::RootMessage::CopyToClipboard(contents) => iced::clipboard::write(contents),
                view::RootMessage::SaveAndExit => self.exit(),
                view::RootMessage::Empty => Command::none(),
                view::RootMessage::ConfirmExit => {
                    tracing::debug!("Confirming exit");
                    self.windows
                        .screen
                        .update(AppMessage::View(view::ViewMessage::Root(msg)))
                        .map(|x| x)
                }
            },

            _ => self.windows.screen.update(message),
        })
    }

    pub fn view(&self) -> Element<AppMessage> {
        view::app_layout(
            &self.app_clock,
            &self.windows.sidebar,
            self.windows.screen.view(),
        )
        .map(AppMessage::View)
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        self.windows.screen.subscription()
    }

    pub fn exit(&mut self) -> Command<AppMessage> {
        let result = self.model.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        let mut commands = Vec::new();
        let cmd = self.windows.screen.exit();
        commands.push(cmd);

        if self.client.anvil.is_some() {
            let cmd = Command::perform(
                save_snapshot(self.client.clone()),
                UserProfileMessage::SaveAnvilSnapshot,
            )
            .map(AppMessage::UpdateUser);
            commands.push(cmd);
        }

        Command::batch(commands)
    }

    fn sync_model(&mut self) -> Command<AppMessage> {
        let model = self.model.clone();
        let provider = self.client.get_client();
        Command::perform(
            async move {
                let mut model = model;
                model.update(provider).await?;
                Ok(model)
            },
            AppMessage::ModelSyncResult,
        )
    }

    fn update_user(&mut self, message: UserProfileMessage) -> Command<AppMessage> {
        let model = &mut self.model;
        match message {
            UserProfileMessage::SaveAnvilSnapshot(snapshot) => {
                tracing::debug!("Saving anvil snapshot to profile");
                match snapshot {
                    Ok(snapshot) => {
                        self.model.user.anvil_snapshot = Some(snapshot);
                        if let Err(e) = self.model.save() {
                            tracing::error!("Failed to save anvil snapshot to file: {:?}", e);
                        } else {
                            tracing::debug!("Saved anvil snapshot to file");
                        }
                    }
                    Err(e) => tracing::error!("Failed to save anvil snapshot: {:?}", e),
                }

                return Command::perform(async {}, |_| AppMessage::QuitReady);
            }
            UserProfileMessage::AddAddress(name, address, category) => {
                model.user.contacts.add(
                    address,
                    ContactValue {
                        label: name,
                        ..Default::default()
                    },
                    category,
                );
            }
            UserProfileMessage::RemoveAddress(name, category) => {
                let address = name.parse::<Address>().unwrap();
                model.user.contacts.remove(&address, category);
            }
            UserProfileMessage::ClearAddresses(category) => {
                model.user.contacts.clear(category);
            }
            UserProfileMessage::AddRPC(chain) => {
                model.user.rpcs.add(chain);
            }
            UserProfileMessage::RemoveRPC(name) => {
                tracing::debug!("Removing RPC from storage: {}", name);
                model.user.rpcs.remove(&name);
            }

            UserProfileMessage::ClearRPCs => {
                model.user.rpcs.clear();
            }
        }

        let result = model.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        let rpcs = model.user.rpcs.clone();
        Command::perform(async {}, move |_| {
            view::ViewMessage::Settings(settings::Message::Rpc(settings::rpc::Message::Sync(rpcs)))
        })
        .map(|x| x.into())
    }

    #[allow(unreachable_patterns)]
    fn switch_window(&mut self, navigate_to: &view::sidebar::Route) -> Command<AppMessage> {
        let mut cmds = Vec::new();

        let exit_cmd = self.windows.screen.exit();
        cmds.push(exit_cmd);

        self.windows.screen = match navigate_to {
            view::sidebar::Route::Page(page) => {
                cmds.push(
                    self.windows
                        .sidebar
                        .update(view::sidebar::Route::Page(*page))
                        .map(|x| x.into()),
                );

                match page {
                    view::sidebar::Page::Empty => EmptyScreen::new().into(),
                    view::sidebar::Page::Portfolio => {
                        PortfolioRoot::new(Some(self.client.clone()), self.model.clone()).into()
                    }
                    view::sidebar::Page::Settings => {
                        SettingsScreen::new(self.model.user.clone()).into()
                    }
                    view::sidebar::Page::Exit => ExitScreen::new(true).into(),
                }
            }
            _ => EmptyScreen::new().into(),
        };

        let load_cmd = self.windows.screen.load();
        cmds.push(load_cmd);

        Command::batch(cmds)
    }
}

#[derive(Debug)]
pub struct AppClock {
    pub last_update: std::time::Instant,
    pub total_updates: usize,
    pub total_time: Duration,
    pub updates: Vec<Update>,
}

#[derive(Debug)]
pub struct Update {
    pub time: std::time::Instant,
    pub duration: Duration,
}

impl Default for AppClock {
    fn default() -> Self {
        Self::new()
    }
}

impl AppClock {
    pub fn new() -> Self {
        Self {
            last_update: std::time::Instant::now(),
            total_updates: 0,
            total_time: Duration::from_secs(0),
            updates: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_update);
        self.last_update = now;
        self.total_updates += 1;
        self.total_time += elapsed;

        self.updates.push(Update {
            time: now,
            duration: elapsed,
        });
    }

    pub fn average_cycle(&self, window_duration: Duration) -> Duration {
        let now = std::time::Instant::now();
        let window_start = now - window_duration;

        let updates_in_window = self
            .updates
            .iter()
            .filter(|update| update.time >= window_start)
            .collect::<Vec<_>>();

        if updates_in_window.is_empty() {
            return Duration::from_secs(0);
        }

        let total_time_in_window: Duration =
            updates_in_window.iter().map(|update| update.duration).sum();

        total_time_in_window / updates_in_window.len() as u32
    }

    pub fn max_update_time(&self) -> Duration {
        self.updates
            .iter()
            .map(|update| update.duration)
            .max()
            .unwrap_or(Duration::from_secs(0))
    }

    pub fn min_update_time(&self) -> Duration {
        self.updates
            .iter()
            .map(|update| update.duration)
            .min()
            .unwrap_or(Duration::from_secs(0))
    }

    pub fn update_frequency(&self, window_duration: Duration) -> f64 {
        let updates_in_window = self
            .updates
            .iter()
            .filter(|update| update.time >= std::time::Instant::now() - window_duration)
            .count();
        updates_in_window as f64 / window_duration.as_secs_f64()
    }

    pub fn time_between_updates(&self) -> Option<Duration> {
        self.updates
            .iter()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .windows(2)
            .next()
            .map(|window| window[0].time - window[1].time)
    }

    pub fn view_tbu<Message>(&self) -> Element<'_, Message> {
        let tbu_value = self
            .time_between_updates()
            .unwrap_or(Duration::from_secs(0))
            .as_millis();
        let tbu = format!("dur:  {}ms", tbu_value);
        label(tbu)
            .tertiary()
            .caption2()
            .custom_format(move |_| {
                if tbu_value > 5_000 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        1.0, 0.0, 0.0, 0.05,
                    )))
                } else {
                    None
                }
            })
            .into()
    }

    pub fn view_max<Message>(&self) -> Element<'_, Message> {
        let max_value = self.max_update_time().as_millis();
        let max = format!("max:  {}ms", max_value);
        label(max)
            .tertiary()
            .caption2()
            .custom_format(move |_| {
                if max_value > 10_000 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        1.0, 0.0, 0.0, 0.05,
                    )))
                } else if max_value > 5_000 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        0.0, 0.8, 0.2, 0.05,
                    )))
                } else if max_value < 5_000 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        0.0, 1.0, 0.0, 0.05,
                    )))
                } else {
                    None
                }
            })
            .into()
    }

    pub fn view_min<Message>(&self) -> Element<'_, Message> {
        let min = self.min_update_time().as_millis();
        let min = format!("min:  {}ms", min);
        label(min).tertiary().caption2().into()
    }

    pub fn view_frequency<Message>(&self) -> Element<'_, Message> {
        let frequency_value = self.update_frequency(Duration::from_secs(30));
        let frequency = format!("freq:  {:.2}", frequency_value);
        label(frequency)
            .tertiary()
            .caption2()
            .custom_format(move |_| {
                if frequency_value > 10.0 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        1.0, 0.0, 0.0, 0.05,
                    )))
                } else if frequency_value < 2.0 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        0.0, 1.0, 0.0, 0.05,
                    )))
                } else {
                    None
                }
            })
            .into()
    }

    pub fn view_average<Message>(&self) -> Element<'_, Message> {
        let average_value = self.average_cycle(Duration::from_secs(30)).as_millis();
        let average = format!("avg.:  {}ms", average_value);
        label(average)
            .tertiary()
            .caption2()
            .custom_format(move |_| {
                if average_value > 2_500 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        1.0, 0.0, 0.0, 0.05,
                    )))
                } else if average_value > 1_000 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        0.0, 0.8, 0.2, 0.05,
                    )))
                } else if average_value < 1_000 {
                    Some(ExcaliburColor::Custom(iced::Color::from_rgba(
                        0.0, 1.0, 0.0, 0.05,
                    )))
                } else {
                    None
                }
            })
            .into()
    }

    #[allow(dead_code)]
    pub fn view<Message>(&self) -> Element<'_, Message> {
        let average = self.average_cycle(Duration::from_secs(30));
        let average = format!("update time/s:  {}ms", average.as_millis());
        label(average).tertiary().caption2().into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnvilSave {
    pub snapshot: String,
    pub block_number: u64,
}

#[tracing::instrument(skip(client))]
async fn save_snapshot(
    client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<AnvilSave> {
    tracing::debug!("Attempting to save anvil snapshot");
    client.snapshot().await
}
