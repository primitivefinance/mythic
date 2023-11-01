use super::*;

pub mod time_based;

pub trait Swapper {
    fn swap(&self, amount: f64) -> f64;
}

pub enum SwapperType {
    TimeBased(TimeBasedSwapper),
}
