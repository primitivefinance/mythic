//! The application starts in the Loader state which executes async loading
//! tasks. Once these complete they emit a message back to the iced Application
//! impl in ./lib.rs, which then switches to the runtime App state.

use std::time::Instant;

use clients::{dev::DevClient, ledger::LedgerClient};
use datatypes::portfolio::coin::Coin;
use iced::{
    font,
    widget::{canvas::Cache, column, container, progress_bar},
    Length,
};
use iced_aw::graphics::icons::ICON_FONT_BYTES;
use sim::from_ethers_address;

use super::{
    middleware::*,
    model::{contacts, user::UserProfile},
    *,
};
use crate::{
    app::AnvilSave,
    components::{
        logos::PhiLogo,
        progress::CustomProgressBar,
        system::{label, ExcaliburContainer},
    },
    model::user::Saveable,
};

type LoadResult = (Model, Arc<middleware::ExcaliburMiddleware<Ws, LocalWallet>>);

#[derive(Debug)]
pub enum Message {
    View,
    Tick,
    Loaded(super::Flags),
    Connected,
    LoadingFailed,
    Ready(LoadResult),
    BrandFontLoaded,
    IconFontLoaded,
    Quit,
}
pub struct Loader {
    pub screen_open: bool,
    pub progress: f32,
    pub feedback: String,
    pub max_load_ticks: f32,
    pub load_ticks: f32,
    pub logo: PhiLogo,
}

/// This function attempts to load a user profile. If it fails, it creates a new
/// default profile. It then logs the loaded profile's name and file path.
#[tracing::instrument(level = "debug")]
pub fn load_profile() -> anyhow::Result<UserProfile> {
    let profile = UserProfile::load(None);
    let profile = match profile {
        Ok(profile) => profile,
        Err(e) => {
            tracing::warn!("Failed to load profile: {:?}", e);
            tracing::info!("Creating a new default profile.");

            UserProfile::create_new(None)
        }
    };

    tracing::debug!(
        "Loaded profile {:?} at path {:?}",
        profile.name,
        profile.file_path()
    );

    Ok(profile)
}

/// This function attempts to load user data into a model. If it fails, it
/// creates a new default model. It then logs the loaded model's user name and
/// file path.
#[tracing::instrument(level = "debug")]
pub fn load_user_data() -> anyhow::Result<Model> {
    // first log we see on start up comes from here
    let model = Model::load(None);
    let model = match model {
        Ok(model) => model,
        Err(e) => {
            tracing::warn!("Failed to load model: {:?}", e);
            tracing::info!("Creating a new default model.");

            Model::create_new(None)
        }
    };

    tracing::debug!(
        "Loaded model {:?} at path {:?}",
        model.user.name,
        model.file_path()
    );

    Ok(model)
}

