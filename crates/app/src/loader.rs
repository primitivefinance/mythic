//! The application starts in the Loader state which executes async loading
//! tasks. Once these complete they emit a message back to the iced Application
//! impl in ./lib.rs, which then switches to the runtime App state.

use std::time::Instant;

use alloy_primitives;
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
    components::{
        logos::PhiLogo,
        progress::CustomProgressBar,
        system::{label, ExcaliburContainer},
    },
    model::user::Saveable,
};

type LoadResult =
    anyhow::Result<(Model, Arc<middleware::ExcaliburMiddleware<Ws, LocalWallet>>), anyhow::Error>;

#[derive(Debug)]
pub enum Message {
    View,
    Tick,
    Loaded(super::Flags),
    Connected,
    LoadingFailed,
    Ready(LoadResult),
}
pub struct Loader {
    pub screen_open: bool,
    pub progress: f32,
    pub feedback: String,
    pub max_load_ticks: f32,
    pub load_ticks: f32,
    pub logo: PhiLogo,
}

#[tracing::instrument(level = "debug")]
pub fn load_profile() -> anyhow::Result<UserProfile> {
    let profile = UserProfile::load(None);
    let profile = match profile {
        Ok(profile) => profile,
        Err(e) => {
            tracing::warn!("Failed to load profile: {:?}", e);
            tracing::info!("Creating a new default profile.");

            UserProfile::create_new(None)?
        }
    };

    tracing::debug!(
        "Loaded profile {:?} at path {:?}",
        profile.name,
        profile.file_path()
    );

    Ok(profile)
}

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

