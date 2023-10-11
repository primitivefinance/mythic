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

/// Computes the mean of a series of values (arithmetic average), finds the mean
/// of the summed differences of the values vs. the mean.
pub fn compute_variance(values: Vec<f64>) -> f64 {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|&return_| (return_ - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    variance
}

/// Computes the square root of the variance.
pub fn compute_std_deviation(values: Vec<f64>) -> f64 {
    let variance = compute_variance(values);
    variance.sqrt()
}

/// The sharpe ratio is the ratio of the mean of the returns to the standard
/// deviation of the returns.
pub fn compute_sharpe_ratio(values: Vec<f64>) -> f64 {
    let returns = compute_returns(values);
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let std_deviation = compute_std_deviation(returns);
    mean / std_deviation
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
    fn test_compute_returns() {
        let values = vec![1.0, 2.0, 3.0];
        let returns = super::compute_returns(values);
        assert_eq!(returns, vec![1.0, 0.5]);
    }

    #[test]
    fn test_compute_variance() {
        let values = vec![1.0, 2.0, 3.0];
        let variance = super::compute_variance(values);
        assert_eq!(variance, 0.6666666666666666);
    }

    #[test]
    fn test_compute_std_deviation() {
        let values = vec![1.0, 2.0, 3.0];
        let std_deviation = super::compute_std_deviation(values);
        assert_eq!(std_deviation, 0.816496580927726);
    }

    #[test]
    fn test_compute_sharpe_ratio() {
        let values = vec![1.0, 2.0, 3.0];
        let sharpe_ratio = super::compute_sharpe_ratio(values);
        assert_eq!(sharpe_ratio, 3.0);
    }
}
