use std::{collections::HashMap, fmt::Debug};

use anyhow::{Error, Result};
use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};
use bindings::mock_erc20::MockERC20;
use clients::{dev::ProtocolPosition, protocol::ProtocolClient};
use ethers::{
    core::k256::ecdsa::SigningKey,
    utils::{Anvil, AnvilInstance},
};

pub mod alloyed;
pub mod watch;

use super::*;
use crate::screens::portfolio::dashboard::portfolio_model::EthersAddress;

pub const SANDBOX_LABEL: &str = "sandbox";

/// List of available networks to connect to.
/// Local is the default http://localhost:8545.
#[derive(Clone, Debug)]
pub enum ExcaliburNetworks {
    Local,
    Mainnet,
}

#[derive(Clone, Debug)]
pub enum ExcaliburEnvironments {
    Sandbox,
    Anvil,
    Network(ExcaliburNetworks),
}

/// Connects users to networks.
pub struct ExcaliburMiddleware<P: PubsubClient, S: Signer> {
    /// ARBITER
    pub sandbox: Environment,
    /// ARBITER
    pub sandbox_client: Arc<RevmMiddleware>,
    /// ANVIL
    pub anvil: Option<AnvilInstance>,
    /// ANVIL
    pub anvil_client: Option<Arc<SignerMiddleware<Provider<P>, S>>>,
    /// ANY
    pub providers: Vec<Arc<SignerMiddleware<Provider<P>, S>>>,
    /// ANY
    pub signers: Vec<S>,
    /// ANY
    pub contracts: HashMap<String, EthersAddress>,
}

/// Standard client that excalibur uses.
pub type NetworkClient<P, S> = SignerMiddleware<Provider<P>, S>;

impl std::fmt::Debug for ExcaliburMiddleware<Ws, LocalWallet> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExcaliburMiddleware")
            .field("sandbox_client", &self.sandbox_client)
            .field("anvil_client", &self.anvil_client)
            .field("providers", &self.providers)
            .field("signers", &self.signers)
            .finish()
    }
}

impl<P: PubsubClient, S: Signer> ExcaliburMiddleware<P, S> {
    pub fn add_contract(&mut self, name: &str, address: EthersAddress) {
        self.contracts.insert(name.to_string(), address);
    }

    /// Returns the sandbox environment client.
    pub fn sandbox(&self) -> Arc<RevmMiddleware> {
        self.sandbox_client.clone()
    }

    /// Returns the first provider in the list.
    pub fn client(&self) -> Option<&Arc<NetworkClient<P, S>>> {
        self.providers.first()
    }

    /// Returns the first signer in the list.
    pub fn signer(&self) -> Option<&S> {
        self.signers.first()
    }

    /// Returns the address of the signer, if there is one.
    pub fn address(&self) -> Option<Address> {
        if let Some(signer) = self.signer() {
            Some(signer.address())
        } else {
            None
        }
    }

    /// Executes the `anvil_dumpState` rpc call on the anvil instance.
    pub async fn snapshot(&self) -> anyhow::Result<String> {
        let anvil_client = self.anvil_client.as_ref().unwrap();

        let params: Vec<String> = vec![];
        let snapshot = anvil_client
            .provider()
            .request("anvil_dumpState", params)
            .await
            .expect("failed to get snapshot");

        Ok(snapshot)
    }
}

impl ExcaliburMiddleware<Ws, LocalWallet> {
    pub async fn new(
        anvil: Option<AnvilInstance>,
        sandbox: Environment,
        signer: LocalWallet,
    ) -> anyhow::Result<Self> {
        let sandbox_client = RevmMiddleware::new(&sandbox, Some(SANDBOX_LABEL))?;

        let mut anvil_client = None;
        if let Some(anvil_instance) = anvil.as_ref() {
            anvil_client = Some(Arc::new(
                Provider::<Ws>::connect(&anvil_instance.ws_endpoint())
                    .await?
                    .with_signer(signer.clone().with_chain_id(anvil_instance.chain_id())),
            ));
        }

        let signers = vec![signer];

        let mut providers = vec![];
        if let Some(anvil_client) = anvil_client.clone() {
            providers.push(anvil_client);
        }

        Ok(Self {
            sandbox,
            sandbox_client,
            anvil,
            anvil_client,
            providers,
            signers,
            contracts: HashMap::new(),
        })
    }

