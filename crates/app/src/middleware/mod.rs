use ethers::{core::k256::ecdsa::SigningKey, utils::AnvilInstance};

use super::*;

pub async fn connect_call_client(url: String) -> anyhow::Result<Provider<Http>> {
    let client = Provider::<Http>::try_from(&url).unwrap();
    Ok(client)
}

pub async fn connect_sub_client(url: String) -> anyhow::Result<Provider<Ws>> {
    let client = Provider::<Ws>::connect(&url).await?;
    Ok(client)
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
