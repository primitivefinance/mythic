//! Simulates portfolio adjustments.

use ethers::{prelude::*, utils::format_ether};
use simulation::{
    agents::{
        block_admin::BlockAdminParameters,
        g3m::{
            g3m_portfolio_manager::{
                dollar_cost_averaging::DollarCostAveragingParameters,
                G3mPortfolioManagerParameters, G3mPortfolioManagerSpecialty,
            },
            liquidity_provider::LiquidityProviderParameters,
            swapper::SwapperParameters,
        },
        price_changer::{GBMParameters, PriceChangerParameters, PriceProcess},
        token_admin::{TokenAdminParameters, TokenData},
        AgentParameters,
    },
    settings::{
        parameters::{LinspaceParameters, Multiple, Single},
        SimulationConfig,
    },
};

use super::*;

pub fn wad_to_float(wad: U256) -> anyhow::Result<f64, anyhow::Error> {
    format_ether(wad)
        .parse::<f64>()
        .map_err(|e| anyhow::anyhow!(e))
}

/// Helps build a config for simulating a portfolio adjustment.
#[derive(Default, Clone, Debug)]
pub struct ConfigBuilder {
    block_admin: Option<BlockAdminParameters>,
    token_admin: Option<TokenAdminParameters>,
    lp: Option<LiquidityProviderParameters<Multiple>>,
    pub price_changer: EmptyPriceChanger,
    pub portfolio_manager: EmptyPortfolioManager,
    pub swapper: EmptySwapper,
}

/// For this simulation config, we just need to make sure to add the proper
/// agent settings. The rest can be default.
impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// token admin
    pub fn coins(&mut self, coin_x: TokenData, coin_y: TokenData) -> &mut Self {
        self.token_admin = Some(TokenAdminParameters {
            arbx: coin_x,
            arby: coin_y,
        });
        self
    }

    /// block admin
    pub fn timestep_size(&mut self, timestep_size: u64) -> &mut Self {
        self.block_admin = Some(BlockAdminParameters { timestep_size });
        self
    }

    /// lp
    pub fn deposit_x(&mut self, deposit_x: U256) -> &mut Self {
        let formatted = wad_to_float(deposit_x).unwrap();
        if let Some(lp) = &mut self.lp {
            lp.x_liquidity = Multiple(LinspaceParameters {
                fixed: Some(formatted),
                ..Default::default()
            });
        } else {
            self.lp = Some(LiquidityProviderParameters {
                x_liquidity: Multiple(LinspaceParameters {
                    fixed: Some(formatted),
                    ..Default::default()
                }),
                initial_price: Multiple(LinspaceParameters {
                    ..Default::default()
                }),
            });
        }
        self
    }

    /// lp
    pub fn initial_price(&mut self, price_x: U256) -> &mut Self {
        let formatted = wad_to_float(price_x).unwrap();
        if let Some(lp) = &mut self.lp {
            lp.initial_price = Multiple(LinspaceParameters {
                fixed: Some(formatted),
                ..Default::default()
            });
        } else {
            self.lp = Some(LiquidityProviderParameters {
                x_liquidity: Multiple(LinspaceParameters {
                    ..Default::default()
                }),
                initial_price: Multiple(LinspaceParameters {
                    fixed: Some(formatted),
                    ..Default::default()
                }),
            });
        }
        self
    }

    pub fn build(self) -> SimulationConfig<Multiple> {
        let mut config = SimulationConfig::default();

        if let Some(block_admin) = self.block_admin {
            config.agent_parameters.insert(
                "block_admin".to_string(),
                AgentParameters::BlockAdmin(block_admin),
            );
        }

        if let Some(token_admin) = self.token_admin {
            config.agent_parameters.insert(
                "token_admin".to_string(),
                AgentParameters::TokenAdmin(token_admin),
            );
        }

        if let Some(lp) = self.lp {
            config
                .agent_parameters
                .insert("lp".to_string(), AgentParameters::LiquidityProvider(lp));
        }

        config.agent_parameters.insert(
            "price_changer".to_string(),
            AgentParameters::PriceChanger(self.price_changer.build()),
        );

        config.agent_parameters.insert(
            "portfolio_manager".to_string(),
            AgentParameters::G3mPortfolioManager(self.portfolio_manager.build()),
        );

        config.agent_parameters.insert(
            "swapper".to_string(),
            AgentParameters::Swapper(self.swapper.build()),
        );

        config
    }
}