    /// Starts the sandbox environment.
    /// If `dev` is true, it will also start an anvil instance and connect a
    /// provider to it.
    pub async fn setup(dev: bool) -> anyhow::Result<Self> {
        if dev {
            let anvil = Anvil::default()
                .arg("--gas-limit")
                .arg("20000000")
                .chain_id(31337_u64)
                .spawn();

            let signer = LocalWallet::from(anvil.keys()[0].clone());
            let sandbox = EnvironmentBuilder::new().build();
            let sandbox_client = RevmMiddleware::new(&sandbox, Some(SANDBOX_LABEL))?;
            let anvil_client = Arc::new(
                Provider::<Ws>::connect(&anvil.ws_endpoint())
                    .await?
                    .with_signer(signer.clone().with_chain_id(anvil.chain_id())),
            );
            let signers = vec![signer];
            let providers = vec![anvil_client.clone()];

            Ok(Self {
                sandbox,
                sandbox_client,
                anvil: Some(anvil),
                anvil_client: Some(anvil_client),
                providers,
                signers,
                contracts: HashMap::new(),
            })
        } else {
            let sandbox = EnvironmentBuilder::new().build();
            let sandbox_client = RevmMiddleware::new(&sandbox, Some(SANDBOX_LABEL))?;
            let providers = vec![];
            let signers = vec![];

            Ok(Self {
                anvil: None,
                anvil_client: None,
                sandbox,
                sandbox_client,
                providers,
                signers,
                contracts: HashMap::new(),
            })
        }
    }

