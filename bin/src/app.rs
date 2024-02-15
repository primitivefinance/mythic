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

/// Root message for the Application.
#[derive(Debug, Default)]
pub enum AppMessage {
    /// An empty message used as a default.
    #[default]
    Empty,
    /// Emitted on the initial load of the App.
    Load,
    /// Exits the application immediately.
    QuitReady,
    /// All children controllers wrap their messages in View.
    View(view::ViewMessage),
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

/// State for specific windows that are open.
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

/// The main application structure.
///
/// This structure follows the Elm architecture, which is a pattern for
/// architecting interactive programs. The Elm architecture has a few core
/// parts: a model, an update function, and a view function.
///
/// The model is the single source of truth for our application. It is defined
/// in the `model` field of this structure.
///
/// The [`App.update()`] function is where we make changes to our model. It
/// takes the current model and a message, and produces a new model.
/// This is done in the `update` method of this structure.
///
/// The [`App.view()`] function takes the current model and produces a
/// description of what we want to see on screen. This is done in the `view`
/// method of this structure.
///
/// The flow of information in the application goes as follows:
/// 1. The user interacts with the view (e.g., clicking a button).
/// 2. The view produces a message based on the user's interaction.
/// 3. The message is sent to the update function.
/// 4. The update function takes the current model and the message, and produces
///    a new model.
/// 5. The new model is sent to the view function, and the cycle repeats.
impl App {
    /// Creates a new instance of the App.
    ///
    /// This function initializes the application with the provided model and
    /// client. It sets up the dashboard with the portfolio root and the
    /// sidebar with the portfolio page. It also initializes the application
    /// clock.
    ///
    /// The function returns a tuple containing the newly created App and a
    /// Command to load the application.
    ///
    /// # Arguments
    ///
    /// * `model` - The data model of the application.
    /// * `client` - The client used for network connections.
    ///
    /// # Returns
    ///
    /// * `(Self, Command<Message>)` - A tuple containing the newly created App
    ///   and a Command to load the application.
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

    /// This function is responsible for loading the sidebar and the default
    /// screen. It is called when a user starts the application, after the
    /// new() function.
    pub fn load(&mut self) -> Command<AppMessage> {
        // Load the sidebar and the current window.
        let cmds = vec![
            self.windows.sidebar.load().map(|x| x.into()),
            self.windows.screen.load().map(|x| x),
        ];
        Command::batch(cmds)
    }

    /// Handles all updates to the application's state.
    ///
    /// This function is responsible for updating the application's state based
    /// on the provided message. It first updates the application's clock,
    /// then handles the message in a variety of ways depending on its type.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to handle. This can be a variety of types,
    ///   each of which results in a different update to the application's
    ///   state.
    ///
    /// # Returns
    ///
    /// * `Command<Message>` - A command that will be executed after the update.
    ///   This can be a variety of types, each of which results in a different
    ///   action being taken.
    ///
    /// # Panics
    ///
    /// This function will panic if the `Message::ModelSyncResult` variant is
    /// used and the result is an error.
    pub fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        // Handle the update clock first.
        self.app_clock.update();

