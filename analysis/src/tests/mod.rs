use super::*;
use crate::reader::SimulationData;
// use crate::visualize::statistical::plot_statistical;
use arbiter_core::bindings::liquid_exchange;

// #[test]
// fn read_in_and_plot_statistical() {
//     let file1 = "src/tests/test_output/trajectory=0.json";
//     let data1 = SimulationData::new(file1).unwrap();
//     let values1 = data1.get_vectorized_events::<liquid_exchange::PriceChangeFilter>("lex");

//     let file2 = "src/tests/test_output/trajectory=1.json";
//     let data2 = SimulationData::new(file2).unwrap();
//     let values2 = data2.get_vectorized_events::<liquid_exchange::PriceChangeFilter>("lex");

//     let (x_data, y_data1): (Vec<f64>, Vec<f64>) = values1
//         .iter()
//         .enumerate()
//         .map(|(index, event)| (index as f64, wad_to_float(event.price)))
//         .unzip();

//     let y_data2 = values2
//         .iter()
//         .map(|event| wad_to_float(event.price))
//         .collect();

//     let y_data = vec![y_data1, y_data2];
//     plot_statistical(&x_data, &y_data).unwrap();
// }
