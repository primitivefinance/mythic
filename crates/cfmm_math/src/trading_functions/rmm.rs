use reikna::integral::*;
use statrs::distribution::Continuous;

use super::*;

#[tracing::instrument(ret, level = "trace")]
pub fn compute_sigma_sqrt_tau(sigma: f64, tau: f64) -> f64 {
    let sqrt_tau = tau.sqrt();
    sigma * sqrt_tau
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_half_sigma_power_2_tau(sigma: f64, tau: f64) -> f64 {
    (sigma.powi(2) * tau) / 2.0
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

#[allow(clippy::too_many_arguments)]
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

#[allow(clippy::too_many_arguments)]
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

#[allow(clippy::too_many_arguments)]
#[tracing::instrument(ret, level = "trace")]
pub fn compute_y_given_x_rust(
    reserve_x_float: f64,
    liquidity_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
    tau_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let sigma_sqrt_tau = compute_sigma_sqrt_tau(sigma_float, tau_float);
    let kl = strike_price_float * (liquidity_float);

    let cdf =
        normal.cdf(-normal.inverse_cdf((reserve_x_float) / (liquidity_float)) - sigma_sqrt_tau);

    kl * cdf
}

/// K e^(-1/2 v^2 t) [v sqrt(t) e^(ln(x / K) + 1/2 v^2 t) / Gaussian.pdf{
/// ln(x/K) / v sqrt(t) + 1/2 v sqrt(t) }]
/// Computes a necessary component of the liquidity distribution, but I am not
/// 100% sure what it represents. Add a comment here to tell me!
#[allow(clippy::too_many_arguments)]
#[tracing::instrument(ret, level = "trace")]
pub fn g_x(
    reserve_x_wad_float: f64,
    strike_price_wad_float: f64,
    sigma_percent_wad_float: f64,
    time_to_expiry_years_wad_float: f64,
) -> f64 {
    // If x is within float epilson of 0, return 0.
    if reserve_x_wad_float < f64::EPSILON && reserve_x_wad_float > -f64::EPSILON {
        return 0.0;
    }

    let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
    // v sqrt(t)
    let v_sqrt_t = compute_sigma_sqrt_tau(sigma_percent_wad_float, time_to_expiry_years_wad_float);
    // ln(x / K)
    let ln_x_div_k = compute_ln_s_div_k(reserve_x_wad_float, strike_price_wad_float);
    // 1/2 v sqrt(t)
    let half_sigma_pow_two_tau =
        compute_half_sigma_power_2_tau(sigma_percent_wad_float, time_to_expiry_years_wad_float);
    // K e^(-1/2 v^2 t)
    let term_1 = strike_price_wad_float * (-half_sigma_pow_two_tau).exp();
    // v sqrt(t) e^(ln(x / K) + 1/2 v^2 t)
    let numerator = v_sqrt_t * (ln_x_div_k + half_sigma_pow_two_tau).exp();
    // Gaussian.pdf(ln(x/K) / v sqrt(t) + 1/2 v sqrt(t))
    let denominator = normal.pdf(ln_x_div_k / v_sqrt_t + half_sigma_pow_two_tau);

    term_1 * numerator / denominator
}

/// a = L / integral(0, 100)( 1 / g(x) gx )
/// Warning: This function is undefined for x = 0. This will throw a NaN, which
/// is why we pass the lower bound as f64::EPSILON.
///
/// Should do up to 1000, but thats slower than 100.
#[allow(clippy::too_many_arguments)]
#[tracing::instrument(ret, level = "trace")]
pub fn get_a(
    total_liquidity_wad_float: f64,
    strike_price_wad_float: f64,
    sigma_percent_wad_float: f64,
    time_to_expiry_years_wad_float: f64,
) -> f64 {
    let g_x = reikna::func!(move |x: f64| 1.0
        / g_x(
            x,
            strike_price_wad_float,
            sigma_percent_wad_float,
            time_to_expiry_years_wad_float
        ));
    let integral = reikna::integral::integrate(&g_x, f64::EPSILON, 100.0);
    total_liquidity_wad_float / integral
}

/// a / g(x)
/// Computes the liquidity distribution of the Log Normal (RMM) curve.
#[allow(clippy::too_many_arguments)]
#[tracing::instrument(ret, level = "trace")]
pub fn liq_distribution(
    reserve_x_wad_float: f64,
    total_liquidity_wad_float: f64,
    strike_price_wad_float: f64,
    sigma_percent_wad_float: f64,
    time_to_expiry_years_wad_float: f64,
) -> f64 {
    let a = get_a(
        total_liquidity_wad_float,
        strike_price_wad_float,
        sigma_percent_wad_float,
        time_to_expiry_years_wad_float,
    );
    a / g_x(
        reserve_x_wad_float,
        strike_price_wad_float,
        sigma_percent_wad_float,
        time_to_expiry_years_wad_float,
    )
}

/// P_x(x) = K e^(phi^-1(1 - x / L) sigma sqrt(t) - 1/2 sigma^2 t)
/// x(P_x) = L * (1 - phi( [ln(P_x / K) + 1/2 sigma^2 t] / sigma sqrt(t) ))
#[tracing::instrument(level = "trace")]
pub fn compute_x_given_price(
    spot_price_float: f64,
    total_liquidity: f64,
    strike_price_wad_float: f64,
    sigma_percent_wad_float: f64,
    time_to_expiry_years_wad_float: f64,
) -> f64 {
    let normal = statrs::distribution::Normal::new(0.0, 1.0).expect("Normal distribution failed");
    let sigma_sqrt_tau =
        compute_sigma_sqrt_tau(sigma_percent_wad_float, time_to_expiry_years_wad_float);
    let half_sigma_pow_two_tau =
        compute_half_sigma_power_2_tau(sigma_percent_wad_float, time_to_expiry_years_wad_float);

    let ln_x_div_k = compute_ln_s_div_k(spot_price_float, strike_price_wad_float);
    let cdf = normal.cdf((ln_x_div_k + half_sigma_pow_two_tau) / sigma_sqrt_tau);

    total_liquidity * (1.0 - cdf)
}

/// V = P(1 - n(d1)) + Kn(d2)
pub fn compute_value_function(
    spot_price_float: f64,
    strike_price_wad_float: f64,
    sigma_percent_wad_float: f64,
    time_to_expiry_years_wad_float: f64,
) -> f64 {
    let normal = statrs::distribution::Normal::new(0.0, 1.0).expect("Normal distribution failed");
    let d1 = compute_d1(
        spot_price_float,
        strike_price_wad_float,
        sigma_percent_wad_float,
        time_to_expiry_years_wad_float,
    );

    let d2 = compute_d2(
        spot_price_float,
        strike_price_wad_float,
        sigma_percent_wad_float,
        time_to_expiry_years_wad_float,
    );

    let n_d1 = normal.cdf(d1);
    let n_d2 = normal.cdf(d2);
    spot_price_float * (1.0 - n_d1) + strike_price_wad_float * n_d2
}

/// P_x(x) = K e^(phi^-1(1 - x / L) sigma sqrt(t) - 1/2 sigma^2 t)
pub fn compute_price_given_x_rust(
    reserve_x_float: f64,
    total_liquidity: f64,
    strike_price_wad_float: f64,
    sigma_percent_wad_float: f64,
    time_to_expiry_years_wad_float: f64,
) -> f64 {
    let normal = statrs::distribution::Normal::new(0.0, 1.0).expect("Normal distribution failed");
    let sigma_sqrt_tau =
        compute_sigma_sqrt_tau(sigma_percent_wad_float, time_to_expiry_years_wad_float);
    let half_sigma_pow_two_tau =
        compute_half_sigma_power_2_tau(sigma_percent_wad_float, time_to_expiry_years_wad_float);

    let power = normal.inverse_cdf(1.0 - reserve_x_float / total_liquidity) * sigma_sqrt_tau
        - half_sigma_pow_two_tau;

    strike_price_wad_float * power.exp()
}

#[cfg(test)]
mod test {
    use statrs::assert_almost_eq;

    use super::*;

    #[test]
    fn test_g_x() {
        let x = 0.5;
        let strike = 1.0;
        let sigma = 1.0;
        let time_to_expiry = 1.0;

        let depth = g_x(x, strike, sigma, time_to_expiry);
        println!("depth: {}", depth);
    }

    #[test]
    fn test_get_a() {
        let liquidity = 1.0;
        let strike = 1.0;
        let sigma = 1.0;
        let time_to_expiry = 1.0;

        let depth = get_a(liquidity, strike, sigma, time_to_expiry);
        println!("depth: {}", depth);

        assert_almost_eq!(depth, 1.0, 1e-2);
    }

    #[test]
    fn test_liquidity_distribution() {
        let x = 0.05;
        let liquidity = 1.0;
        let strike = 1.0;
        let sigma = 1.0;
        let time_to_expiry = 1.0;

        let depth = liq_distribution(x, liquidity, strike, sigma, time_to_expiry);
        println!("depth: {}", depth);

        assert_almost_eq!(depth, 0.354, 1e-3);
    }
}
