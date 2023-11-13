// use crate::visualize::statistical::plot_statistical;
use arbiter_core::bindings::liquid_exchange;

use super::*;
use crate::{
    reader::SimulationData,
    visualize::{
        plots::{line::LinePlot, statistical::StatisticalPlot, PlotSettings},
        Figure,
    },
};

#[test]
fn read_in_and_plot_statistical() {
    let file1 = "src/tests/output/0.json";
    let data1 = SimulationData::new(file1).unwrap();
    let values1 = data1.get_vectorized_events::<liquid_exchange::PriceChangeFilter>("lex");

    let file2 = "src/tests/output/1.json";
    let data2 = SimulationData::new(file2).unwrap();
    let values2 = data2.get_vectorized_events::<liquid_exchange::PriceChangeFilter>("lex");

    let (x_data, y_data1): (Vec<f64>, Vec<f64>) = values1
        .iter()
        .enumerate()
        .map(|(index, event)| (index as f64, wad_to_float(event.price)))
        .unzip();

    let y_data2 = values2
        .iter()
        .map(|event| wad_to_float(event.price))
        .collect();

    let y_data = vec![y_data1, y_data2];
    let statistical_plot = StatisticalPlot::new(x_data, y_data);

    let mut figure = Figure::new("test_read_in_and_plot_statistical", None);
    figure.add_plot(statistical_plot);
    figure.create().unwrap();
    assert!(std::path::Path::new("test_read_in_and_plot_statistical.png").exists());
    std::fs::remove_file("test_read_in_and_plot_statistical.png").unwrap();
}

#[test]
#[ignore]
fn plot_dca_weights() {
    let file1 = "dca/static/1.json";
    let data1 = SimulationData::new(file1).unwrap();
    let weight_filter = data1.get_vectorized_events::<g3m::LogSyncingWeightFilter>("g3m");
    let indices: Vec<f64> = weight_filter
        .iter()
        .enumerate()
        .map(|(index, _)| index as f64)
        .collect();

    // Plot the weights
    let plot_settings = PlotSettings::new()
        .title("DCA Weights")
        .labels("Index", "Weight X");

    let line_plot = LinePlot::new(
        indices.clone(),
        weight_filter
            .iter()
            .map(|event| wad_to_float(event.weight_x))
            .collect(),
    )
    .settings(plot_settings);

    // Plot the reserves
    let plot_settings = PlotSettings::new()
        .title("Reserves")
        .labels("Index", "Reserve X");
    let reserves = data1.get_vectorized_events::<g3m::LogReservesFilter>("g3m");
    let reserves_plot = LinePlot::new(
        indices.clone(),
        reserves
            .iter()
            .map(|event| wad_to_float(event.reserve_x))
            .collect(),
    )
    .settings(plot_settings);

    // Plot the prices
    let plot_settings = PlotSettings::new().title("Prices").labels("Index", "Price");
    let swap_filter = data1.get_vectorized_events::<g3m::SwapFilter>("g3m");
    let prices_plot = LinePlot::new(
        indices.clone(),
        swap_filter
            .iter()
            .map(|event| wad_to_float(event.new_price))
            .collect(),
    )
    .settings(plot_settings);

    // Plot the portfolio value
    let plot_settings = PlotSettings::new()
        .title("Portfolio Value")
        .labels("Index", "Portfolio Value");
    let portfolio_value = reserves
        .iter()
        .zip(swap_filter)
        .map(|(event, price_event)| {
            let reserve_x = wad_to_float(event.reserve_x);
            let reserve_y = wad_to_float(event.reserve_y);
            let price = wad_to_float(price_event.new_price);
            reserve_x * price + reserve_y
        });
    let portfolio_value_plot =
        LinePlot::new(indices, portfolio_value.collect()).settings(plot_settings);

    let mut figure = Figure::new("plot_dca_weights", Some((1920, 1080)));
    figure.add_plot(line_plot);
    figure.add_plot(reserves_plot);
    figure.add_plot(prices_plot);
    figure.add_plot(portfolio_value_plot);
    figure.create().unwrap();
}
