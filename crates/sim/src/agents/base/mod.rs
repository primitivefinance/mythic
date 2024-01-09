pub mod block_admin;
pub mod price_changer;
pub mod token_admin;

use anyhow::Result;
use tracing::{debug, trace};

use super::*;

// https://www.coingecko.com/en/methodology data source and transparency
// API docs: https://www.coingecko.com/api/documentation
// supports up to 9 years of data
pub async fn get_historical_daily_prices(number_of_days: usize) -> Result<Vec<f64>> {
    let eth_historical_price = tokio::task::spawn_blocking(move || {
        rust_gecko::coins::market_chart(
            "ethereum",
            "usd",
            (number_of_days - 1).to_string().as_str(),
            Some("daily"),
        )
    })
    .await?;

    if !eth_historical_price.is_success {
        return Err(anyhow::anyhow!(
            "request failed with status code: {:#?}",
            eth_historical_price.error
        ));
    }

    let json = eth_historical_price.json.clone().unwrap();
    // can also parse the daily market caps and total volumes from this repsonse
    let prices: Vec<_> = json
        .get("prices")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_array().unwrap()[1].as_f64().unwrap())
        .collect();
    Ok(prices)
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[tokio::test]
    async fn test_get_historical_daily_prices() {
        let prices = get_historical_daily_prices(5).await.unwrap();
        assert_eq!(prices.len(), 5);
    }
}
