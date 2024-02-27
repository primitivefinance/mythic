use std::{collections::HashMap, sync::Mutex};

use bindings::atomic_v2::{LogAssetDataFilter, LogDfmmDataFilter};
use dfmm::{agents::AgentParameters, settings::parameters::Single};

use rayon::prelude::*;

use super::*;

pub async fn plot_prices(batch_data: BatchData) {
    let dataset: Mutex<HashMap<(String, String), Vec<SimulationData>>> = Mutex::new(HashMap::new());
    batch_data.data.into_par_iter().for_each(|data| {
        let agent_params = data
            .metadata
            .as_ref()
            .unwrap()
            .agent_parameters
            .get("lst_lp")
            .unwrap();
        let key = if let AgentParameters::LogNormalLiquidityProvider(ln_lp_params) = agent_params {
            (
                format!("{:.5}", ln_lp_params.sigma.0),
                format!("{:.5}", ln_lp_params.swap_fee.0),
            )
        } else {
            panic!()
        };
        let mut dataset = dataset.lock().unwrap();
        match dataset.get_mut(&key) {
            Some(data_vec) => data_vec.push(data),
            None => {
                dataset.insert(key.clone(), vec![data]);
            }
        }
    });

    let f = |x: &AgentParameters<Single>| {
        if let AgentParameters::BlockAdmin(params) = x {
            Some(*params)
        } else {
            None
        }
    };

    let dataset = dataset.lock().unwrap();

    dataset.par_iter().for_each(|(key, data_vec)| {
        println!("Key: {:?}: ", key);
        let mut figure = Figure::new(
            &format!("Sigma = {}, SwapFee = {}", key.0, key.1),
            Some((1500, 1500)),
        );

        //////////////////////////////////////////////////////////////////////////////////////////////////////////
        // TODO: Make getting this sort of thing easier
        // First get the average over the price paths and plot statistical
        let (mut x_data, mut y_data) = (vec![], vec![]);
        let mut get_x_data = true;
        for data in data_vec.iter() {
            let events = data.get_vectorized_events::<LogAssetDataFilter>("atomic_arbitrage");
            let prices = events
                .iter()
                .map(|ev| u256_to_f64(ev.lex_price))
                .collect::<Vec<f64>>();
            y_data.push(prices);
            if get_x_data {
                x_data = events
                    .iter()
                    .map(|ev| ev.timestamp.as_u64() as f64)
                    .collect::<Vec<_>>();
            }
            get_x_data = false;
        }
        let settings = PlotSettings::new()
            .labels("Time", "Price")
            .title("Market Price");
        let plot = StatisticalPlot::new(x_data, y_data).settings(settings);
        figure.add_plot(plot);
        //////////////////////////////////////////////////////////////////////////////////////////////////////////

        //////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Get the LN prices
        let (mut x_data, mut y_data) = (vec![], vec![]);
        let mut get_x_data = true;
        for data in data_vec.iter() {
            let events = data.get_vectorized_events::<LogDfmmDataFilter>("atomic_arbitrage");
            let prices = events
                .iter()
                .map(|ev| u256_to_f64(ev.price))
                .collect::<Vec<f64>>();
            y_data.push(prices);
            if get_x_data {
                x_data = events
                    .iter()
                    .map(|ev| ev.timestamp.as_u64() as f64)
                    .collect::<Vec<_>>();
            }
            get_x_data = false;
        }
        let settings = PlotSettings::new()
            .labels("Time", "Price")
            .title("LogNormal Path");
        let plot = StatisticalPlot::new(x_data, y_data).settings(settings);
        figure.add_plot(plot);
        //////////////////////////////////////////////////////////////////////////////////////////////////////////

        //////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Plot the LP liquidity over time
        let (mut x_data, mut y_data) = (vec![], vec![]);
        let mut get_x_data = true;
        for data in data_vec.iter() {
            let events = data.get_vectorized_events::<LogDfmmDataFilter>("atomic_arbitrage");
            let prices = events
                .iter()
                .map(|ev| u256_to_f64(ev.liq))
                .collect::<Vec<f64>>();
            y_data.push(prices);
            if get_x_data {
                x_data = events
                    .iter()
                    .map(|ev| ev.timestamp.as_u64() as f64)
                    .collect::<Vec<_>>();
            }
            get_x_data = false;
        }
        let settings = PlotSettings::new()
            .labels("Time", "Liquidity")
            .title("LogNormal Liquidity");
        let plot = StatisticalPlot::new(x_data, y_data).settings(settings);
        figure.add_plot(plot);
        //////////////////////////////////////////////////////////////////////////////////////////////////////////

        figure.create().unwrap();
    });
}
