//! In the Model-view-controller architecture the view is
//! ["Any representation of information such as a chart, diagram or table."](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller).
//!
//! The portfolio view takes in the portfolio's data model and builds components
//! for the [controller](./mod.rs) to render.

use super::{
    portfolio_model::{AlloyAddress, AlloyU256, RawDataModel},
    *,
};
use crate::components::{
    double_labeled_data,
    system::{label, ExcaliburText},
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
            label(&"N/A").caption().tertiary()
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
            label(&"N/A").caption().tertiary()
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataView {
    pub model: RawDataModel<AlloyAddress, AlloyU256>,
    pub portfolio_value_series: ExcaliburChart,
    pub portfolio_strategy_plot: ExcaliburChart,
}

impl DataView {
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

    /// Does not update the model, only updates the chart data to match the
    /// existing model.
    pub fn sync_portfolio_strategy_curve(&mut self) {
        // Get the series data.
        let data = self.model.portfolio_strategy_plot();
        let data = match data {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };

        self.portfolio_strategy_plot.update_many_series(data.1);
        self.portfolio_strategy_plot
            .update_points_of_interest(data.2);
        self.portfolio_strategy_plot.update_x_range(data.0.x_range);
        self.portfolio_strategy_plot.update_y_range(data.0.y_range);
    }

    /// Does not update the model, only updates the chart data to match the
    /// existing model.
    pub fn sync_portfolio_value_series(&mut self) {
        // Get the series data.
        let data = self.model.portfolio_value_series();
        let data = match data {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };

        self.portfolio_strategy_plot
            .update_many_series(vec![data.1]);
        self.portfolio_strategy_plot.update_x_range(data.0.x_range);
        self.portfolio_strategy_plot.update_y_range(data.0.y_range);
    }

    /// Standard view for a single data element with a title and caption.
    pub fn data_title_caption<'a, Message>(
        &self,
        data: ExcaliburText,
        title: &'a str,
        caption: &'a str,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        double_labeled_data(
            data.build(),
            label(title).highlight().build(),
            label(caption).secondary().caption().build(),
        )
        .into()
    }

    pub fn internal_price<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.model.raw_internal_spot_price.to_label();

        // todo: add a tooltip to the caption
        // todo: get the proper caption too!
        self.data_title_caption(data, "Internal Price", "ETH/USD")
    }

    pub fn external_price<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.model.raw_external_spot_price.to_label();
        self.data_title_caption(data, "External Price", "ETH/USD")
    }

    pub fn internal_portfolio_value<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.model.internal_portfolio_value().to_label();
        self.data_title_caption(data, "Portfolio Value", "USD")
    }

    pub fn portfolio_health<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.model.portfolio_health().to_label();
        self.data_title_caption(data, "Portfolio Health", "PFV / tPFV")
    }

    /// todo: this is a hack because right now all the reserves == all the
    /// deposits.
    pub fn tvl<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let data = self.model.external_portfolio_value().to_label();
        self.data_title_caption(data, "TVL", "USD")
    }

    /// Plots the portfolio value series per block.
    pub fn portfolio_value_series(&self) -> Element<'_, chart::Message> {
        self.portfolio_value_series.build()
    }

    /// Plots the portfolio strategy.
    pub fn portfolio_strategy_plot(&self) -> Element<'_, chart::Message> {
        self.portfolio_strategy_plot.build()
    }
}
