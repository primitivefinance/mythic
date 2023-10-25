//! Math functions for portfolio optimization and management.
/// Compute the returns of a series of values.
/// Which is defined as the ratio of the current value to the previous value
/// minus 1.
pub fn compute_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let values = values.into_iter().collect::<Vec<f64>>();
    let mut returns = Vec::new();
    for i in 1..values.len() {
        returns.push(values[i] / values[i - 1] - 1.0);
    }
    returns
}

pub fn compute_log_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut previous_value = 0.0_f64;
    let mut returns = Vec::new();
    for value in values {
        if previous_value != 0.0 {
            returns.push((value / previous_value).ln());
        }
        previous_value = value;
    }
    returns
}

pub fn compute_simple_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut previous_value = 0.0_f64;
    let mut returns = Vec::new();
    for value in values {
        if previous_value != 0.0 {
            returns.push(value / previous_value - 1.0);
        }
        previous_value = value;
    }
    returns
}

pub fn compute_variance(values: impl IntoIterator<Item = f64>) -> f64 {
    let values = values.into_iter().collect::<Vec<f64>>();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|&return_| (return_ - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    variance
}

pub fn compute_std_deviation(values: impl IntoIterator<Item = f64>) -> f64 {
    let variance = compute_variance(values);
    variance.sqrt()
}

pub fn compute_realized_volatility(values: impl IntoIterator<Item = f64>) -> f64 {
    let returns = compute_log_returns(values);
    let len = returns.len() + 1;
    compute_std_deviation(returns) / (len as f64 / 365.0)
}

// TODO: don't use log returns here, use simple returns
pub fn compute_sharpe_ratio(values: impl IntoIterator<Item = f64>) -> f64 {
    let returns = compute_simple_returns(values);
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
    use super::*;
    const VALUES: [f64; 3] = [1.0, 2.0, 3.0];

    #[test]
    fn test_compute_log_returns() {
        let returns = compute_log_returns(VALUES);
        assert_eq!(returns, [0.6931471805599453, 0.4054651081081644]);
    }

    #[test]
    fn test_compute_simple_returns() {
        let returns = compute_simple_returns(VALUES);
        assert_eq!(returns, [1.0, 0.5]);
    }

    #[test]
    fn test_compute_variance() {
        let variance = compute_variance(VALUES);
        assert_eq!(variance, 0.6666666666666666);
    }

    #[test]
    fn test_compute_std_deviation() {
        let std_deviation = compute_std_deviation(VALUES);
        assert_eq!(std_deviation, 0.816496580927726);
    }

    #[test]
    fn test_std_dev_log_returns() {
        let log_returns = compute_log_returns(VALUES);
        let vol = compute_std_deviation(log_returns) / (3.0 / 365.0);
        assert_eq!(vol, 17.50065940748334);
    }

    #[test]
    fn test_realized_volatility() {
        let vol = compute_realized_volatility(VALUES);
        assert_eq!(vol, 17.50065940748334);
    }

    #[test]
    fn test_compute_sharpe_ratio() {
        let sharpe_ratio = compute_sharpe_ratio(VALUES);
        assert_eq!(sharpe_ratio, 3.0);
    }
}
