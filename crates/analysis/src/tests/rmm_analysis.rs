use super::*;

// #[test]
// pub fn read_single_simulation() {
//     let file = "../../analysis/rmm/vol_targeting/static/0.json";
//     let data = SimulationData::new(file).unwrap();
//     let pool_stats = data.get_vectorized_events::<dfmm::LogPoolStatsFilter>("dfmm");
//     let lex_prices = data.get_vectorized_events::<lex::PriceChangeFilter>("lex");
//     let dex_prices = data.get_vectorized_events::<atomic_v2::PriceFilter>("atomic_arbitrage");

//     let dex_prices: (Vec<f64>, Vec<f64>) = dex_prices
//         .iter()
//         .map(|event| (event.timestamp.as_u64().as_f64(), wad_to_float(event.price)))
//         .unzip();

//     let lex_prices: (Vec<f64>, Vec<f64>) = lex_prices
//         .iter()
//         .map(|event| (event.timestamp.as_u64().as_f64(), wad_to_float(event.price)))
//         .unzip();

//     let timestamps = pool_stats
//         .iter()
//         .map(|event| event.timestamp.as_u64().as_f64())
//         .collect::<Vec<f64>>();
//     let mut rx = (timestamps.clone(), vec![]);
//     let mut ry = (timestamps.clone(), vec![]);
//     let mut invariant = (timestamps.clone(), vec![]);
//     let mut strike = (timestamps.clone(), vec![]);
//     let mut sigma = (timestamps.clone(), vec![]);
//     let mut tau = (timestamps.clone(), vec![]);

//     rx.1.extend(
//         pool_stats
//             .iter()
//             .map(|event| wad_to_float(event.rx))
//             .collect::<Vec<f64>>(),
//     );
//     ry.1.extend(
//         pool_stats
//             .iter()
//             .map(|event| wad_to_float(event.ry))
//             .collect::<Vec<f64>>(),
//     );
//     invariant.1.extend(
//         pool_stats
//             .iter()
//             .map(|event| event.invariant.as_i128() as f64)
//             .collect::<Vec<f64>>(),
//     );
//     strike.1.extend(
//         pool_stats
//             .iter()
//             .map(|event| wad_to_float(event.strike))
//             .collect::<Vec<f64>>(),
//     );
//     sigma.1.extend(
//         pool_stats
//             .iter()
//             .map(|event| wad_to_float(event.sigma))
//             .collect::<Vec<f64>>(),
//     );
//     tau.1.extend(
//         pool_stats
//             .iter()
//             .map(|event| wad_to_float(event.tau))
//             .collect::<Vec<f64>>(),
//     );

//     let mut figure = Figure::new("backtest_1000_days", Some((2000, 2000)));

//     let plot_settings = PlotSettings::new()
//         .title("Lex Prices")
//         .labels("index", "price");
//     let lex_plot = LinePlot::new(lex_prices.0, lex_prices.1).settings(plot_settings);
//     figure.add_plot(lex_plot);

//     let plot_settings = PlotSettings::new()
//         .title("Dex Prices")
//         .labels("index", "price");
//     let dex_plot = LinePlot::new(dex_prices.0, dex_prices.1).settings(plot_settings);
//     figure.add_plot(dex_plot);

//     let plot_settings = PlotSettings::new()
//         .title("Reserve X")
//         .labels("timestamp", "rx");
//     let rx_plot = LinePlot::new(rx.0, rx.1).settings(plot_settings);
//     figure.add_plot(rx_plot);

//     let plot_settings = PlotSettings::new()
//         .title("Reserve Y")
//         .labels("timestamp", "ry");
//     let ry_plot = LinePlot::new(ry.0, ry.1).settings(plot_settings);
//     figure.add_plot(ry_plot);

//     let plot_settings = PlotSettings::new()
//         .title("Invariant")
//         .labels("timestamp", "invariant");
//     let invariant_plot = LinePlot::new(invariant.0, invariant.1).settings(plot_settings);
//     figure.add_plot(invariant_plot);

//     let plot_settings = PlotSettings::new()
//         .title("Strike")
//         .labels("timestamp", "strike");
//     let strike_plot = LinePlot::new(strike.0, strike.1).settings(plot_settings);
//     figure.add_plot(strike_plot);

//     let plot_settings = PlotSettings::new()
//         .title("Sigma")
//         .labels("timestamp", "sigma");
//     let sigma_plot = LinePlot::new(sigma.0, sigma.1).settings(plot_settings);
//     figure.add_plot(sigma_plot);

//     let plot_settings = PlotSettings::new().title("Tau").labels("timestamp", "tau");
//     let tau_plot = LinePlot::new(tau.0, tau.1).settings(plot_settings);
//     figure.add_plot(tau_plot);
//     figure.create().unwrap();
// }

// use rayon::prelude::*;