/// This function loads a development client. It first logs the loading process,
/// then creates a signer with the chain id of the client. It then gets the
/// address of the signer and clones the client. It deploys the development
/// client and returns it.
#[tracing::instrument(skip(client), level = "trace")]
pub async fn load_dev_client(
    client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<DevClient<NetworkClient<Ws, LocalWallet>>> {
    tracing::debug!("Loading dev client");
    let signer = client
        .signer()
        .unwrap()
        .clone()
        .with_chain_id(client.clone().anvil.as_ref().unwrap().chain_id());
    let sender = signer.address();
    let client = client.client().unwrap().clone();
    let dev_client = DevClient::deploy(client, sender).await?;
    Ok(dev_client)
}

/// Contracts that we start up the client with
pub const CONTRACT_NAMES: [&str; 5] = ["protocol", "strategy", "token_x", "token_y", "lex"];

/// Loads any async data or disk data into the application's state types.
/// On load, the application will emit the Ready message to the root
/// application, which will then open the App.
#[tracing::instrument(level = "debug")]
pub async fn load_app(
    flags: super::Flags,
) -> (Model, Arc<middleware::ExcaliburMiddleware<Ws, LocalWallet>>) {
    // Load the user's save or create a new one.

    let mut model = load_user_data().unwrap();

    let mut exc_client = ExcaliburMiddleware::setup(true).await.unwrap();
    let chain_id = if let Some(anvil) = &exc_client.anvil {
        anvil.chain_id()
    } else {
        31337
    };

    // todo: try the connection to the sandbox next
    // Connect the model to the desired network.
    model
        .connect_to_network(exc_client.client().cloned().unwrap())
        .await?;

    // If profile has an anvil snapshot, load it.
    let loaded_snapshot = if let Some(AnvilSave {
        snapshot,
        block_number,
    }) = &model.user.anvil_snapshot
    {
        tracing::debug!(
            "Attempting to load snapshot: {}",
            &snapshot[..10.min(snapshot.len())],
        );

        let client = exc_client.client().unwrap().clone();

        let success = client
            .provider()
            .request::<[String; 1], bool>("anvil_loadState", [snapshot.to_string()])
            .await
            .expect("Failed to load snapshot.");

        tracing::info!("Loaded snapshot: {:?}", success);

        if success {
            tracing::info!("Syncing Anvil to block: {}", block_number);
            client
                .provider()
                .request::<[u64; 1], ()>("anvil_mine", [*block_number])
                .await
                .expect("Failed to sync to block.");
        }

        success
    } else {
        false
    };

    // To synchronize a loaded snapshot with the necessary state:
    // 1. Get the "saved" addresses from the user's contact book.
    // 2. Sync the addresses to the exc_client.
    // 3. Sync the addresses to the model.
    if loaded_snapshot {
        for name in CONTRACT_NAMES.iter() {
            if let Some(contracts) = model
                .user
                .contacts
                .get_class_list(contacts::Class::Contract)
            {
                // If the contract value's label contains the name, add it to the exc_client.
                // todo: need better loading of contracts from storage.
                for (address, contact) in contracts.get_all() {
                    if contact.label.contains(name) {
                        exc_client.add_contract(name, *address);
                    }
                }
            }
        }
    }

    // If we are loading a fresh instance, deploy the contracts.
    if !loaded_snapshot {
        let signer = exc_client.signer().unwrap().clone().with_chain_id(chain_id);
        let sender = signer.address();
        let client = exc_client.client().unwrap().clone();
        let client = client.with_signer(signer);
        tracing::debug!("Deploying contracts with sender: {:?}", sender);
        // error comes from this line
        let dev_client = DevClient::deploy(client.into(), sender).await.unwrap();

        let protocol = dev_client.protocol.dfmm.address();
        let strategy = dev_client.protocol.get_strategy().await.unwrap().address();
        let token_x = dev_client.token_x.address();
        let token_y = dev_client.token_y.address();
        let solver = dev_client.solver.address();
        let lex = dev_client.liquid_exchange.address();

        exc_client.add_contract("protocol", protocol);
        exc_client.add_contract("strategy", strategy);
        exc_client.add_contract("solver", solver);
        exc_client.add_contract("token_x", token_x);
        exc_client.add_contract("token_y", token_y);
        exc_client.add_contract("lex", lex);

        model.user.contacts.add(
            protocol,
            contacts::ContactValue {
                label: "protocol".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        model.user.contacts.add(
            strategy,
            contacts::ContactValue {
                label: "strategy".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Trusted,
        );

        model.user.contacts.add(
            token_x,
            contacts::ContactValue {
                label: "token_x".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        model.user.contacts.add(
            token_y,
            contacts::ContactValue {
                label: "token_y".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        model.user.contacts.add(
            lex,
            contacts::ContactValue {
                label: "lex".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        model.user.contacts.add(
            lex,
            contacts::ContactValue {
                label: "solver".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        tracing::info!("Loaded contacts: {:?}", model.user.contacts);
        // TODO(matt): Create a shared type so that the order of the arguments isn't
        // finicky
        if let Some(connected_model) = model.get_current_mut() {
            connected_model.setup(
                from_ethers_address(exc_client.address().unwrap()),
                from_ethers_address(lex),
                from_ethers_address(protocol),
                from_ethers_address(strategy),
                from_ethers_address(solver),
                from_ethers_address(token_x),
                from_ethers_address(token_y),
            );
        }

        let token_x = alloy_primitives::Address::from(token_x.as_fixed_bytes());
        let token_y = alloy_primitives::Address::from(token_y.as_fixed_bytes());
        let tokens = model.user.coins.tokens.clone();
        let coin_x = tokens.iter().find(|c| c.address == token_x);
        let coin_y = tokens.iter().find(|c| c.address == token_y);

        if coin_x.is_none() {
            let coin: Coin = Coin {
                name: "Token X".to_string(),
                symbol: "TKNX".to_string(),
                address: token_x,
                decimals: 18,
                chain_id,
                logo_uri: "".to_string(),
                tags: vec!["mock".to_string()],
            };
            model.user.coins += coin;
        }

        if coin_y.is_none() {
            let coin: Coin = Coin {
                name: "Token Y".to_string(),
                symbol: "TKNY".to_string(),
                address: token_y,
                decimals: 18,
                chain_id,
                logo_uri: "".to_string(),
                tags: vec!["mock".to_string()],
            };
            model.user.coins += coin;
        }

        model.save().unwrap();
    }

    // Add the default signer to the contacts book, if there is a signer.
    if let Some(address) = exc_client.address() {
        model.user.contacts.add(
            address,
            contacts::ContactValue {
                label: "You".to_string(),
                class: contacts::Class::EOA,
                ..Default::default()
            },
            contacts::Category::Trusted,
        );
        model.save().unwrap();
    }

    (model, Arc::new(exc_client))
}

/// Attempts to establish a new connection with the Ledger hardware wallet.
/// If the connection is successful, it returns an instance of the LedgerClient.
/// If the connection fails, it logs a warning, creates a new default ledger,
/// and returns None.
#[tracing::instrument(level = "debug")]
pub async fn connect_ledger() -> Option<LedgerClient> {
    let ledger =
        LedgerClient::new_connection(clients::ledger::types::DerivationType::LedgerLive(0)).await;

    match ledger {
        Ok(ledger) => Some(ledger),
        Err(e) => {
            tracing::warn!("Failed to connect to ledger: {:?}", e);
            tracing::info!("Creating a new default ledger.");

            None
        }
    }
}

/// Placeholder function for any future async calls we might want to do.
pub async fn connect_to_server() -> anyhow::Result<()> {
    Ok(())
}

pub const DAGGER_SQUARE_FONT_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/DAGGERSQUARE.otf");

impl Loader {
    /// Creates a new Loader with the given flags and returns a tuple of the
    /// Loader and a Command. The Command triggers the next step in the main
    /// application loop by emitting the Loaded message. The Loader is
    /// initialized with a progress of 0.0, a feedback message of "Loading
    /// profile", and a logo. The max_load_ticks is calculated as the
    /// product of max_load_seconds and ticks_per_s. The function also
    /// attempts to connect to the server and load the icon and brand fonts.
    /// If any of these operations fail, a LoadingFailed message is returned.
    /// If all operations are successful, a tuple of the Loader and a Command is
    /// returned.
    pub fn new(flags: super::Flags) -> (Self, Command<Message>) {
        let max_load_seconds = 5.0;
        let ticks_per_s = 40.0;

        (
            Self {
                screen_open: false,
                progress: 0.0,
                feedback: "Loading profile".to_string(),
                max_load_ticks: max_load_seconds * ticks_per_s,
                load_ticks: 0.0,
                logo: PhiLogo {
                    start: Instant::now(),
                    rotation: 0.0,
                    cache: Cache::default(),
                },
            },
            Command::batch(vec![
                Command::perform(connect_to_server(), |res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to connect to server: {:?}", e);
                        return Message::LoadingFailed;
                    }

                    Message::Connected
                }),
                font::load(ICON_FONT_BYTES).map(move |res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to load icon font: {:?}", e);
                        return Message::LoadingFailed;
                    }

                    Message::IconFontLoaded
                }),
                font::load(DAGGER_SQUARE_FONT_BYTES).map(move |res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to load icon font: {:?}", e);
                        return Message::LoadingFailed;
                    }

                    Message::BrandFontLoaded
                }),
                Command::perform(async {}, move |_| Message::Loaded(flags)),
            ]),
        )
    }

    /// Takes in the application flags and returns a command to load the
    /// application. The loading process is performed asynchronously.
    fn load(&mut self, flags: super::Flags) -> Command<Message> {
        Command::perform(load_app(flags), Message::Ready)
    }

    /// Updates the state of the loader based on the received message.
    /// This function handles different types of messages and updates the
    /// loader's state accordingly. For example, it updates the progress of
    /// the loading process, handles connection status, and initiates the
    /// loading process.
    pub fn update(&mut self, message: Message) -> Command<Message> {
        self.logo.cache.clear();

        match message {
            Message::Tick => {
                self.feedback = self.get_progress_feedback();

                if !self.screen_open {
                    return Command::none();
                }

                self.load_ticks += 1.0;

                self.progress = self.load_ticks / self.max_load_ticks;

                Command::none()
            }
            Message::Connected => {
                self.screen_open = true;
                self.load_ticks = 0.0;
                Command::none()
            }
            Message::Loaded(flags) => self.load(flags),
            _ => Command::none(),
        }
    }

    /// Returns a string that represents the current progress of the loading
    /// process. The progress is represented by different stages of the
    /// loading process. The stages are: "Initiated loading procedure...",
    /// "Starting sandbox environment...", "Connected. Deploying contracts
    /// in sandbox...", "Initializing sandbox state...", and "Launching
    /// Excalibur...".
    pub fn get_progress_feedback(&self) -> String {
        let s_curve_result = s_curve(self.progress);
        let progress = (s_curve_result * 4.0) as usize;

        match progress {
            0 => "Initiated loading procedure...".to_string(),
            1 => "Starting sandbox environment...".to_string(),
            2 => "Connected. Deploying contracts in sandbox...".to_string(),
            3 => "Initializing sandbox state...".to_string(),
            _ => "Launching Excalibur...".to_string(),
        }
    }

    /// This function generates a view of the loading screen.
    /// It displays a progress bar that updates based on the loading progress.
    /// It also displays a random symbol from a collection of Greek and currency
    /// symbols. The symbol changes with each update of the progress bar.
    /// The function also displays a feedback message that corresponds to the
    /// current stage of the loading process.
    pub fn view(&self) -> Element<Message> {
        let all_symbols = GREEK_SYMBOLS
            .iter()
            .chain(CURRENCY_SYMBOLS.iter())
            .collect::<Vec<_>>();

        let random_index = self.progress as usize % all_symbols.len();
        let random_symbol = all_symbols[random_index].to_string();

        container(
            container(
                column![
                    progress_bar(0.0..=1.0, s_curve(self.progress))
                        .style(CustomProgressBar::theme())
                        .height(Length::Fixed(Sizes::Md.into())),
                    Row::new()
                        .push(
                            Column::new()
                                .push(label(random_symbol).secondary().build())
                                .align_items(alignment::Alignment::Start)
                                .width(Length::FillPortion(1)),
                        )
                        .push(
                            Column::new()
                                .push(label(&self.feedback).secondary().build())
                                .align_items(alignment::Alignment::End)
                                .width(Length::FillPortion(3))
                        )
                        .align_items(alignment::Alignment::Center)
                        .spacing(Sizes::Sm)
                        .width(Length::Fill)
                ]
                .padding(Sizes::Sm)
                .align_items(alignment::Alignment::End)
                .spacing(Sizes::Sm),
            )
            .max_width(ByteScale::Xl6 as u32 as f32),
        )
        .style(
            ExcaliburContainer::default()
                .background_iced(iced::Color::BLACK)
                .theme(),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    // Every 25ms update the progress bar by 0.001.
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(25)).map(|_| Message::Tick)
    }
}

pub const GREEK_SYMBOLS: [char; 10] = ['Γ', 'Δ', 'Θ', 'Λ', 'Ξ', 'Π', 'Σ', 'Φ', 'Ψ', 'Ω'];
pub const CURRENCY_SYMBOLS: [char; 11] = ['$', '€', '£', '¥', '₩', '₿', '₽', '₹', '₺', '₴', 'Ξ'];

pub fn s_curve(x: f32) -> f32 {
    let sigmoid_x = 1.0 / (1.0 + (-x).exp());
    (sigmoid_x - 0.5) * 2.0
}