#[derive(Clone, Debug)]
pub struct EmptySwapper {
    pub inner: SwapperParameters<Multiple>,
}

impl Default for EmptySwapper {
    fn default() -> Self {
        let inner: SwapperParameters<Multiple> = SwapperParameters {
            num_swaps: Multiple(LinspaceParameters {
                start: None,
                end: None,
                steps: None,
                fixed: Some(0.0),
            }),
            initial_balance: 0.0,
            swap_direction: false,
            end_timestamp: 0.0,
            start_timestamp: 0.0,
        };

        Self { inner }
    }
}

impl EmptySwapper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_swaps(&mut self, num_swaps: u64) -> &mut Self {
        self.inner.num_swaps = Multiple(LinspaceParameters {
            fixed: Some(num_swaps as f64),
            ..Default::default()
        });
        self
    }

    pub fn initial_balance(&mut self, initial_balance: U256) -> &mut Self {
        let formatted = wad_to_float(initial_balance).unwrap();
        self.inner.initial_balance = formatted;
        self
    }

    pub fn swap_direction(&mut self, swap_direction: bool) -> &mut Self {
        self.inner.swap_direction = swap_direction;
        self
    }

    pub fn end_timestamp(&mut self, end_timestamp: u64) -> &mut Self {
        self.inner.end_timestamp = end_timestamp as f64;
        self
    }

    pub fn start_timestamp(&mut self, start_timestamp: u64) -> &mut Self {
        self.inner.start_timestamp = start_timestamp as f64;
        self
    }

    pub fn build(self) -> SwapperParameters<Multiple> {
        self.inner
    }
}

#[derive(Clone, Debug)]
pub struct EmptyPortfolioManager {
    pub inner: G3mPortfolioManagerParameters<Multiple>,
}

impl Default for EmptyPortfolioManager {
    fn default() -> Self {
        let inner: G3mPortfolioManagerParameters<Multiple> = G3mPortfolioManagerParameters {
            initial_weight_x: Multiple(LinspaceParameters {
                start: None,
                end: None,
                steps: None,
                fixed: Some(0.0),
            }),
            fee: Multiple(LinspaceParameters {
                start: None,
                end: None,
                steps: None,
                fixed: Some(0.0),
            }),
            specialty: G3mPortfolioManagerSpecialty::DollarCostAveraging(
                DollarCostAveragingParameters {
                    end_weight: 0.0,
                    end_timestamp: 0,
                },
            ),
        };

        Self { inner }
    }
}

impl EmptyPortfolioManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Modifies the start weight of the portfolio.
    pub fn start_weight_x(&mut self, start_weight_x: U256) -> &mut Self {
        let formatted = wad_to_float(start_weight_x).unwrap();
        self.inner.initial_weight_x = Multiple(LinspaceParameters {
            fixed: Some(formatted),
            ..Default::default()
        });
        self
    }

    pub fn end_weight_x(&mut self, end_weight_x: U256) -> &mut Self {
        let formatted = wad_to_float(end_weight_x).unwrap();
        match &mut self.inner.specialty {
            G3mPortfolioManagerSpecialty::DollarCostAveraging(parameters) => {
                parameters.end_weight = formatted;
            }
            _ => {
                // only dca is supported.
            }
        }
        self
    }

    pub fn end_timestamp(&mut self, end_timestamp: u64) -> &mut Self {
        match &mut self.inner.specialty {
            G3mPortfolioManagerSpecialty::DollarCostAveraging(parameters) => {
                parameters.end_timestamp = end_timestamp;
            }
            _ => {
                // only dca is supported.
            }
        }
        self
    }

    /// Right now this assumes fee is in f32, in basis points.
    /// Assuming we start with a fee in wad, we need to multiply it by 10000 for
    /// now.
    pub fn fee(&mut self, wad: U256) -> &mut Self {
        let bps = U256::from(10000);
        let fee_bps_wad = wad.checked_mul(bps).unwrap();
        let fee_bps_float = wad_to_float(fee_bps_wad).unwrap();
        self.inner.fee = Multiple(LinspaceParameters {
            fixed: Some(fee_bps_float),
            ..Default::default()
        });
        self
    }

    pub fn build(self) -> G3mPortfolioManagerParameters<Multiple> {
        self.inner
    }
}

