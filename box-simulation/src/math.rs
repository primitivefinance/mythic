//! Math functions for portfolio optimization and management.

/// Compute the returns of a series of values.
/// Which is defined as the ratio of the current value to the previous value
/// minus 1.
pub fn compute_returns(values: Vec<f64>) -> Vec<f64> {
    let mut returns = Vec::new();
    for i in 1..values.len() {
        returns.push(values[i] / values[i - 1] - 1.0);
    }
    returns
}

pub trait ComputeReturns {
    fn compute_log_returns(self) -> Vec<f64>;
    fn compute_simple_returns(self) -> Vec<f64>;
    fn compute_variance(self) -> f64;
    fn compute_std_deviation(self) -> f64;
    fn compute_realized_volatility(self) -> f64;
    fn compute_sharpe_ratio(self) -> f64;
}

impl<I> ComputeReturns for I
where
    I: IntoIterator<Item = f64>,
{
    fn compute_log_returns(self) -> Vec<f64> {
        let mut previous_value = 0.0_f64;
        let mut returns = Vec::new();
        for value in self {
            if previous_value != 0.0 {
                returns.push((value / previous_value).ln());
            }
            previous_value = value;
        }
        returns
    }
    fn compute_simple_returns(self) -> Vec<f64> {
        let mut previous_value = 0.0_f64;
        let mut returns = Vec::new();
        for value in self {
            if previous_value != 0.0 {
                returns.push(value / previous_value - 1.0);
            }
            previous_value = value;
        }
        returns
    }
    fn compute_variance(self) -> f64 {
        let values = self.into_iter().collect::<Vec<f64>>();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values
            .iter()
            .map(|&return_| (return_ - mean).powi(2))
            .sum::<f64>()
            / values.len() as f64;
        variance
    }
    fn compute_std_deviation(self) -> f64 {
        let variance = self.compute_variance();
        variance.sqrt()
    }
    fn compute_realized_volatility(self) -> f64 {
        let returns = self.compute_log_returns();
        let len = returns.len() + 1;
        let rv = returns.compute_std_deviation() / (len as f64 / 365.0);
        rv
    }
    // TODO: don't use log returns here, use simple returns
    fn compute_sharpe_ratio(self) -> f64 {
        let returns = self.compute_simple_returns();
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let std_deviation = returns.compute_std_deviation();
        mean / std_deviation
    }
}

/// Finds the volatility delta to match the target volatility, with respect to
/// the current volatility difference between the assets.
/// - current_volatility: Current volatility of portfolio of assets.
/// - target_volatility: Target volatility of portfolio of assets.
/// - current_delta: Current volatility difference between the assets.
/// - returns: Volatility delta to match the target volatility, in percentage
///   units.
pub fn compute_target_volatility_delta(
    current_volatility_pct: f64,
    target_volatility_pct: f64,
    current_delta_pct: f64,
) -> f64 {
    (target_volatility_pct - current_volatility_pct) / current_delta_pct
}

// todo: make sure these tests are correct, used copilot for them.
mod tests {
    #[test]
    fn test_compute_log_returns() {
        use super::ComputeReturns;
        let values = vec![1.0, 2.0, 3.0];
        let returns = values.compute_log_returns();
        assert_eq!(returns, [0.6931471805599453, 0.4054651081081644]);
    }

    #[test]
    fn test_compute_simple_returns() {
        use super::ComputeReturns;
        let values = vec![1.0, 2.0, 3.0];
        let returns = values.compute_simple_returns();
        assert_eq!(returns, [1.0, 0.5]);
    }

    #[test]
    fn test_compute_variance() {
        use super::ComputeReturns;
        let values = vec![1.0, 2.0, 3.0];
        let variance = values.compute_variance();
        assert_eq!(variance, 0.6666666666666666);
    }

    #[test]
    fn test_compute_std_deviation() {
        use super::ComputeReturns;
        let values = vec![1.0, 2.0, 3.0];
        let std_deviation = values.compute_std_deviation();
        assert_eq!(std_deviation, 0.816496580927726);
    }

    #[test]
    fn test_std_dev_log_returns() {
        use super::ComputeReturns;
        let values = vec![1.0, 2.0, 3.0];
        let vol = values.compute_log_returns().compute_std_deviation() / (3.0 / 365.0);
        assert_eq!(vol, 17.50065940748334);
    }

    #[test]
    fn test_realized_volatility() {
        use super::ComputeReturns;
        let values = vec![1.0, 2.0, 3.0];
        let vol = values.compute_realized_volatility();
        assert_eq!(vol, 17.50065940748334);
    }

    #[test]
    fn test_compute_sharpe_ratio() {
        use super::ComputeReturns;
        let values = vec![1.0, 2.0, 3.0];
        let sharpe_ratio = values.compute_sharpe_ratio();
        assert_eq!(sharpe_ratio, 3.0);
    }
}
