use analysis::{
    reader::SimulationData,
    visualize::{
        plots::{line::LinePlot, PlotSettings},
        Figure,
    },
};
use bindings::atomic_v2::{
    LogArbDataFilter, LogAssetDataFilter, LogDfmmDataFilter, PriceFilter, ProfitFilter,
};
use ethers::utils::format_ether;

fn u256_to_f64(value: ethers::types::U256) -> f64 {
    let str = format_ether(value);
    str.parse::<f64>().unwrap()
}

pub fn main() {
    let mut figure = Figure::new("test", Some((1500, 1500)));
    let data = SimulationData::new("analysis/lst/0.json").unwrap();

    // Get the vectorized events for the dfmm contract
    let atomic_arb_price_events =
        data.get_vectorized_events::<LogDfmmDataFilter>("atomic_arbitrage");
    let (x_data, y_data) = atomic_arb_price_events
        .into_iter()
        .enumerate()
        .map(|(idx, ev)| (idx as f64, u256_to_f64(ev.price)))
        .unzip();
    let settings = PlotSettings::new()
        .labels("Index", "Atomic Arb Price")
        .title("LN Price Path");
    let plot = LinePlot::new(x_data, y_data).settings(settings);
    figure.add_plot(plot);

    // Get the vectorized events for the lex contract
    let lex_price_events = data.get_vectorized_events::<LogAssetDataFilter>("atomic_arbitrage");
    println!("len: {}", lex_price_events.len());
    let (x_data, y_data) = lex_price_events
        .into_iter()
        .enumerate()
        .map(|(idx, ev)| (idx as f64, u256_to_f64(ev.lex_price)))
        .unzip();
    let settings = PlotSettings::new()
        .labels("Index", "LEX Price")
        .title("Price Path");
    let plot = LinePlot::new(x_data, y_data).settings(settings);
    figure.add_plot(plot);

    let lex_price_events = data.get_vectorized_events::<LogAssetDataFilter>("atomic_arbitrage");
    println!("len: {}", lex_price_events.len());
    let (x_data, y_data) = lex_price_events
        .into_iter()
        .enumerate()
        .map(|(idx, ev)| (idx as f64, u256_to_f64(ev.timestamp)))
        .unzip();
    let settings = PlotSettings::new()
        .labels("Index", "Timestamp")
        .title("Price Path");
    let plot = LinePlot::new(x_data, y_data).settings(settings);
    figure.add_plot(plot);

    let atomic_arb_profit_events =
        data.get_vectorized_events::<LogArbDataFilter>("atomic_arbitrage");
    let (x_data, y_data): (Vec<_>, Vec<_>) = atomic_arb_profit_events
        .into_iter()
        .enumerate()
        .map(|(idx, ev)| (idx as f64, u256_to_f64(ev.y_balance)))
        .scan(0.0, |acc, (idx, profit)| {
            *acc += profit;
            Some((idx, *acc))
        })
        .unzip();
    let settings = PlotSettings::new()
        .labels("Index", "Atomic Arb Profit")
        .title("Arbitrage Profit");
    let plot = LinePlot::new(x_data, y_data).settings(settings);
    figure.add_plot(plot);

    figure.create().unwrap();
}
