//! In the Model-view-controller architecture the view is
//! ["Any representation of information such as a chart, diagram or table."](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller).
//!
//! The portfolio view takes in the portfolio's data model and builds components
//! for the [controller](./mod.rs) to render.

use chrono::{DateTime, Utc};

use super::{
    model::portfolio::{AlloyAddress, AlloyU256, RawDataModel},
    *,
};
use crate::components::{
    double_labeled_data,
    system::{label, ExcaliburChart, ExcaliburText},
};

pub trait ValueToLabel<V> {
    fn to_label(&self) -> ExcaliburText;
}

impl ValueToLabel<Option<AlloyU256>> for Option<AlloyU256> {
    fn to_label(&self) -> ExcaliburText {
        if let Some(value) = self {
            let value = alloy_primitives::utils::format_ether(*value);
            match value.parse::<f64>() {
                Ok(_) => label(&value).quantitative().title1(),
                Err(_) => label(&"Failed to parse U256 as float.")
                    .caption()
                    .tertiary(),
            }
        } else {
            label(&"N/A").title1().secondary()
        }
    }
}

impl ValueToLabel<Result<AlloyU256, anyhow::Error>> for Result<AlloyU256, anyhow::Error> {
    fn to_label(&self) -> ExcaliburText {
        if let Ok(value) = self {
            let value = alloy_primitives::utils::format_ether(*value);
            match value.parse::<f64>() {
                Ok(_) => label(&value).quantitative().title1(),
                Err(_) => label(&"Failed to parse U256 as float.")
                    .caption()
                    .tertiary(),
            }
        } else {
            label(&"N/A").title1().secondary()
        }
    }
}

/// Responsible for transforming the model data into components for the view to
/// handle, e.g. converting an `AlloyU256` into a `ExcaliburText` that can be
/// shown to the user.
#[derive(Debug, Clone)]
pub struct PortfolioPresenter {
    model: RawDataModel<AlloyAddress, AlloyU256>,
    /// Value series is a live chart of portfolio value wrt. time.
    /// Therefore, it is continuously updated.
    portfolio_value_series: ExcaliburChart,
    /// Strategy plot can be computationally expensive. It is also static most
    /// of the time, so it is cached and only force updated on user request.
    portfolio_strategy_plot: ExcaliburChart,
}

impl Default for PortfolioPresenter {
    /// Makes `new` ExcaliburCharts so that the series and points are not set.
    /// The default ExcaliburChart sets a line series and single point,
    /// which affects the starting ranges of the chart.
    fn default() -> Self {
        Self {
            model: RawDataModel::new(),
            portfolio_value_series: ExcaliburChart::new(),
            portfolio_strategy_plot: ExcaliburChart::new(),
        }
    }
}

impl PortfolioPresenter {
    pub fn new(
        model: RawDataModel<AlloyAddress, AlloyU256>,
        portfolio_value_series: ExcaliburChart,
        portfolio_strategy_plot: ExcaliburChart,
    ) -> Self {
        Self {
            model,
            portfolio_value_series,
            portfolio_strategy_plot,
        }
    }

    pub fn update(&mut self, model: RawDataModel<AlloyAddress, AlloyU256>) {
        self.model = model;
        self.sync_portfolio_value_series();
        self.sync_portfolio_strategy_points();

        // Update static if needed.
        if self.static_needs_update() {
            self.sync_portfolio_strategy_curve();
        }
    }

    pub fn static_needs_update(&self) -> bool {
        self.portfolio_strategy_plot.chart.series.is_empty()
    }

    /// Does not update the model, only updates the chart data to match the
    /// existing model.
    /// todo: these can be computationally costly, we should only do this when
    /// the model changes or the user forces it.
    pub fn sync_portfolio_strategy_curve(&mut self) {
        // Get the series data.
        let data = self.model.derive_portfolio_strategy_plot();
        let data = match data {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };

        self.portfolio_strategy_plot.override_series(data.1);
        self.portfolio_strategy_plot.update_x_range(data.0.x_range);
        self.portfolio_strategy_plot.update_y_range(data.0.y_range);
        // Only happens once.
        self.portfolio_strategy_plot.override_ranges_flag(true);
    }

    pub fn sync_portfolio_strategy_points(&mut self) {
        // Get the pois
        let data = self.model.derive_portfolio_strategy_points();
        let data = match data {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };

        self.portfolio_strategy_plot.override_points(data);
    }

    /// Does not update the model, only updates the chart data to match the
    /// existing model.
    /// todo: these can be computationally costly, we should only do this
    /// when the model changes or the user forces it.
    pub fn sync_portfolio_value_series(&mut self) {
        // Get the series data.
        let data = self.model.derive_portfolio_value_series();
        let data = match data {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to get portfolio value series: {:}", e);
                return;
            }
        };

