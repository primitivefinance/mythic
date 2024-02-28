use std::collections::HashSet;

use bindings::atomic_v2::{LogAssetDataFilter, LogDfmmDataFilter};
use dfmm::{
    agents::{
        portfolio_management::lognormal::ln_liquidity_provider::LogNormalLiquidityProviderParameters,
        AgentParameters,
    },
    settings::parameters::Single,
};

use super::*;

pub async fn plot_heatmap(batch_data: BatchData) {
    let param_extractor = |x: &AgentParameters<Single>| match x {
        AgentParameters::LogNormalLiquidityProvider(params) => *params,
        _ => unreachable!(),
    };
    let key_extractor = |x: &LogNormalLiquidityProviderParameters<Single>| {
        (format!("{:.5}", x.sigma.0), format!("{:.5}", x.swap_fee.0))
    };
    let dataset = batch_data.get_keyed_data("lst_lp", param_extractor, key_extractor);

    let mut figure = Figure::new(
        &format!("Fee Returns for Sigma and SwapFee"),
        Some((1500, 1500)),
    );
    let mut x_values = HashSet::new();
    let mut y_values = HashSet::new();

    for (key, data_vec) in dataset.iter() {
        x_values.insert(key.0.clone());
        y_values.insert(key.1.clone());
        let final_liqs: Vec<f64> = data_vec
            .iter()
            .map(|data| {
                let dfmm_events =
                    data.get_vectorized_events::<LogDfmmDataFilter>("atomic_arbitrage");
                u256_to_f64(dfmm_events.last().unwrap().liq)
            })
            .collect();
        let average = final_liqs.iter().sum::<f64>() / final_liqs.len() as f64;
    }

    figure.create().unwrap();
}
