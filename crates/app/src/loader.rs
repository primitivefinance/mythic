use api::contacts;
use arbiter_core::environment::builder::EnvironmentBuilder;
use clients::rpc::local::Local;
use iced::{
    font,
    widget::{column, container, progress_bar},
    Length,
};
use iced_aw::graphics::icons::ICON_FONT_BYTES;
use profiles::coins::{CoinList, StaticCoin};

use super::{profile::Profile, *};

type LoadResult = anyhow::Result<(app::Storage, app::Chains), anyhow::Error>;

#[derive(Debug)]
pub enum Message {
    View,
    Tick,
    Loaded,
    Connected,
    LoadingFailed,
    Ready(LoadResult),
}
pub struct Loader {
    pub progress: f32,
    pub feedback: String,
}

/// Loads any async data or disk data into the application's state types.
/// On load, the application will emit the Ready message to the root
/// application, which will then open the App.
#[tracing::instrument]
pub async fn load_app() -> LoadResult {
    // todo: do we want this?
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

    // todo: get this working without running anvil in background
    let with_counter = std::env::var("WITH_COUNTER").is_ok();
    let with_coin = std::env::var("WITH_COIN").is_ok();
    let local = match with_counter {
        true => match with_coin {
            true => {
                tracing::info!("Starting Anvil with Counter and Coin contracts");
                Local::default()
                    .with_anvil()
                    .with_dev_wallet()
                    .await
                    .with_counter()
                    .await
                    .with_coin()
                    .await
            }
            false => {
                tracing::info!("Starting Anvil with Counter contract");
                Local::default()
                    .with_anvil()
                    .with_dev_wallet()
                    .await
                    .with_counter()
                    .await
            }
        },
        false => match with_coin {
            true => {
                tracing::info!("Starting Anvil with Coin contract");
                Local::default()
                    .with_anvil()
                    .with_dev_wallet()
                    .await
                    .with_coin()
                    .await
            }
            false => {
                tracing::info!("Starting Anvil with no contracts");
                Local::default().with_anvil().with_dev_wallet().await
            }
        },
    };

    // Get the default coinlist, and if it's empty, populate it with the coin
    // contract, if there's a coin contract.
    // todo: move this logic to a better place that handles coin lists.
    let chain_id = local.anvil.as_ref().unwrap().chain_id();
    if let Some(address) = local.coin_contract {
        let mut coinlist = CoinList::load(None)?;
        if coinlist.tokens.is_empty() {
            let coin: StaticCoin = StaticCoin {
                name: "Coin".to_string(),
                symbol: "COIN".to_string(),
                address: format!("0x{:x}", address),
                decimals: 18,
                chain_id,
                logo_uri: "".to_string(),
                tags: vec![],
            };
            coinlist.tokens.push(coin);
            coinlist.save()?;
        }
    }

    let mut storage = app::Storage { profile };

    // Add the counter contract to the storage.
    let mut label = "default";
    let default_address = match local.counter_contract {
        Some(address) => {
            label = "counter";
            address
        }
        // Address from deploying counter contract in dev mode.
        None => "0x5fbdb2315678afecb367f032d93f642f64180aa3"
            .parse::<Address>()
            .unwrap(),
    };

    storage.profile.contacts.add(
        default_address,
        contacts::ContactValue {
            label: label.to_string(),
            class: contacts::Class::Contract,
            ..Default::default()
        },
        contacts::Category::Untrusted,
    );

    let coin_address = match local.coin_contract {
        Some(address) => address,
        // Address from deploying coin contract in dev mode.
        None => Address::zero(),
    };

    storage.profile.contacts.add(
        coin_address,
        contacts::ContactValue {
            label: "Coin".to_string(),
            class: contacts::Class::Contract,
            ..Default::default()
        },
        contacts::Category::Recent,
    );

    let from = local.client.as_ref().unwrap().address();
    storage.profile.contacts.add(
        from,
        contacts::ContactValue {
            label: "You".to_string(),
            class: contacts::Class::EOA,
            ..Default::default()
        },
        contacts::Category::Trusted,
    );

    let chains = app::Chains {
        local,
        arbiter: Arc::new(Mutex::new(EnvironmentBuilder::new().build())),
    };

    Ok((storage, chains))
}

/// Placeholder function for any future async calls we might want to do.
pub async fn connect_to_server() -> anyhow::Result<()> {
    Ok(())
}

impl Loader {
    pub fn new() -> (Self, Command<Message>) {
        // Triggers the next step in the main application loop by emitting the Loaded
        // message.
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
                font::load(ICON_FONT_BYTES).map(|res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to load icon font: {:?}", e);
                        return Message::LoadingFailed;
                    }

                    Message::Loaded
                }),
            ]),
        )
    }

    fn on_load(&mut self) -> Command<Message> {
        Command::perform(load_app(), Message::Ready)
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
            Message::Loaded => self.on_load(),
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            container(
                column![
                    highlight_label(self.feedback.clone()),
                    progress_bar(0.0..=1.0, self.progress.clone())
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