        // Handle the update.
        app_span().in_scope(|| match message {
            AppMessage::Load => self.load(),
            AppMessage::QuitReady => {
                // Caught by the lib.rs and exits the application.
                Command::none()
            }
            AppMessage::ModelSyncResult(Ok(model)) => {
                // Update the root model.
                self.model = model.clone();

                // Propagate the model to the active screen.
                // todo: remove side effects, @alex what did you mean by this?
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
                    // todo: remove side effects
                    tracing::debug!("Confirming exit");
                    self.windows
                        .screen
                        .update(AppMessage::View(view::ViewMessage::Root(msg)))
                        .map(|x| x)
                }
            },

            // All the view messages are forwarded to the screen.
            _ => self.windows.screen.update(message),
        })
    }

    /// Returns the view of the application.
    ///
    /// This function takes the current state of the application and produces a
    /// description of what we want to see on screen. It uses the
    /// `app_layout` function from the `view` module to create a layout for the
    /// application. The layout includes the application clock, the sidebar,
    /// and the current screen. The function then maps the layout to a
    /// `Message::View` and returns it.
    ///
    /// # Returns
    /// * `Element<Message>` - The view of the application.
    pub fn view(&self) -> Element<AppMessage> {
        view::app_layout(
            &self.app_clock,
            &self.windows.sidebar,
            self.windows.screen.view(),
        )
        .map(AppMessage::View)
    }

    /// Returns the subscription of the current screen.
    ///
    /// This function retrieves the subscription of the current screen in the
    /// application window. Subscriptions are a way to listen for external
    /// events that are not user interactions, like time passing or messages
    /// arriving from a server.
    ///
    /// # Returns
    /// * `Subscription<Message>` - The subscription of the current screen.
    pub fn subscription(&self) -> Subscription<AppMessage> {
        self.windows.screen.subscription()
    }

    /// Exits the application.
    ///
    /// This function performs several operations before exiting the
    /// application:
    /// 1. It saves the current profile to disk.
    /// 2. It calls the exit function on the currently opened window.
    /// 3. If the development client is active, it saves a snapshot of the anvil
    ///    state to the profile.
    ///
    /// # Returns
    /// * `Command<Message>` - A batch of commands to be executed during the
    ///   exit process.
    pub fn exit(&mut self) -> Command<AppMessage> {
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

    /// Synchronizes the application model with the client.
    ///
    /// This function checks if a client is available. If a client is available,
    /// it clones the current model and the client's provider.
    /// It then asynchronously updates the model using the cloned provider and
    /// returns the updated model in a `Command`. If no client is available,
    /// it logs a debug message and returns an empty `Command`.
    ///
    /// # Returns
    /// * `Command<Message>` - A command containing the result of the model
    ///   synchronization.
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

    /// Updates the user profile based on the provided message.
    ///
    /// This function handles various types of UserProfileMessage to update the
    /// user profile. It can save an anvil snapshot, add or remove
    /// addresses, clear addresses, add or remove RPCs, and clear RPCs.
    /// After updating the user profile, it saves the profile to disk and logs
    /// the result. It then clones the user's RPCs and returns a Command to
    /// sync the RPCs.
    ///
    /// # Arguments
    ///
    /// * `message` - The UserProfileMessage that specifies how to update the
    ///   user profile.
    ///
    /// # Returns
    ///
    /// * `Command<Message>` - A command to sync the RPCs after updating the
    ///   user profile.
    // #[allow(unused_assignments)]
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

                // Exits the application after saving the anvil snapshot.
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

    /// This function is responsible for switching between different windows in
    /// the application. It first creates an exit command for the current
    /// window and adds it to a command vector. Then, it checks the route
    /// provided to navigate to the appropriate window. If the route is a
    /// page, it updates the current page in the sidebar and creates a new
    /// screen based on the page type (Empty, Portfolio, Settings, Exit).
    /// If the route is not a page, it defaults to an empty screen.
    /// After setting the new screen, it creates a load command for the new
    /// window and adds it to the command vector. Finally, it batches all
    /// the commands in the vector and returns them.
    #[allow(unreachable_patterns)]
    fn switch_window(&mut self, navigate_to: &view::sidebar::Route) -> Command<AppMessage> {
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

/// The `Update` struct is used to track the time and duration of updates in the
/// application. It contains two public fields: `time` and `duration`.
/// `time` is a `std::time::Instant` that marks the time of the update.
/// `duration` is a `std::time::Duration` that represents the duration of the
/// update.
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
    /// Constructor for AppClock.
    pub fn new() -> Self {
        Self {
            last_update: std::time::Instant::now(),
            total_updates: 0,
            total_time: Duration::from_secs(0),
            updates: Vec::new(),
        }
    }

    /// Updates the `AppClock` with the current time and duration since the last
    /// update. It increments the total number of updates and adds the
    /// elapsed time to the total time. It also pushes a new `Update` struct
    /// to the updates vector with the current time and elapsed duration.
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

    /// This function calculates the average update cycle time within a given
    /// duration. It filters the updates that happened within the window
    /// duration from the current time, sums up their durations, and divides
    /// by the number of updates to get the average. If there are no updates
    /// within the window duration, it returns a duration of 0 seconds.
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

    /// This function calculates the maximum time taken for an update.
    /// It iterates over all updates, finds the one with the longest duration,
    /// and returns that duration. If there are no updates, it returns a
    /// duration of 0 seconds.
    pub fn max_update_time(&self) -> Duration {
        self.updates
            .iter()
            .map(|update| update.duration)
            .max()
            .unwrap_or(Duration::from_secs(0))
    }

    /// This function calculates the minimum time taken for an update.
    /// It iterates over all updates, finds the one with the smallest duration,
    /// and returns that duration. If there are no updates, it returns a
    /// duration of 0 seconds.
    pub fn min_update_time(&self) -> Duration {
        self.updates
            .iter()
            .map(|update| update.duration)
            .min()
            .unwrap_or(Duration::from_secs(0))
    }

    /// This function calculates the frequency of updates within a given
    /// duration. It returns a floating point number representing the
    /// frequency of updates per second.
    pub fn update_frequency(&self, window_duration: Duration) -> f64 {
        let updates_in_window = self
            .updates
            .iter()
            .filter(|update| update.time >= std::time::Instant::now() - window_duration)
            .count();
        updates_in_window as f64 / window_duration.as_secs_f64()
    }

    /// This function calculates the time between the last two updates.
    /// It returns an Option with the Duration of the time between updates.
    /// If there are less than two updates, it returns None.
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

    /// `view_tbu` is a function that returns an Element displaying the time
    /// between updates (tbu) in milliseconds. The label color changes based
    /// on the tbu:
    /// - Red if the tbu is greater than 5,000 milliseconds.
    /// - No color if the tbu is less than or equal to 5,000 milliseconds.
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

    /// This function returns an Element that displays the maximum update time
    /// in milliseconds. The color of the label changes based on the maximum
    /// update time:
    /// - Red if the maximum update time is greater than 10,000 milliseconds.
    /// - Green if the maximum update time is less than 5,000 milliseconds.
    /// - A mix of green and red if the maximum update time is between 5,000 and
    ///   10,000 milliseconds.
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

    /// This function returns an Element that displays the minimum update time
    /// in milliseconds.
    pub fn view_min<Message>(&self) -> Element<'_, Message> {
        let min = self.min_update_time().as_millis();
        let min = format!("min:  {}ms", min);
        label(min).tertiary().caption2().into()
    }

    /// This function returns an Element that displays the update frequency.
    /// The update frequency is calculated over a duration of 30 seconds.
    /// The color of the label changes based on the update frequency:
    /// - Red if the update frequency is greater than 10.0.
    /// - Green if the update frequency is less than 2.0.
    /// - No color if the update frequency is between 2.0 and 10.0.
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

    /// This function returns an Element that displays the average cycle
    /// duration in milliseconds. The average cycle duration is calculated
    /// over a duration of 30 seconds. The color of the label changes based
    /// on the average cycle duration:
    /// - Red if the average cycle duration is greater than 2,500 milliseconds.
    /// - Green if the average cycle duration is less than 1,000 milliseconds.
    /// - A mix of green and red if the average cycle duration is between 1,000
    ///   and 2,500 milliseconds.
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

    /// This function returns an Element that displays the average update time
    /// in milliseconds. The average update time is calculated over a
    /// duration of 30 seconds.
    #[allow(dead_code)]
    pub fn view<Message>(&self) -> Element<'_, Message> {
        let average = self.average_cycle(Duration::from_secs(30));
        let average = format!("update time/s:  {}ms", average.as_millis());
        label(average).tertiary().caption2().into()
    }
}

// AnvilSave is a struct that represents a snapshot of the Anvil state at a
// specific block number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnvilSave {
    // snapshot is a string representation of the Anvil state.
    pub snapshot: String,
    // block_number is the block number at which the snapshot was taken.
    pub block_number: u64,
}

// save_snapshot is an asynchronous function that attempts to save a snapshot of
// the Anvil state. It takes an Arc of ExcaliburMiddleware with Ws and
// LocalWallet as parameters. It returns a Result of AnvilSave.
#[tracing::instrument(skip(client))]
async fn save_snapshot(
    client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<AnvilSave> {
    // Log a debug message indicating that a snapshot save attempt is being made.
    tracing::debug!("Attempting to save anvil snapshot");
    // Call the snapshot method on the client and await the result.
    client.snapshot().await
}
