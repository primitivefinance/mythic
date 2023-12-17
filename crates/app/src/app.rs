//! Root controller for Excalibur.
//!
//! Handles connecting all children controllers, the data model, and the data
//! view together.
//!
//! ## Syncing the model
//! Children components can request to sync and update the model by emitting a
//! `SyncModel` message that is wrapped in a
//! Message::View(view::Message::Root(..)).
//!
//! This is caught by the root application controller, which can handle the
//! model update.
//!
//! Children components have their own model updated lazily. That is:
//! - The root application controller updates the model.
//! - The model gets propagated to the active children controller (i.e. a
//!   screen).
//! - Changing to a new screen will be instantiated with the updated model, but
//!   the previous screen will not be updated.
//! - The previous screen will be therefore be updated when it is reloaded from
//!   a `switch_window` call.
//!
//! ## Routing
//! The root application controller handles routing to different screens.
//! These route messages start at the View message and get caught in flight,
//! like the SyncModel message. From here, a message is returned to the Root
//! application to route to a new page.
//!
//!
//! ## Finding Bug Culprits:
//! - Empty data: check the controller.
//! - Missing data: check the model.
//! - Wrong data: check the presenter.
//! - Placement in the view: check the view.
//!
//! The Controller handles user input and updates the Model, the Presenter
//! prepares data for the View, and the View handles rendering.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use tracing::Span;

use super::{
    controller::{empty::EmptyScreen, exit::ExitScreen, Screen},
    *,
};
use crate::{
    components::system::{label, ExcaliburColor},
    controller::{
        dev::experimental::ExperimentalScreen, portfolio::PortfolioRoot, settings::SettingsScreen,
        State,
    },
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

/// Root message for the Application.
#[derive(Debug, Default)]
pub enum Message {
    /// An empty message used as a default.
    #[default]
    Empty,
    /// Emitted on the initial load of the App.
    Load,
    /// Exits the application immediately.
    QuitReady,
    /// All children controllers wrap their messages in View.
    View(view::Message),
    /// Modifications to the persistent user profile.
    UpdateUser(UserProfileMessage),
    /// Switches the active "app" to the target Route.
    SwitchWindow(view::sidebar::Route),
    /// Updates the model after it has been fetched.
    ModelSyncResult(Result<Model, Arc<anyhow::Error>>),
}

/// All messages for making modifications to the persistent user profile.
#[derive(Debug)]
pub enum UserProfileMessage {
    /// Stores a stringified snapshot of an Anvil instance.
    SaveAnvilSnapshot(anyhow::Result<AnvilSave>),
    /// Adds an address to the contacts list.
    AddAddress(String, Address, contacts::Category),
    /// Removes an address from the contacts list.
    RemoveAddress(String, contacts::Category),
    /// warning! Deletes all addresses from a category in the list.
    ClearAddresses(contacts::Category),
    /// Adds an RPC to the RPC list.
    AddRPC(RPCValue),
    /// Removes an RPC from the RPC list.
    RemoveRPC(String),
    /// warning! Deletes all RPCs from the list.
    ClearRPCs,
}

pub type RootMessage = Message;
pub type RootViewMessage = view::Message;

impl MessageWrapper for Message {
    type ParentMessage = Message;
}

impl From<UserProfileMessage> for Message {
    fn from(message: UserProfileMessage) -> Self {
        Self::UpdateUser(message)
    }
}

/// State for specific windows that are open.
pub struct Windows {
    pub screen: Screen,
    pub sidebar: Sidebar,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: ExperimentalScreen::new().into(),
            sidebar: Sidebar::new(),
        }
    }
}

impl Windows {
    pub fn new(screen: Screen, sidebar: Sidebar) -> Self {
        Self { screen, sidebar }
    }
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children
/// components will need.
pub struct App {
    /// Connection to networks.
    pub client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    /// Data module of the application.
    pub model: Model,
    /// State of the active window and sidebar the user is viewing.
    pub windows: Windows,

    /// Rough tracking of update intervals.
    pub app_clock: AppClock,
}

impl App {
    pub fn new(
        model: Model,
        client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    ) -> (Self, Command<Message>) {
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
            Command::perform(async {}, |_| Message::Load),
        )
    }

    /// Loads the sidebar and the default screen. Called after new().
    pub fn load(&mut self) -> Command<Message> {
        let mut cmds = Vec::new();

        // Load the sidebar.
        cmds.push(self.windows.sidebar.load().map(|x| x.into()));

        // Load the current window.
        cmds.push(self.windows.screen.load().map(|x| x.into()));

        Command::batch(cmds)
    }

