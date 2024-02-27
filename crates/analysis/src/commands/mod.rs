use ethers::utils::format_ether;

use super::*;

pub mod plot_prices;

pub fn u256_to_f64(value: ethers::types::U256) -> f64 {
    let str = format_ether(value);
    str.parse::<f64>().unwrap()
}