    pub fn add_signer(&mut self, signer: LocalWallet) -> anyhow::Result<()> {
        self.signers.push(signer);
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait Protocol {
    fn protocol(&self) -> Result<ProtocolClient<NetworkClient<Ws, LocalWallet>>>;

    async fn create_position(
        &self,
        sender: Address,
        amount_dollars: f64,
        price: f64,
        strike_price_wad: f64,
        sigma_percent_wad: f64,
        tau_years_wad: f64,
    ) -> anyhow::Result<Option<TransactionReceipt>>;

    async fn get_position(&self) -> anyhow::Result<ProtocolPosition>;
}

pub async fn connect_call_client(url: String) -> anyhow::Result<Provider<Http>> {
    let client = Provider::<Http>::try_from(&url).unwrap();
    Ok(client)
}

pub async fn connect_sub_client(url: String) -> anyhow::Result<Provider<Ws>> {
    let client = Provider::<Ws>::connect(&url).await?;
    Ok(client)
}

#[async_trait::async_trait]
impl Protocol for ExcaliburMiddleware<Ws, LocalWallet> {
    fn protocol(&self) -> Result<ProtocolClient<NetworkClient<Ws, LocalWallet>>> {
        let client = self.client().unwrap().clone();
        let address = self.contracts.get("protocol").unwrap().clone();
        let protocol = ProtocolClient::new(client, address);
        Ok(protocol)
    }

    async fn create_position(
        &self,
        sender: Address,
        amount_dollars: f64,
        price: f64,
        strike_price_wad: f64,
        sigma_percent_wad: f64,
        tau_years_wad: f64,
    ) -> anyhow::Result<Option<TransactionReceipt>> {
        let client = self.anvil_client.clone().unwrap();
        let protocol = self.protocol()?;

        let amount_x = amount_dollars / price;
        let amount_y = amount_x * price;
        let amount_x_wad = ethers::utils::parse_ether(amount_x).unwrap();
        let amount_y_wad = ethers::utils::parse_ether(amount_y).unwrap();

        let (token_x, token_y) = protocol.get_tokens().await?;
        let (token_x, token_y) = (
            MockERC20::new(token_x, client.clone()),
            MockERC20::new(token_y, client.clone()),
        );

        token_x.mint(sender, amount_x_wad).send().await?;
        token_y.mint(sender, amount_y_wad).send().await?;

        Ok(protocol
            .initialize(
                price,
                amount_x,
                strike_price_wad,
                sigma_percent_wad,
                tau_years_wad,
            )
            .await?)
    }

    async fn get_position(&self) -> anyhow::Result<ProtocolPosition> {
        let client = self.client().unwrap().clone();
        let protocol = self.protocol()?;
        let (balance_x, balance_y, liquidity) = protocol.get_reserves_and_liquidity().await?;
        let internal_price = protocol.get_internal_price().await?;
        let balance_x = ethers::utils::format_ether(balance_x);
        let balance_y = ethers::utils::format_ether(balance_y);
        let liquidity = ethers::utils::format_ether(liquidity);
        let internal_price = ethers::utils::format_ether(internal_price);

        let balance_x = format!("{:.2}", balance_x.parse::<f64>().unwrap());
        let balance_y = format!("{:.2}", balance_y.parse::<f64>().unwrap());
        let liquidity = format!("{:.2}", liquidity.parse::<f64>().unwrap());
        let internal_price = format!("{:.2}", internal_price.parse::<f64>().unwrap());

        Ok(ProtocolPosition {
            balance_x: Some(balance_x),
            balance_y: Some(balance_y),
            liquidity: Some(liquidity),
            internal_price: Some(internal_price),
        })
    }
}

pub async fn from_anvil(
    anvil: &Arc<AnvilInstance>,
) -> anyhow::Result<(Vec<Provider<Ws>>, Vec<Wallet<SigningKey>>)> {
    let mut clients = Vec::new();
    let mut wallets = Vec::new();

    let wallet: LocalWallet = anvil
        .keys()
        .first()
        .expect("no keys in anvil")
        .clone()
        .into();

    let wallet = wallet.with_chain_id(anvil.chain_id());
    let url = anvil.endpoint();
    let url = url.replace("http", "ws");

    let provider = connect_sub_client(url)
        .await
        .expect("failed to connect to anvil");

    clients.push(provider);
    wallets.push(wallet);

    Ok((clients, wallets))
}

pub fn s_curve(x: f32) -> f32 {
    let sigmoid_x = 1.0 / (1.0 + (-x).exp());
    (sigmoid_x - 0.5) * 2.0
}

#[cfg(test)]
mod tests {

    use ethers::utils::{format_ether, Anvil};

    use super::*;

    async fn setup() -> anyhow::Result<AnvilInstance> {
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        Ok(anvil)
    }

    #[test]
    fn test_s_curve() {
        let mut t = 0.0;
        while t < 1.0 {
            let s_curve = super::s_curve(t);
            println!("s_curve: {} {}", t, s_curve);
            assert!(s_curve >= 0.0);
            assert!(s_curve <= 1.0);
            t += 0.01;
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_excalibur_local() -> anyhow::Result<()> {
        let client = ExcaliburMiddleware::setup(true).await?;

        let anvil = client.anvil.as_ref().unwrap();

        let bob: LocalWallet = anvil
            .keys()
            .first()
            .expect("no keys in anvil")
            .clone()
            .into();

        let alice_wallet = LocalWallet::from(anvil.keys()[1].clone());
        let bob = bob.with_chain_id(anvil.chain_id());

        let alice_address = alice_wallet.address();
        let bob_address = bob.address();

        let balance = client
            .client()
            .unwrap()
            .get_balance(bob_address, None)
            .await
            .unwrap();

        let alice_balance = client
            .client()
            .unwrap()
            .get_balance(alice_address, None)
            .await
            .unwrap();

        println!("balance: {}", format_ether(balance));
        println!("alice balance: {}", format_ether(alice_balance));

        let pay_tx = TransactionRequest::pay(
            alice_address,
            ethers::types::U256::from(1_000_000_000_000_000_000u128),
        );

        // do the tx
        let tx = client
            .anvil_client
            .clone()
            .unwrap()
            .send_transaction(pay_tx, None)
            .await?
            .await?;

        let balance = client
            .client()
            .unwrap()
            .get_balance(bob_address, None)
            .await
            .unwrap();

        let alice_balance = client
            .client()
            .unwrap()
            .get_balance(alice_address, None)
            .await
            .unwrap();

        println!("balance: {}", format_ether(balance));
        println!("alice balance: {}", format_ether(alice_balance));

        Ok(())
    }
}
