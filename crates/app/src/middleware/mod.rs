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

pub fn s_curve(x: f32) -> f32 {
    let sigmoid_x = 1.0 / (1.0 + (-x).exp());
    (sigmoid_x - 0.5) * 2.0
}

#[cfg(test)]
mod tests {

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
}
