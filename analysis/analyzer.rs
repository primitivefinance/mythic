use std::{collections::HashMap, fs, path::Path};

use analysis::{
    reader::SimulationData,
    visualize::{
        plots::{statistical::StatisticalPlot, PlotSettings},
        Figure,
    },
};
use bindings::atomic_v2::{LogAssetDataFilter, LogDfmmDataFilter};
use dfmm::agents::AgentParameters;
use ethers::utils::format_ether;

pub fn main() {
    let files = read_dir("analysis/stable/sweep").unwrap();
    // TODO: Turn this into a nice function
    let mut dataset: HashMap<(String, String), Vec<SimulationData>> = HashMap::new();
    for file in files {
        let data = SimulationData::new(&file).unwrap();
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
        match dataset.get_mut(&key) {
            Some(data_vec) => data_vec.push(data),
            None => {
                dataset.insert(key.clone(), vec![data]);
            }
        }
    }

    println!("{:?}", dataset.keys().collect::<Vec<_>>());

    for (key, data_vec) in dataset.iter() {
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
    }
}

pub fn read_dir(dir: &str) -> Result<Vec<String>, String> {
    let path = Path::new(dir);

    // Check if the path exists and is a directory
    if !path.is_dir() {
        return Err("Path does not exist or is not a directory".to_string());
    }

    let mut file_paths = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => file_paths.push(e.path().to_str().unwrap().to_owned()),
                    Err(err) => return Err(format!("Error reading entry: {}", err)),
                }
            }
        }
        Err(err) => return Err(format!("Error reading directory: {}", err)),
    }

    Ok(file_paths)
}

fn u256_to_f64(value: ethers::types::U256) -> f64 {
    let str = format_ether(value);
    str.parse::<f64>().unwrap()
}
