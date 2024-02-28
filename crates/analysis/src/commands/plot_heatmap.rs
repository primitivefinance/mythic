use std::collections::{HashMap, HashSet};

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

    let mut figure = Figure::new("Fee Returns for Sigma and SwapFee", Some((1500, 1500)));

    let mut averages = HashMap::new();

    for (key, data_vec) in dataset.iter() {
        let final_liqs: Vec<f64> = data_vec
            .iter()
            .map(|data| {
                let dfmm_events =
                    data.get_vectorized_events::<LogDfmmDataFilter>("atomic_arbitrage");
                u256_to_f64(dfmm_events.last().unwrap().liq)
            })
            .collect();
        let average = final_liqs.iter().sum::<f64>() / final_liqs.len() as f64;
        averages.insert(key.clone(), average);
    }

    let settings = PlotSettings::new()
        .labels("Sigma", "SwapFee")
        .title("Fee Returns for Sigma and SwapFee");
    let heatmap = hashmap_to_heatmap(averages).settings(settings);
    figure.add_plot(heatmap);

    figure.create().unwrap();
}

fn hashmap_to_heatmap(map: HashMap<(String, String), f64>) -> HeatMapPlot {
    let mut column_data = HashSet::new();
    let mut row_data = HashSet::new();
    for (key, _) in map.iter() {
        column_data.insert(key.0.clone());
        row_data.insert(key.1.clone());
    }

    let mut column_data: Vec<f64> = column_data
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut row_data: Vec<f64> = row_data.into_iter().map(|x| x.parse().unwrap()).collect();

    column_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    row_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mut value = vec![vec![0.0; column_data.len()]; row_data.len()];

    for ((sigma, swap_fee), val) in map.iter() {
        let sigma_val = sigma.parse::<f64>().expect("Failed to parse sigma");
        let swap_fee_val = swap_fee.parse::<f64>().expect("Failed to parse swap fee");

        // Find the index with an epsilon comparison
        let epsilon = 1e-12; // Define an appropriate epsilon for your use case
        let x_index = column_data
            .iter()
            .position(|&x| (x - sigma_val).abs() < epsilon)
            .expect("Sigma not found in column_data");
        let y_index = row_data
            .iter()
            .position(|&x| (x - swap_fee_val).abs() < epsilon)
            .expect("Swap fee not found in row_data");

        value[y_index][x_index] = *val;
    }

    HeatMapPlot::new(column_data, row_data, value)
}
