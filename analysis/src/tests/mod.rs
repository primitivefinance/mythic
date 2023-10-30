// use crate::visualize::statistical::plot_statistical;
use arbiter_core::bindings::liquid_exchange;

use super::*;
use crate::{
    reader::SimulationData,
    visualize::{plots::statistical::StatisticalPlot, Figure},
};

#[test]
fn read_in_and_plot_statistical() {
    let file1 = "src/tests/test_output/trajectory=0.json";
    let data1 = SimulationData::new(file1).unwrap();
    let values1 = data1.get_vectorized_events::<liquid_exchange::PriceChangeFilter>("lex");

    let file2 = "src/tests/test_output/trajectory=1.json";
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
    let statistical_plot = StatisticalPlot { x_data, y_data };

    let mut figure = Figure::new("test", None);
    figure.add_statistical_plot(statistical_plot);
    figure.create().unwrap();
    assert!(std::path::Path::new("test.png").exists());
    std::fs::remove_file("test.png").unwrap();
}
