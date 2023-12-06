use alloy_primitives;
use clients::{
    client::{AnvilClient, Local},
    dev::DevClient,
    ledger::LedgerClient,
};
use datatypes::portfolio::{coin::Coin, coin_list::CoinList};
use ethers::middleware::SignerMiddleware;
use iced::{
    font,
    widget::{column, container, progress_bar},
    Length,
};
use iced_aw::graphics::icons::ICON_FONT_BYTES;
use user::contacts;

use super::{middleware::*, profile::Profile, *};

type LoadResult = anyhow::Result<
    (
        app::Storage,
        app::Chains,
        Option<clients::ledger::LedgerClient>,
        Option<clients::dev::DevClient<DefaultMiddleware>>,
    ),
    anyhow::Error,
>;

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
    pub progress: f32,
    pub feedback: String,
}

#[tracing::instrument(level = "debug")]
pub fn load_profile() -> anyhow::Result<Profile> {
    let profile = Profile::load(None);
    let profile = match profile {
        Ok(profile) => profile,
        Err(e) => {
            tracing::warn!("Failed to load profile: {:?}", e);
            tracing::info!("Creating a new default profile.");

            Profile::create_new(None)?
        }
    };

    tracing::debug!(
        "Loaded profile {:?} at path {:?}",
        profile.name,
        profile.file_path()
    );

    Ok(profile)
}

pub type DefaultMiddleware = SignerMiddleware<Provider<Ws>, LocalWallet>;

#[tracing::instrument(skip(client), level = "trace")]
pub async fn load_dev_client(
    client: Arc<DefaultMiddleware>,
) -> anyhow::Result<DevClient<DefaultMiddleware>> {
    tracing::debug!("Loading dev client");
    let sender = client.address();
    let dev_client = DevClient::deploy(client, sender).await?;
    Ok(dev_client)
}

/// Loads any async data or disk data into the application's state types.
/// On load, the application will emit the Ready message to the root
/// application, which will then open the App.
#[tracing::instrument(level = "debug")]
pub async fn load_app(flags: super::Flags) -> LoadResult {
    // Load an existing profile, or create a new one.
    let profile = load_profile()?;

    // Start anvil the background.
    let anvil = AnvilClient::new()?;
    let anvil_default_chain_id = anvil.anvil.clone().chain_id();

    // Get a default signer + provider from anvil.
    let (sub_clients, sign_clients) = from_anvil(&anvil.anvil.clone()).await?;
    let chains = app::Chains {
        call_clients: vec![],
        sub_clients,
        sign_clients,
        anvil_client: anvil,
    };

    // Create a client from the default provider + signer.
    let default_anvil_client = chains.get_signer(0, 0).unwrap();

    // If profile has an anvil snapshot, load it.
    if let Some(snapshot) = &profile.anvil_snapshot {
        let result: String = default_anvil_client
            .provider()
            .request("anvil_loadState", ())
            .await
            .expect("Failed to load snapshot.");

        tracing::info!("Loaded snapshot: {:?}", result);
    }

    // Load the dev client from dev mode flag.
    let dev_client = match flags.dev_mode {
        true => Some(load_dev_client(default_anvil_client.clone()).await?),
        false => None,
    };

    // Load the coinlist from disk.
    let mut coinlist = CoinList::load(None)?;

    // If dev_client is some, add the tokens to the coinlist.
    if let Some(dev_client) = &dev_client {
        let token_x = dev_client.token_x.address();
        let token_y = dev_client.token_y.address();
        let token_x = alloy_primitives::Address::from(token_x.as_fixed_bytes());
        let token_y = alloy_primitives::Address::from(token_y.as_fixed_bytes());
        let tokens = coinlist.tokens.clone();
        let coin_x = tokens.iter().find(|c| c.address == token_x);
        let coin_y = tokens.iter().find(|c| c.address == token_y);

        if coin_x.is_none() {
            let coin: Coin = Coin {
                name: "Token X".to_string(),
                symbol: "TKNX".to_string(),
                address: token_x,
                decimals: 18,
                chain_id: anvil_default_chain_id,
                logo_uri: "".to_string(),
                tags: vec!["mock".to_string()],
            };
            coinlist += coin;
            coinlist.save()?;
        }

        if coin_y.is_none() {
            let coin: Coin = Coin {
                name: "Token Y".to_string(),
                symbol: "TKNY".to_string(),
                address: token_y,
                decimals: 18,
                chain_id: anvil_default_chain_id,
                logo_uri: "".to_string(),
                tags: vec!["mock".to_string()],
            };
            coinlist += coin;
            coinlist.save()?;
        }
    }

    let mut storage = app::Storage { profile };

    // Add the default signer to the contacts book.
    storage.profile.contacts.add(
        default_anvil_client.address(),
        contacts::ContactValue {
            label: "You".to_string(),
            class: contacts::Class::EOA,
            ..Default::default()
        },
        contacts::Category::Trusted,
    );

    // If dev_client is some, add the protocol's contracts to the storage.
    if let Some(dev_client) = &dev_client {
        let protocol = dev_client.protocol.protocol.address();
        let strategy = dev_client.protocol.get_strategy().await.unwrap().address();
        let token_x = dev_client.token_x.address();
        let token_y = dev_client.token_y.address();

        storage.profile.contacts.add(
            protocol,
            contacts::ContactValue {
                label: "Protocol".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        storage.profile.contacts.add(
            strategy,
            contacts::ContactValue {
                label: "Strategy".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Trusted,
        );

        storage.profile.contacts.add(
            token_x,
            contacts::ContactValue {
                label: "Token X".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );

        storage.profile.contacts.add(
            token_y,
            contacts::ContactValue {
                label: "Token Y".to_string(),
                class: contacts::Class::Contract,
                ..Default::default()
            },
            contacts::Category::Untrusted,
        );
    }

    let ledger = connect_ledger().await;

    Ok((storage, chains, ledger, dev_client))
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
        (
            Self {
                progress: 0.0,
                feedback: "Loading profile".to_string(),
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
        match message {
            Message::Tick => {
                // Update over time until its 80%.
                if self.progress >= 0.8 {
                    return Command::none();
                }

                self.progress += 0.001;
                Command::none()
            }
            Message::Connected => {
                self.progress += 0.2;
                self.feedback = "Starting Anvil...".to_string();
                Command::none()
            }
            Message::Loaded(flags) => self.load(flags),
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            container(
                column![
                    highlight_label(self.feedback.clone()),
                    progress_bar(0.0..=1.0, self.progress)
                ]
                .spacing(Sizes::Lg as u16),
            )
            .max_width(ByteScale::Xl6 as u32 as f32),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Every 250ms update the progress bar by 0.001.
        iced::time::every(std::time::Duration::from_millis(25)).map(|_| Message::Tick)
    }
}
