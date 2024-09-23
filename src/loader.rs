use iced::{
    font,
    widget::{canvas::Cache, column, container, progress_bar},
    Length,
};
use iced_aw::graphics::icons::ICON_FONT_BYTES;
use std::time::Instant;

use crate::{
    app::AnvilSave,
    components::{
        logos::PhiLogo,
        progress::CustomProgressBar,
        system::{label, ExcaliburContainer},
    },
    data::user::Saveable,
};

type LoadResult =
    anyhow::Result<(Model, Arc<middleware::ExcaliburMiddleware<Ws, LocalWallet>>), anyhow::Error>;

use self::{
    data::contacts,
    middleware::{start_anvil, ExcaliburMiddleware},
};
use super::*;

#[derive(Debug)]
pub enum LoaderMessage {
    View,
    Tick,
    Loaded(Flags),
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

#[tracing::instrument(level = "debug")]
pub fn load_user_data() -> anyhow::Result<Model> {
    let model = Model::load(None);
    let model = match model {
        Ok(model) => model,
        Err(e) => {
            tracing::warn!("Failed to load model: {:?}", e);
            tracing::info!("Creating a new default model.");

            Model::create_new(None)?
        }
    };

    tracing::debug!(
        "Loaded model {:?} at path {:?}",
        model.user.name,
        model.file_path()
    );

    Ok(model)
}

const CONTRACT_NAMES: [&str; 6] = [
    "protocol", "strategy", "token_x", "token_y", "lex", "solver",
];

#[tracing::instrument(level = "debug")]
pub async fn load_app(flags: Flags) -> LoadResult {
    let mut model = load_user_data()?;

    let mut exc_client = ExcaliburMiddleware::new(None, None).await?;

    let anvil = start_anvil(None)?;
    exc_client.connect_anvil(anvil).await?;

    let chain_id = if let Some(anvil) = &exc_client.anvil {
        anvil.chain_id()
    } else {
        31337
    };

    let client = exc_client.get_client();

    model.connect_to_network(client.clone()).await?;

    let loaded_snapshot = if let Some(AnvilSave {
        snapshot,
        block_number,
    }) = &model.user.anvil_snapshot
    {
        tracing::debug!(
            "Attempting to load snapshot: {}",
            &snapshot[..10.min(snapshot.len())],
        );

        let success = client
            .clone()
            .provider()
            .request::<[String; 1], bool>("anvil_loadState", [snapshot.to_string()])
            .await
            .expect("Failed to load snapshot.");

        tracing::info!("Loaded snapshot: {:?}", success);

        if success {
            tracing::info!("Syncing Anvil to block: {}", block_number);
            client
                .clone()
                .provider()
                .request::<[u64; 1], ()>("anvil_mine", [*block_number])
                .await
                .expect("Failed to sync to block.");
        }

        success
    } else {
        false
    };

    if loaded_snapshot {
        for name in CONTRACT_NAMES.iter() {
            if let Some(contracts) = model
                .user
                .contacts
                .get_class_list(contacts::Class::Contract)
            {
                for (address, contact) in contracts.get_all() {
                    if contact.label == *name {
                        exc_client.add_contract(name, *address);
                    }
                }
            }
        }
    }

    let signer = exc_client.signer.clone().unwrap().with_chain_id(chain_id);
    let sender = signer.address();
    exc_client.connect_signer(signer).await?;

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
        model.save()?;
    }

    Ok((model, Arc::new(exc_client)))
}

pub async fn connect_to_server() -> anyhow::Result<()> {
    Ok(())
}

pub const DAGGER_SQUARE_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/DAGGERSQUARE.otf");

impl Loader {
    pub fn new(flags: Flags) -> (Self, Command<LoaderMessage>) {
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
                        return LoaderMessage::LoadingFailed;
                    }

                    LoaderMessage::Connected
                }),
                font::load(ICON_FONT_BYTES).map(move |res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to load icon font: {:?}", e);
                        return LoaderMessage::LoadingFailed;
                    }

                    LoaderMessage::IconFontLoaded
                }),
                font::load(DAGGER_SQUARE_FONT_BYTES).map(move |res| {
                    if let Err(e) = res {
                        tracing::error!("Failed to load icon font: {:?}", e);
                        return LoaderMessage::LoadingFailed;
                    }

                    LoaderMessage::BrandFontLoaded
                }),
                Command::perform(async {}, move |_| LoaderMessage::Loaded(flags)),
            ]),
        )
    }

    fn load(&mut self, flags: Flags) -> Command<LoaderMessage> {
        Command::perform(load_app(flags), LoaderMessage::Ready)
    }

    pub fn update(&mut self, message: LoaderMessage) -> Command<LoaderMessage> {
        self.logo.cache.clear();

        match message {
            LoaderMessage::Tick => {
                self.feedback = self.get_progress_feedback();

                if !self.screen_open {
                    return Command::none();
                }

                self.load_ticks += 1.0;

                self.progress = self.load_ticks / self.max_load_ticks;

                Command::none()
            }
            LoaderMessage::Connected => {
                self.screen_open = true;
                self.load_ticks = 0.0;
                Command::none()
            }
            LoaderMessage::Loaded(flags) => self.load(flags),
            _ => Command::none(),
        }
    }

    pub fn get_progress_feedback(&self) -> String {
        let s_curve_result = s_curve(self.progress);
        let progress = (s_curve_result * 4.0) as usize;

        match progress {
            0 => "Initiated loading procedure...".to_string(),
            1 => "Starting sandbox environment...".to_string(),
            2 => "Connected. Deploying contracts in sandbox...".to_string(),
            3 => "Initializing sandbox state...".to_string(),
            _ => "Launching Mythic...".to_string(),
        }
    }

    pub fn view(&self) -> Element<LoaderMessage> {
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

    pub fn subscription(&self) -> Subscription<LoaderMessage> {
        iced::time::every(std::time::Duration::from_millis(25)).map(|_| LoaderMessage::Tick)
    }
}

pub const GREEK_SYMBOLS: [char; 10] = ['Γ', 'Δ', 'Θ', 'Λ', 'Ξ', 'Π', 'Σ', 'Φ', 'Ψ', 'Ω'];
pub const CURRENCY_SYMBOLS: [char; 11] = ['$', '€', '£', '¥', '₩', '₿', '₽', '₹', '₺', '₴', 'Ξ'];

pub fn s_curve(x: f32) -> f32 {
    let sigmoid_x = 1.0 / (1.0 + (-x).exp());
    (sigmoid_x - 0.5) * 2.0
}