// #[test]
// fn read_multiple_simulation() {
//     (0..4).into_par_iter().for_each(|idx| {
//         let file = format!("../../analysis/rmm/vol_targeting/static/{idx}.json");
//         let data = SimulationData::new(file.as_str()).unwrap();
//         let pool_stats = data.get_vectorized_events::<dfmm::LogPoolStatsFilter>("dfmm");
//         let lex_prices = data.get_vectorized_events::<lex::PriceChangeFilter>("lex");
//         let dex_prices = data.get_vectorized_events::<atomic_v2::PriceFilter>("atomic_arbitrage");

//         let dex_prices: (Vec<f64>, Vec<f64>) = dex_prices
//             .iter()
//             .map(|event| (event.timestamp.as_u64().as_f64(), wad_to_float(event.price)))
//             .unzip();

//         let lex_prices: (Vec<f64>, Vec<f64>) = lex_prices
//             .iter()
//             .map(|event| (event.timestamp.as_u64().as_f64(), wad_to_float(event.price)))
//             .unzip();

//         let timestamps = pool_stats
//             .iter()
//             .map(|event| event.timestamp.as_u64().as_f64())
//             .collect::<Vec<f64>>();
//         let mut rx = (timestamps.clone(), vec![]);
//         let mut ry = (timestamps.clone(), vec![]);
//         let mut invariant = (timestamps.clone(), vec![]);
//         let mut strike = (timestamps.clone(), vec![]);
//         let mut sigma = (timestamps.clone(), vec![]);
//         let mut tau = (timestamps.clone(), vec![]);

//         rx.1.extend(
//             pool_stats
//                 .iter()
//                 .map(|event| wad_to_float(event.rx))
//                 .collect::<Vec<f64>>(),
//         );
//         ry.1.extend(
//             pool_stats
//                 .iter()
//                 .map(|event| wad_to_float(event.ry))
//                 .collect::<Vec<f64>>(),
//         );
//         invariant.1.extend(
//             pool_stats
//                 .iter()
//                 .map(|event| event.invariant.as_i128() as f64)
//                 .collect::<Vec<f64>>(),
//         );
//         strike.1.extend(
//             pool_stats
//                 .iter()
//                 .map(|event| wad_to_float(event.strike))
//                 .collect::<Vec<f64>>(),
//         );
//         sigma.1.extend(
//             pool_stats
//                 .iter()
//                 .map(|event| wad_to_float(event.sigma))
//                 .collect::<Vec<f64>>(),
//         );
//         tau.1.extend(
//             pool_stats
//                 .iter()
//                 .map(|event| wad_to_float(event.tau))
//                 .collect::<Vec<f64>>(),
//         );

//         let mut figure = Figure::new(format!("plot_figs_{idx}").as_str(), Some((2000, 2000)));

//         let plot_settings = PlotSettings::new()
//             .title("Lex Prices")
//             .labels("index", "price");
//         let lex_plot = LinePlot::new(lex_prices.0, lex_prices.1).settings(plot_settings);
//         figure.add_plot(lex_plot);

//         let plot_settings = PlotSettings::new()
//             .title("Dex Prices")
//             .labels("index", "price");
//         let dex_plot = LinePlot::new(dex_prices.0, dex_prices.1).settings(plot_settings);
//         figure.add_plot(dex_plot);

//         let plot_settings = PlotSettings::new()
//             .title("Reserve X")
//             .labels("timestamp", "rx");
//         let rx_plot = LinePlot::new(rx.0, rx.1).settings(plot_settings);
//         figure.add_plot(rx_plot);

//         let plot_settings = PlotSettings::new()
//             .title("Reserve Y")
//             .labels("timestamp", "ry");
//         let ry_plot = LinePlot::new(ry.0, ry.1).settings(plot_settings);
//         figure.add_plot(ry_plot);

//         let plot_settings = PlotSettings::new()
//             .title("Invariant")
//             .labels("timestamp", "invariant");
//         let invariant_plot = LinePlot::new(invariant.0, invariant.1).settings(plot_settings);
//         figure.add_plot(invariant_plot);

//         let plot_settings = PlotSettings::new()
//             .title("Strike")
//             .labels("timestamp", "strike");
//         let strike_plot = LinePlot::new(strike.0, strike.1).settings(plot_settings);
//         figure.add_plot(strike_plot);

//         let plot_settings = PlotSettings::new()
//             .title("Sigma")
//             .labels("timestamp", "sigma");
//         let sigma_plot = LinePlot::new(sigma.0, sigma.1).settings(plot_settings);
//         figure.add_plot(sigma_plot);

//         let plot_settings = PlotSettings::new().title("Tau").labels("timestamp", "tau");
//         let tau_plot = LinePlot::new(tau.0, tau.1).settings(plot_settings);
//         figure.add_plot(tau_plot);
//         figure.create().unwrap();
//     });
// }