#[derive(Clone, Debug)]
pub struct EmptyPriceChanger {
    pub inner: PriceChangerParameters<Multiple>,
}

impl Default for EmptyPriceChanger {
    fn default() -> Self {
        let inner: PriceChangerParameters<Multiple> = PriceChangerParameters {
            initial_price: Multiple(LinspaceParameters {
                start: None,
                end: None,
                steps: None,
                fixed: Some(0.0),
            }),
            t_0: Multiple(LinspaceParameters {
                start: None,
                end: None,
                steps: None,
                fixed: Some(0.0),
            }),
            t_n: Multiple(LinspaceParameters {
                start: None,
                end: None,
                steps: None,
                fixed: Some(0.0),
            }),
            num_steps: 0,
            num_paths: 0,
            seed: Some(0),
            process: PriceProcess::Gbm(GBMParameters {
                drift: Multiple(LinspaceParameters {
                    start: None,
                    end: None,
                    steps: None,
                    fixed: Some(0.0),
                }),
                volatility: Multiple(LinspaceParameters {
                    start: None,
                    end: None,
                    steps: None,
                    fixed: Some(0.0),
                }),
            }),
        };

        Self { inner }
    }
}

impl EmptyPriceChanger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initial_price(&mut self, initial_price: U256) -> &mut Self {
        let formatted = wad_to_float(initial_price).unwrap();
        self.inner.initial_price = Multiple(LinspaceParameters {
            fixed: Some(formatted),
            ..Default::default()
        });
        self
    }

    pub fn t_0(&mut self, t_0: f64) -> &mut Self {
        self.inner.t_0 = Multiple(LinspaceParameters {
            fixed: Some(t_0),
            ..Default::default()
        });
        self
    }

    pub fn t_n(&mut self, t_n: f64) -> &mut Self {
        self.inner.t_n = Multiple(LinspaceParameters {
            fixed: Some(t_n),
            ..Default::default()
        });
        self
    }

    pub fn num_steps(&mut self, num_steps: usize) -> &mut Self {
        self.inner.num_steps = num_steps;
        self
    }

    pub fn num_paths(&mut self, num_paths: usize) -> &mut Self {
        self.inner.num_paths = num_paths;
        self
    }

    pub fn seed(&mut self, seed: u64) -> &mut Self {
        self.inner.seed = Some(seed);
        self
    }

    pub fn drift(&mut self, drift: f64) -> &mut Self {
        if let PriceProcess::Gbm(ref mut gbm) = self.inner.process {
            gbm.drift = Multiple(LinspaceParameters {
                fixed: Some(drift),
                ..Default::default()
            });
        } else {
            self.inner.process = PriceProcess::Gbm(GBMParameters {
                drift: Multiple(LinspaceParameters {
                    fixed: Some(drift),
                    ..Default::default()
                }),
                volatility: Multiple(LinspaceParameters {
                    ..Default::default()
                }),
            });
        }
        self
    }

    pub fn volatility(&mut self, volatility: f64) -> &mut Self {
        if let PriceProcess::Gbm(ref mut gbm) = self.inner.process {
            gbm.volatility = Multiple(LinspaceParameters {
                fixed: Some(volatility),
                ..Default::default()
            });
        } else {
            self.inner.process = PriceProcess::Gbm(GBMParameters {
                drift: Multiple(LinspaceParameters {
                    ..Default::default()
                }),
                volatility: Multiple(LinspaceParameters {
                    fixed: Some(volatility),
                    ..Default::default()
                }),
            });
        }
        self
    }

    pub fn build(self) -> PriceChangerParameters<Multiple> {
        self.inner
    }
}
