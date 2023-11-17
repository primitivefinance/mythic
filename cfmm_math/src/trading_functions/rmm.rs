use super::*;

#[tracing::instrument(ret, level = "trace")]
pub fn compute_sigma_sqrt_tau(sigma: f64, tau: f64) -> f64 {
    let sqrt_tau = tau.sqrt() * 10f64.powi(9);
    sigma * sqrt_tau
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_half_sigma_power_2_tau(sigma: f64, tau: f64) -> f64 {
    let inner_term = (sigma.powi(2) * tau) / 2.0;
    inner_term
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_ln_s_div_k(s: f64, k: f64) -> f64 {
    (s / k).ln()
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_d1(s: f64, k: f64, sigma: f64, tau: f64) -> f64 {
    let sigma_sqrt_tau = sigma * tau.sqrt();
    let ln_s_div_k = compute_ln_s_div_k(s, k);
    let half_sigma_pow_two_tau = compute_half_sigma_power_2_tau(sigma, tau);

    (ln_s_div_k + half_sigma_pow_two_tau) / sigma_sqrt_tau
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_d2(s: f64, k: f64, sigma: f64, tau: f64) -> f64 {
    let sigma_sqrt_tau = compute_sigma_sqrt_tau(sigma, tau);
    let ln_s_div_k = compute_ln_s_div_k(s, k);
    let half_sigma_pow_two_tau = compute_half_sigma_power_2_tau(sigma, tau);

    (ln_s_div_k - half_sigma_pow_two_tau) / sigma_sqrt_tau
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_l_given_x_rust(
    reserve_x_float: f64,
    spot_price_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let denominator = 1.0
        - normal.cdf(compute_d1(
            spot_price_float,
            strike_price_float,
            sigma_float,
            tau_float,
        ));
    reserve_x_float / denominator
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_l_given_y_rust(
    reserve_y_float: f64,
    spot_price_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let denominator = 1.0
        - normal.cdf(compute_d2(
            spot_price_float,
            strike_price_float,
            sigma_float,
            tau_float,
        ));
    reserve_y_float / denominator
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_x_given_l_rust(
    liquidity_float: f64,
    spot_price_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let cdf = normal.cdf(compute_d1(
        spot_price_float,
        strike_price_float,
        sigma_float,
        tau_float,
    ));
    liquidity_float * (1.0 - cdf)
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_y_given_l_rust(
    liquidity_float: f64,
    spot_price_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let cdf = normal.cdf(compute_d2(
        spot_price_float,
        strike_price_float,
        sigma_float,
        tau_float,
    ));
    strike_price_float * liquidity_float * cdf
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_spot_price_rust(
    reserve_x_float: f64,
    liquidity_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let sigma_sqrt_tau = compute_sigma_sqrt_tau(sigma_float, tau_float);
    let half_sigma_power_2_tau = compute_half_sigma_power_2_tau(sigma_float, tau_float);
    let r1 = reserve_x_float / liquidity_float;

    strike_price_float
        * ((normal.inverse_cdf(1.0 - r1) * sigma_sqrt_tau - half_sigma_power_2_tau).exp())
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_output_y_given_x_rust(
    reserve_x_float: f64,
    reserve_y_float: f64,
    delta_x_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let sigma_sqrt_tau = compute_sigma_sqrt_tau(sigma_float, tau_float);
    let kl = strike_price_float * (liquidity_float + delta_l_float);

    let cdf = normal.cdf(
        -sigma_sqrt_tau
            - normal
                .inverse_cdf((reserve_x_float + delta_x_float) / (liquidity_float + delta_l_float)),
    );

    kl * cdf - reserve_y_float
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_output_x_given_y_rust(
    reserve_x_float: f64,
    reserve_y_float: f64,
    delta_y_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let sigma_sqrt_tau = compute_sigma_sqrt_tau(sigma_float, tau_float);
    let kl = strike_price_float * (liquidity_float + delta_l_float);

    let cdf =
        normal.cdf(-sigma_sqrt_tau - normal.inverse_cdf((reserve_y_float + delta_y_float) / kl));

    kl * cdf - reserve_x_float
}