    /// All view updates are forwarded to the Screen's update function.
    pub fn update(&mut self, message: Message) -> Command<Message> {
        // Handle the update clock first.
        self.app_clock.update();

        // Handle the update.
        app_span().in_scope(|| match message {
            Message::Load => self.load(),
            Message::QuitReady => {
                // Caught by the lib.rs and exits the application.
                Command::none()
            }
            Message::ModelSyncResult(Ok(model)) => {
                // Update the root model.
                self.model = model.clone();

                // Propagate the model to the active screen.
                // todo: remove side effects
                return self
                    .windows
                    .screen
                    .update(Message::ModelSyncResult(Ok(model)));
            }
            Message::ModelSyncResult(Err(e)) => {
                tracing::error!(
                    "Critical failure - sync model threw an error while updating: {:?}",
                    e
                );
                Command::none()
            }
            Message::UpdateUser(msg) => self.update_user(msg),
            Message::View(view::Message::Root(msg)) => match msg {
                view::RootMessage::ModelSyncRequest => self.sync_model(),
                view::RootMessage::Route(route) => self.switch_window(&route),
                view::RootMessage::CopyToClipboard(contents) => iced::clipboard::write(contents),
                view::RootMessage::SaveAndExit => self.exit(),
                view::RootMessage::Empty => Command::none(),
                view::RootMessage::ConfirmExit => {
                    // todo: remove side effects
                    tracing::debug!("Confirming exit");
                    self.windows
                        .screen
                        .update(Message::View(view::Message::Root(msg)))
                        .map(|x| x.into())
                }
            },

            // All the view messages are forwarded to the screen.
            _ => self.windows.screen.update(message),
        })
    }

    pub fn view(&self) -> Element<Message> {
        view::app_layout(
            &self.app_clock,
            &self.windows.sidebar,
            self.windows.screen.view(),
        )
        .map(Message::View)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.windows.screen.subscription()
    }

    pub fn exit(&mut self) -> Command<Message> {
        // Save the profile to disk.
        let result = self.model.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        // Call exit on the opened window.
        let mut commands = Vec::new();
        let cmd = self.windows.screen.exit();
        commands.push(cmd);

        // If the dev client is Some, call the anvil client using `anvil_dumpState`, and
        // set the profile's anvil snapshot to the result.
        if let Some(_) = self.client.anvil {
            let cmd = Command::perform(
                save_snapshot(self.client.clone()),
                UserProfileMessage::SaveAnvilSnapshot,
            )
            .map(|x| Message::UpdateUser(x));
            commands.push(cmd);
        }

        Command::batch(commands)
    }

    /// Updates the model and returns its mutated state in a Result.
    fn sync_model(&mut self) -> Command<Message> {
        if let Some(client) = self.client.client().cloned() {
            let model = self.model.clone();
            // todo: fix this clunky provider
            let provider = Arc::new(client.provider().clone());
            Command::perform(
                async move {
                    let mut model = model;
                    model.update(provider).await?;
                    Ok(model)
                },
                Message::ModelSyncResult,
            )
        } else {
            tracing::debug!("No client. Not syncing model.");
            Command::none()
        }
    }

    #[allow(unused_assignments)]
    fn update_user(&mut self, message: UserProfileMessage) -> Command<Message> {
        let model = &mut self.model;

        let mut cmd = Command::none();
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

                // Exits the application after saving the anvil snapshot.
                return Command::perform(async {}, |_| Message::QuitReady);
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
        cmd = Command::perform(async {}, move |_| {
            view::Message::Settings(settings::Message::Rpc(settings::rpc::Message::Sync(rpcs)))
        })
        .map(|x| x.into());
        cmd
    }

    #[allow(unreachable_patterns)]
    fn switch_window(&mut self, navigate_to: &view::sidebar::Route) -> Command<Message> {
        let mut cmds = Vec::new();

        let exit_cmd = self.windows.screen.exit();
        cmds.push(exit_cmd);

        self.windows.screen = match navigate_to {
            view::sidebar::Route::Page(page) => {
                // Updates the current page in the sidebar.
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

/// For measuring performance of the app, we measure the time between
/// updates.
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
        label(&tbu)
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
        label(&max)
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
        label(&min).tertiary().caption2().into()
    }

    pub fn view_frequency<Message>(&self) -> Element<'_, Message> {
        let frequency_value = self.update_frequency(Duration::from_secs(30));
        let frequency = format!("freq:  {:.2}", frequency_value);
        label(&frequency)
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
        label(&average)
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

    pub fn view<Message>(&self) -> Element<'_, Message> {
        let average = self.average_cycle(Duration::from_secs(30));
        let average = format!("update time/s:  {}ms", average.as_millis());
        label(&average).tertiary().caption2().into()
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

#[cfg(test)]
mod tests {

    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    use super::*;

    fn cache_update_bench(c: &mut Criterion) {
        // let storage = Storage::default();
        // let chains = Chains::default();
        // let mut app = App::new(storage, chains, Streams::default(), None).0;
        // c.bench_function("cache_update", |b| {
        // b.iter(|| {
        // app.cache_update(CacheMessage::AppEvent(AppEventLog::new(
        // "test".to_string(),
        // "test".to_string(),
        // )))
        // })
        // });
    }
}