/// Loads any async data or disk data into the application's state types.
/// On load, the application will emit the Ready message to the root
/// application, which will then open the App.
#[tracing::instrument(level = "debug")]
pub async fn load_app(flags: super::Flags) -> LoadResult {
    // Load an existing profile, or create a new one.
    let mut profile = load_profile()?;
    let mut model = Model::new(profile.clone());

    let mut exc_client = ExcaliburMiddleware::setup(flags.dev_mode).await?;
    let chain_id = if let Some(anvil) = &exc_client.anvil {
        anvil.chain_id()
    } else {
        1
    };

    // Load the dev client from dev mode flag.
    if flags.dev_mode {
        let signer = exc_client.signer().unwrap().clone().with_chain_id(chain_id);
        let sender = signer.address();
        let client = exc_client.client().unwrap().clone();
        let client = client.with_signer(signer);
        let dev_client = DevClient::deploy(client.into(), sender).await?;

        exc_client.add_contract("protocol", dev_client.protocol.protocol.address());
        exc_client.add_contract(
            "strategy",
            dev_client.protocol.get_strategy().await?.address(),
        );
        exc_client.add_contract("token_x", dev_client.token_x.address());
        exc_client.add_contract("token_y", dev_client.token_y.address());
        exc_client.add_contract("lex", dev_client.liquid_exchange.address());
    }

    // If profile has an anvil snapshot, load it.
    if let Some(snapshot) = &profile.anvil_snapshot {
        let result: String = exc_client
            .client()
            .unwrap()
            .provider()
            .request("anvil_loadState", ())
            .await
            .expect("Failed to load snapshot.");

        tracing::info!("Loaded snapshot: {:?}", result);
    }

    // If dev_client is some, add the tokens to the coins.
    if let Some(anvil) = exc_client.anvil.as_ref() {
        let token_x = exc_client.contracts.get("token_x").unwrap();
        let token_y = exc_client.contracts.get("token_y").unwrap();
        let token_x = alloy_primitives::Address::from(token_x.as_fixed_bytes());
        let token_y = alloy_primitives::Address::from(token_y.as_fixed_bytes());
        let tokens = profile.coins.tokens.clone();
        let coin_x = tokens.iter().find(|c| c.address == token_x);
        let coin_y = tokens.iter().find(|c| c.address == token_y);

        if coin_x.is_none() {
            let coin: Coin = Coin {
                name: "Token X".to_string(),
                symbol: "TKNX".to_string(),
                address: token_x,
                decimals: 18,
                chain_id: anvil.chain_id(),
                logo_uri: "".to_string(),
                tags: vec!["mock".to_string()],
            };
            profile.coins += coin;
            profile.save()?;
        }

        if coin_y.is_none() {
            let coin: Coin = Coin {
                name: "Token Y".to_string(),
                symbol: "TKNY".to_string(),
                address: token_y,
                decimals: 18,
                chain_id: anvil.chain_id(),
                logo_uri: "".to_string(),
                tags: vec!["mock".to_string()],
            };
            profile.coins += coin;
            profile.save()?;
        }
    }

    let mut user = profile;

    // Add the default signer to the contacts book.
    user.contacts.add(
        exc_client.address().unwrap(),
        contacts::ContactValue {
            label: "You".to_string(),
            class: contacts::Class::EOA,
            ..Default::default()
        },
        contacts::Category::Trusted,
    );

    // If dev_client is some, add the protocol's contracts to the user.
    if flags.dev_mode {
        let protocol = exc_client.contracts.get("protocol").cloned().unwrap();
        let strategy = exc_client.contracts.get("strategy").cloned().unwrap();
        let token_x = exc_client.contracts.get("token_x").cloned().unwrap();
        let token_y = exc_client.contracts.get("token_y").cloned().unwrap();
        let lex = exc_client.contracts.get("lex").cloned().unwrap();

        user.contacts.add(
            protocol,
            contacts::ContactValue {
                label: "Protocol".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        user.contacts.add(
            strategy,
            contacts::ContactValue {
                label: "Strategy".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Trusted,
        );

        user.contacts.add(
            token_x,
            contacts::ContactValue {
                label: "Token X".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        user.contacts.add(
            token_y,
            contacts::ContactValue {
                label: "Token Y".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        user.contacts.add(
            lex,
            contacts::ContactValue {
                label: "Liquid Exchange".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        model.portfolio.setup(
            from_ethers_address(exc_client.address().unwrap()),
            from_ethers_address(lex),
            from_ethers_address(protocol),
            from_ethers_address(strategy),
            from_ethers_address(token_x),
            from_ethers_address(token_y),
        );
    }

    Ok((model, Arc::new(exc_client)))
}

#[tracing::instrument(level = "debug")]
pub async fn connect_ledger() -> Option<LedgerClient> {
    let ledger =
        LedgerClient::new_connection(clients::ledger::types::DerivationType::LedgerLive(0)).await;

    let ledger = match ledger {
        Ok(ledger) => Some(ledger),
        Err(e) => {
            tracing::warn!("Failed to connect to ledger: {:?}", e);
            tracing::info!("Creating a new default ledger.");

            None
        }
    };

    ledger
}

/// Placeholder function for any future async calls we might want to do.
pub async fn connect_to_server() -> anyhow::Result<()> {
    Ok(())
}

impl Loader {
    pub fn new(flags: super::Flags) -> (Self, Command<Message>) {
        // Triggers the next step in the main application loop by emitting the Loaded
        // message.
        let flags = flags.clone();

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

                    Message::Loaded(flags)
                }),
            ]),
        )
    }

    fn load(&mut self, flags: super::Flags) -> Command<Message> {
        Command::perform(load_app(flags), Message::Ready)
    }

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

    pub fn get_progress_feedback(&self) -> String {
        let s_curve_result = s_curve(self.progress);
        let progress = (s_curve_result * 4.0) as usize;

        let random_index: usize = progress % CURRENCY_SYMBOLS.len();
        let random_currency = CURRENCY_SYMBOLS[random_index].to_string();

        match progress {
            0 => format!("{} Initiated loading procedure...", random_currency),
            1 => format!("{} Starting sandbox environment...", random_currency),
            2 => format!(
                "{} Connected. Deploying contracts in sandbox...",
                random_currency
            ),
            3 => format!("{} Initializing sandbox state...", random_currency),
            _ => format!("{} Launching Excalibur...", random_currency),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let random_index: usize = self.progress as usize % GREEK_SYMBOLS.len();
        let random_greek = GREEK_SYMBOLS[random_index].to_string();

        container(
            container(
                column![
                    progress_bar(0.0..=1.0, s_curve((self.progress).into()))
                        .style(CustomProgressBar::theme())
                        .height(Length::Fixed(Sizes::Md.into())),
                    Row::new()
                        .push(
                            Column::new()
                                .push(
                                    Container::new(label(&random_greek).highlight().build())
                                        .width(Length::Fixed(48.0))
                                        .height(Length::Fixed(48.0))
                                )
                                .align_items(alignment::Alignment::Start)
                                .width(Length::FillPortion(1)),
                        )
                        .push(
                            Column::new()
                                .push(label(&self.feedback).highlight().build())
                                .align_items(alignment::Alignment::End)
                                .width(Length::FillPortion(3))
                        )
                        .align_items(alignment::Alignment::Center)
                        .spacing(Sizes::Sm)
                        .width(Length::Fill)
                ]
                .padding(Sizes::Sm)
                .align_items(alignment::Alignment::End)
                .spacing(Sizes::Sm as u16),
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

    pub fn subscription(&self) -> Subscription<Message> {
        // Every 25ms update the progress bar by 0.001.
        iced::time::every(std::time::Duration::from_millis(25)).map(|_| Message::Tick)
    }
}

pub const GREEK_SYMBOLS: [char; 10] = ['Γ', 'Δ', 'Θ', 'Λ', 'Ξ', 'Π', 'Σ', 'Φ', 'Ψ', 'Ω'];
pub const CURRENCY_SYMBOLS: [char; 11] = ['$', '€', '£', '¥', '₩', '₿', '₽', '₹', '₺', '₴', 'Ξ'];

pub fn s_curve(x: f32) -> f32 {
    let sigmoid_x = 1.0 / (1.0 + (-x).exp());
    (sigmoid_x - 0.5) * 2.0
}
