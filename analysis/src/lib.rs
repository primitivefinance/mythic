use std::{collections::BTreeMap, fs::File, io::BufReader};

use anyhow::Result;
use arbiter_bindings::bindings::liquid_exchange;
use ethers::types::U256;
use reader::SimulationData;
use serde_json::{from_reader, Value};
pub use simulation::bindings::*;
use tracing::{debug, info};
use visualize::{
    plots::{statistical::StatisticalPlot, PlotSettings},
    Figure,
};

#[allow(unused)]
pub mod reader;
#[cfg(test)]
mod tests;
pub mod unpacker;
#[allow(unused)]
pub mod visualize;

pub fn wad_to_float(wad: U256) -> f64 {
    wad.as_u128() as f64 / 10f64.powi(18)
}

pub fn plot_dca_weights(data_set: &[SimulationData], name: &str) {
    let mut weights_statistical = (vec![], vec![]);
    let mut reserves_statistical = (vec![], vec![]);
    let mut prices_statistical = (vec![], vec![]);
    let mut portfolio_value_statistical = (vec![], vec![]);
    let mut swapper_reserves_statistical = (vec![], vec![]);
    let mut swapper_portfolio_value_statistical = (vec![], vec![]);

    for (idx, data) in data_set.iter().enumerate() {
        // Get the weights and indices for the plots
        let weight_filter = data.get_vectorized_events::<g3m::LogSyncingWeightFilter>("g3m");
        let indices: Vec<f64> = weight_filter
            .iter()
            .enumerate()
            .map(|(index, _)| index as f64)
            .collect();
        if idx == 0 {
            weights_statistical.0 = indices.clone();
        }
        weights_statistical.1.push(
            weight_filter
                .iter()
                .map(|event| wad_to_float(event.weight_x))
                .collect::<Vec<f64>>(),
        );

        // Get the reserves
        let reserves = data.get_vectorized_events::<g3m::LogReservesFilter>("g3m");
        if idx == 0 {
            reserves_statistical.0 = indices.clone();
        }
        reserves_statistical.1.push(
            reserves
                .iter()
                .map(|event| wad_to_float(event.reserve_x))
                .collect::<Vec<f64>>(),
        );

        // Get the prices
        let swap_filter = data.get_vectorized_events::<g3m::LogPricesFilter>("g3m");
        if idx == 0 {
            prices_statistical.0 = indices.clone();
        }
        prices_statistical.1.push(
            swap_filter
                .iter()
                .map(|event| wad_to_float(event.spot_price))
                .collect::<Vec<f64>>(),
        );

        // Get the portfolio values
        let portfolio_value = reserves
            .iter()
            .zip(swap_filter)
            .map(|(event, price_event)| {
                let reserve_x = wad_to_float(event.reserve_x);
                let reserve_y = wad_to_float(event.reserve_y);
                let price = wad_to_float(price_event.spot_price);
                reserve_x * price + reserve_y
            });
        if idx == 0 {
            portfolio_value_statistical.0 = indices.clone();
        }
        portfolio_value_statistical
            .1
            .push(portfolio_value.collect::<Vec<f64>>());

        // Get the swapper's reserves
        let swapper_portfolio = data
            .get_vectorized_events::<portfolio_tracker::LogPortfolioFilter>("portfolio_tracker");
        if idx == 0 {
            swapper_reserves_statistical.0 = indices.clone();
        }
        swapper_reserves_statistical.1.push(
            swapper_portfolio
                .iter()
                .map(|event| wad_to_float(event.token_x_balance))
                .collect::<Vec<f64>>(),
        );

        // Get the swapper's portfolio
        let price_change_filter =
            data.get_vectorized_events::<liquid_exchange::PriceChangeFilter>("lex");

        let swapper_portfolio_value_plot = swapper_portfolio
            .iter()
            .zip(price_change_filter)
            .map(|(event, price_change_event)| {
                let x_balance = wad_to_float(event.token_x_balance);
                let y_balance = wad_to_float(event.token_y_balance);
                let price = wad_to_float(price_change_event.price);
                x_balance * price + y_balance
            })
            .collect::<Vec<f64>>();
        if idx == 0 {
            swapper_portfolio_value_statistical.0 = indices.clone();
        }
        swapper_portfolio_value_statistical
            .1
            .push(swapper_portfolio_value_plot);
    }
    // Create the figure
    let mut figure = Figure::new(name, Some((2000, 2000)));

    // Plot the prices
    let plot_settings = PlotSettings::new().title("Prices").labels("Index", "Price");
    let prices_plot =
        StatisticalPlot::new(prices_statistical.0, prices_statistical.1).settings(plot_settings);
    figure.add_plot(prices_plot);

    // Plot the weights
    let plot_settings = PlotSettings::new()
        .title("LP DCA Weights")
        .labels("Index", "Weight X");

    let weights_plot =
        StatisticalPlot::new(weights_statistical.0, weights_statistical.1).settings(plot_settings);
    figure.add_plot(weights_plot);

    // Plot the reserves
    let plot_settings = PlotSettings::new()
        .title("LP Reserves")
        .labels("Index", "Reserve X");
    let reserves_plot = StatisticalPlot::new(reserves_statistical.0, reserves_statistical.1)
        .settings(plot_settings);
    figure.add_plot(reserves_plot);

    // Plot the portfolio value
    let plot_settings = PlotSettings::new()
        .title("LP Portfolio Value")
        .labels("Index", "Portfolio Value");
    let portfolio_value_plot =
        StatisticalPlot::new(portfolio_value_statistical.0, portfolio_value_statistical.1)
            .settings(plot_settings);
    figure.add_plot(portfolio_value_plot);

    // Plot the swapper reserves
    let plot_settings = PlotSettings::new()
        .title("Swapper Reserves")
        .labels("Index", "Swapper Reserve X");
    let swapper_reserves_plot = StatisticalPlot::new(
        swapper_reserves_statistical.0,
        swapper_reserves_statistical.1,
    )
    .settings(plot_settings);
    figure.add_plot(swapper_reserves_plot);

    // Plot the swapper portfolio value
    let plot_settings = PlotSettings::new()
        .title("Swapper Portfolio Value")
        .labels("Index", "Swapper Portfolio Value");
    let swapper_portfolio_value_plot = StatisticalPlot::new(
        swapper_portfolio_value_statistical.0,
        swapper_portfolio_value_statistical.1,
    )
    .settings(plot_settings);
    figure.add_plot(swapper_portfolio_value_plot);

    figure.create().unwrap();
}