        self.portfolio_value_series.override_series(vec![data.1]);
        self.portfolio_value_series.update_x_range(data.0.x_range);
        self.portfolio_value_series.update_y_range(data.0.y_range);
        // Only happens once.
        self.portfolio_value_series.override_ranges_flag(true);
    }

    pub fn get_block_number(&self) -> Option<u64> {
        self.model.raw_last_chain_data_sync_block
    }

    pub fn get_block_timestamp(&self) -> Option<DateTime<Utc>> {
        self.model.raw_last_chain_data_sync_timestamp
    }

    pub fn get_internal_price(&self) -> ExcaliburText {
        self.model.raw_internal_spot_price.to_label()
    }

    pub fn get_external_price(&self) -> ExcaliburText {
        self.model.raw_external_spot_price.to_label()
    }

    pub fn get_internal_portfolio_value(&self) -> ExcaliburText {
        self.model.derive_internal_portfolio_value().to_label()
    }

    pub fn get_portfolio_health(&self) -> ExcaliburText {
        let data = self.model.derive_portfolio_health();
        match data {
            Ok(data) => {
                let value = alloy_primitives::utils::format_ether(data);
                match value.parse::<f64>() {
                    Ok(_) => label(&format!("{}", value)).title1().percentage(),
                    Err(_) => label(&"Failed to parse U256 as float.")
                        .caption()
                        .tertiary(),
                }
            }
            Err(_) => label(&"N/A").title1().secondary(),
        }
    }

    pub fn get_external_portfolio_value(&self) -> ExcaliburText {
        self.model.derive_external_portfolio_value().to_label()
    }

    pub fn get_last_sync_timestamp(&self) -> ExcaliburText {
        let data = self.model.raw_last_chain_data_sync_timestamp;
        match data {
            Some(data) => label(&format!("Timestamp: {:}", data)).caption().tertiary(),
            None => label(&"Timestamp: N/A").caption().tertiary(),
        }
    }

    pub fn get_last_sync_block(&self) -> ExcaliburText {
        let data = self.model.raw_last_chain_data_sync_block;
        match data {
            Some(data) => label(&format!("Block: {:}", data)).caption().tertiary(),
            None => label(&"Block: N/A").caption().tertiary(),
        }
    }
}

/// View all of the underlying data from the model for the portfolio dashboard.
///
/// Important! Make sure the chart starts completely blank (not default), this
/// will make sure the initial ranges are set correctly.
#[derive(Debug, Clone, Default)]
pub struct DataView {
    pub presenter: PortfolioPresenter,
}

impl DataView {
    pub fn new(presenter: PortfolioPresenter) -> Self {
        Self { presenter }
    }

    /// Standard view for a single data element with a title and caption.
    pub fn data_title_caption<'a, Message>(
        &self,
        data: ExcaliburText,
        title: String,
        caption: String,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        double_labeled_data(
            data.build(),
            label(&title).highlight().build(),
            label(&caption).secondary().caption().build(),
        )
        .into()
    }

    pub fn internal_price<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.presenter.get_internal_price();
        let caption = format!(
            "ETH/USD @ {}",
            self.presenter.get_block_number().unwrap_or(0)
        );

        // todo: add a tooltip to the caption
        // todo: get the proper caption too!
        self.data_title_caption(data, "Price".to_string(), caption)
    }

    pub fn external_price<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.presenter.get_external_price();
        let caption = format!(
            "ETH/USD @ {}",
            self.presenter.get_block_number().unwrap_or(0)
        );
        self.data_title_caption(data, "External Price".to_string(), caption)
    }

    pub fn internal_portfolio_value<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.presenter.get_internal_portfolio_value();
        self.data_title_caption(data, "Portfolio Value".to_string(), "USD".to_string())
    }

    pub fn portfolio_health<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.presenter.get_portfolio_health();
        self.data_title_caption(
            data,
            "Portfolio Health".to_string(),
            "PFV / tPFV".to_string(),
        )
    }

    /// todo: this is a hack because right now all the reserves == all the
    /// deposits.
    pub fn tvl<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.presenter.get_external_portfolio_value();
        self.data_title_caption(data, "TVL".to_string(), "USD".to_string())
    }

    /// Plots the portfolio value series per block.
    pub fn portfolio_value_series(&self) -> Element<'_, chart::Message> {
        self.presenter.portfolio_value_series.build()
    }

    /// Plots the portfolio strategy.
    pub fn portfolio_strategy_plot(&self) -> Element<'_, chart::Message> {
        self.presenter.portfolio_strategy_plot.build()
    }

    /// Renders the last sync time as a caption.
    pub fn last_sync_timestamp<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        self.presenter.get_last_sync_timestamp().build().into()
    }

    /// Renders the last sync block as a caption.
    pub fn last_sync_block<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        self.presenter.get_last_sync_block().build().into()
    }
}
