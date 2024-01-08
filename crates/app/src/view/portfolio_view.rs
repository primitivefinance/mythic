//! In the Model-view-controller architecture the view is
//! ["Any representation of information such as a chart, diagram or table."](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller).
//!
//! The portfolio view takes in the portfolio's data model and builds components
//! for the [controller](./mod.rs) to render.

use chrono::{DateTime, Utc};
use datatypes::portfolio::position::Positions;

use super::{model::portfolio::AlloyU256, *};
use crate::components::{
    double_labeled_data,
    system::{label, ExcaliburChart, ExcaliburTable, ExcaliburText},
    tables::cells::CellBuilder,
};

pub trait ValueToLabel<V> {
    fn to_label(&self) -> ExcaliburText;
}

impl ValueToLabel<Option<AlloyU256>> for AlloyU256 {
    fn to_label(&self) -> ExcaliburText {
        let value = alloy_primitives::utils::format_ether(*self);
        match value.parse::<f64>() {
            Ok(_) => label(&value).quantitative().title1(),
            Err(_) => label("Failed to parse U256 as float.").caption().tertiary(),
        }
    }
}

impl ValueToLabel<Result<AlloyU256, anyhow::Error>> for Result<AlloyU256, anyhow::Error> {
    fn to_label(&self) -> ExcaliburText {
        if let Ok(value) = self {
            let value = alloy_primitives::utils::format_ether(*value);
            match value.parse::<f64>() {
                Ok(_) => label(&value).quantitative().title1(),
                Err(_) => label("Failed to parse U256 as float.").caption().tertiary(),
            }
        } else {
            label("N/A").title1().secondary()
        }
    }
}

/// Responsible for transforming the model data into components for the view to
/// handle, e.g. converting an `AlloyU256` into a `ExcaliburText` that can be
/// shown to the user.
#[derive(Debug, Clone)]
pub struct PortfolioPresenter {
    model: Model,
    /// Value series is a live chart of portfolio value wrt. time.
    /// Therefore, it is continuously updated.
    pub portfolio_value_series: ExcaliburChart,
    /// Strategy plot can be computationally expensive. It is also static most
    /// of the time, so it is cached and only force updated on user request.
    pub portfolio_strategy_plot: ExcaliburChart,
}

impl Default for PortfolioPresenter {
    /// Makes `new` ExcaliburCharts so that the series and points are not set.
    /// The default ExcaliburChart sets a line series and single point,
    /// which affects the starting ranges of the chart.
    fn default() -> Self {
        Self {
            model: Model::default(),
            portfolio_value_series: ExcaliburChart::new(),
            portfolio_strategy_plot: ExcaliburChart::new(),
        }
    }
}

#[allow(dead_code)]
impl PortfolioPresenter {
    #[allow(dead_code)]
    pub fn new(
        model: Model,
        portfolio_value_series: ExcaliburChart,
        portfolio_strategy_plot: ExcaliburChart,
    ) -> Self {
        Self {
            model,
            portfolio_value_series,
            portfolio_strategy_plot,
        }
    }

    /// Updates the model and syncs the chart data to match the model.
    pub fn update(&mut self, model: Model) {
        self.model = model;
        self.sync_portfolio_value_series();
        self.sync_portfolio_strategy_points();

        // Update static if needed.
        if self.static_needs_update() {
            self.sync_portfolio_strategy_curve();
        }
    }

    /// Returns true if the static chart data is empty.
    pub fn static_needs_update(&self) -> bool {
        self.portfolio_strategy_plot.chart.series.is_empty()
    }

    /// Does not update the model, only updates the chart data to match the
    /// existing model.
    /// todo: these can be computationally costly, we should only do this when
    /// the model changes or the user forces it.
    pub fn sync_portfolio_strategy_curve(&mut self) {
        // Get the series data.
        let data = self.model.portfolio.derive_portfolio_strategy_plot();
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
        // this does not update the model. it just gets the data from the model and
        // turns it into a `ChartPoint` struct.
        let data = self.model.portfolio.derive_portfolio_strategy_points();
        let data = match data {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };
        // this updates the chart with the data from the model.
        self.portfolio_strategy_plot.override_points(data);
    }

    /// Does not update the model, only updates the chart data to match the
    /// existing model.
    /// todo: these can be computationally costly, we should only do this
    /// when the model changes or the user forces it.
    pub fn sync_portfolio_value_series(&mut self) {
        // Get the series data.
        let data = self.model.portfolio.derive_portfolio_value_series();
        let data = match data {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to get portfolio value series: {:}", e);
                return;
            }
        };

        let asset_value_series = self.model.portfolio.derive_asset_value_series();
        match asset_value_series {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to get asset value series: {:}", e);
                return;
            }
        };

        let quote_value_series = self.model.portfolio.derive_quote_value_series();
        match quote_value_series {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to get quote value series: {:}", e);
                return;
            }
        };

        let unallocated_value_series = self
            .model
            .portfolio
            .derive_unallocated_portfolio_value_series();
        match unallocated_value_series {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to get unallocated value series: {:}", e);
                return;
            }
        };

        let protocol_quote_value_series = self.model.portfolio.derive_protocol_quote_value_series();
        match protocol_quote_value_series {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to get protocol quote value series: {:}", e);
                return;
            }
        };

        let protocol_asset_value_series = self.model.portfolio.derive_protocol_asset_value_series();
        match protocol_asset_value_series {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to get protocol asset value series: {:}", e);
                return;
            }
        };

        // todo: make series toggleable.
        self.portfolio_value_series.override_series(vec![data.1]);
        self.portfolio_value_series.update_x_range(data.0.x_range);
        self.portfolio_value_series.update_y_range(data.0.y_range);
        // Only happens once.
        self.portfolio_value_series.override_ranges_flag(true);
    }

    pub fn get_block_number(&self) -> u64 {
        self.model.portfolio.latest_block
    }
    #[allow(dead_code)]
    pub fn get_block_timestamp(&self) -> DateTime<Utc> {
        self.model.portfolio.latest_timestamp
    }
    #[allow(dead_code)]
    pub fn get_internal_price(&self) -> ExcaliburText {
        self.model.portfolio.internal_spot_price.to_label()
    }

    pub fn get_external_price(&self) -> ExcaliburText {
        self.model.portfolio.external_spot_price.to_label()
    }

    pub fn get_internal_portfolio_value(&self) -> ExcaliburText {
        self.model
            .portfolio
            .derive_internal_portfolio_value()
            .to_label()
    }

    pub fn get_portfolio_health(&self) -> ExcaliburText {
        let data = self.model.portfolio.derive_portfolio_health();
        match data {
            Ok(data) => {
                let value = alloy_primitives::utils::format_ether(data);
                match value.parse::<f64>() {
                    Ok(_) => label(value.to_string()).title1().percentage(),
                    Err(_) => label("Failed to parse U256 as float.").caption().tertiary(),
                }
            }
            Err(_) => label("N/A").title1().secondary(),
        }
    }

    pub fn get_external_portfolio_value(&self) -> ExcaliburText {
        self.model
            .portfolio
            .derive_external_portfolio_value()
            .to_label()
    }

    pub fn get_last_sync_timestamp(&self) -> ExcaliburText {
        let data = self.model.portfolio.latest_timestamp;
        label(format!("Timestamp: {:}", data)).caption().tertiary()
    }

    pub fn get_last_sync_block(&self) -> ExcaliburText {
        let data = self.model.portfolio.latest_block;
        label(format!("Block: {:}", data)).caption().tertiary()
    }

    pub fn get_positions(&self) -> Positions {
        let portfolio = self.model.user.portfolio.clone();
        let position_x = portfolio
            .clone()
            .positions
            .0
            .iter()
            .find(|x| x.asset.symbol == "X")
            .cloned()
            .unwrap_or_default();

        let position_y = portfolio
            .clone()
            .positions
            .0
            .iter()
            .find(|x| x.asset.symbol == "Y")
            .cloned()
            .unwrap_or_default();

        vec![position_x, position_y].into()
    }
}

/// View all of the underlying data from the model for the portfolio dashboard.
///
/// Important! Make sure the chart starts completely blank (not default), this
/// will make sure the initial ranges are set correctly.
#[derive(Debug, Clone, Default)]
pub struct DataView;

#[allow(dead_code)]
impl DataView {
    /// Creates a layout for the portfolio metrics. It takes in various
    /// parameters such as the strategy plot, strategy plot title, external
    /// price, external and internal AUM, portfolio health, sync timestamp,
    /// and sync block. It returns a container with the portfolio metrics layout
    /// and chart layout.
    #[allow(clippy::too_many_arguments)]
    pub fn metrics_layout<'a, Message>(
        &'a self,
        strategy_plot: &'a ExcaliburChart,
        strategy_plot_title: ExcaliburText,
        external_price: ExcaliburText,
        external_aum: ExcaliburText,
        internal_aum: ExcaliburText,
        portfolio_health: ExcaliburText,
        sync_timestamp: ExcaliburText,
        sync_block: Option<u64>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default,
    {
        ExcaliburContainer::default()
            .build(
                Column::new()
                    .spacing(Sizes::Lg)
                    .push(self.portfolio_metrics_layout(
                        external_price,
                        external_aum,
                        internal_aum,
                        portfolio_health,
                        sync_block,
                    ))
                    .push(self.chart_layout(strategy_plot, strategy_plot_title, sync_timestamp)),
            )
            .padding(Sizes::Md)
    }

    /// Creates a layout for the portfolio metrics. It takes in various
    /// parameters such as the external price, external and internal AUM,
    /// portfolio health, and sync block. It returns a row with the
    /// portfolio metrics layout.
    pub fn portfolio_metrics_layout<'a, Message>(
        &'a self,
        external_price: ExcaliburText,
        external_aum: ExcaliburText,
        internal_aum: ExcaliburText,
        portfolio_health: ExcaliburText,
        sync_block: Option<u64>,
    ) -> Row<'a, Message>
    where
        Message: 'a + Default,
    {
        Row::new()
            .spacing(Sizes::Sm)
            .push(
                Container::new(self.external_price(external_price, sync_block))
                    .width(Length::FillPortion(1)),
            )
            .push(Container::new(self.tvl(external_aum)).width(Length::FillPortion(1)))
            .push(
                Container::new(self.internal_portfolio_value(internal_aum))
                    .width(Length::FillPortion(1)),
            )
            .push(
                Container::new(self.portfolio_health(portfolio_health))
                    .width(Length::FillPortion(1)),
            )
    }

    /// Creates a layout that includes a live chart and a greeting message for
    /// the user. It takes in various parameters such as the live chart,
    /// user greeting and message, chart title, and sync timestamp.
    /// It returns a container with the layout.
    pub fn chart_and_greet_layout<'a, Message>(
        &'a self,
        live_chart: &'a ExcaliburChart,
        user_greeting: ExcaliburText,
        user_message: ExcaliburText,
        chart_title: ExcaliburText,
        sync_timestamp: ExcaliburText,
    ) -> Container<'a, Message>
    where
        Message: 'a + Default,
    {
        ExcaliburContainer::default()
            .build(
                Column::new()
                    .spacing(Sizes::Lg)
                    .push(self.user_message_layout(user_greeting, user_message))
                    .push(self.chart_layout(live_chart, chart_title, sync_timestamp)),
            )
            .padding(Sizes::Md)
    }

    /// Creates a layout that includes a greeting message for the user.
    pub fn user_message_layout<'a, Message>(
        &'a self,
        user_greeting: ExcaliburText,
        user_message: ExcaliburText,
    ) -> Column<'a, Message>
    where
        Message: 'a + Default,
    {
        Column::new()
            .spacing(Sizes::Sm)
            .push(user_greeting.build())
            .push(user_message.build())
            .push(
                label(format!("Date: {}", Utc::now().format("%Y-%m-%d")))
                    .caption()
                    .tertiary()
                    .build(),
            )
    }

    /// Creates a layout for the chart.
    pub fn chart_layout<'a, Message>(
        &'a self,
        chart: &'a ExcaliburChart,
        chart_title: ExcaliburText,
        sync_timestamp: ExcaliburText,
    ) -> Column<'a, Message>
    where
        Message: 'a + Default,
    {
        Column::new()
            .spacing(Sizes::Md)
            .push(chart_title.build())
            .push(chart.build().map(|_| Message::default()))
            .push(sync_timestamp.build())
    }

    /// Creates a layout for the table.
    pub fn table_layout<'a, Message>(
        &'a self,
        table_title: ExcaliburText,
        table_actions: Vec<Element<'a, Message>>,
        table_builder: ExcaliburTable<Message>,
        table_cells: Vec<Vec<CellBuilder<Message>>>,
        last_sync_timestamp: ExcaliburText,
        last_sync_block: ExcaliburText,
    ) -> Column<'a, Message>
    where
        Message: 'a + Default + Clone,
    {
        let actions = Row::with_children(table_actions).spacing(Sizes::Sm);

        Column::new()
            .spacing(Sizes::Lg)
            .push(
                Row::new()
                    .spacing(Sizes::Md)
                    .push(table_title.build())
                    .push(actions),
            )
            .push(
                Column::new()
                    .spacing(Sizes::Sm)
                    .push(table_builder.build_custom(table_cells).build())
                    .push(
                        Row::new()
                            .spacing(Sizes::Sm)
                            .push(last_sync_timestamp.build())
                            .push(last_sync_block.build()),
                    ),
            )
    }

    // Individual data elements.
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
            label(title).highlight().build(),
            label(caption).secondary().caption().build(),
        )
        .into()
    }
    #[allow(dead_code)]
    pub fn internal_price<'a, Message>(
        &self,
        data: ExcaliburText,
        block_number: Option<u64>,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let caption = format!("ETH/USD @ {}", block_number.unwrap_or(0));

        // todo: add a tooltip to the caption
        // todo: get the proper caption too!
        self.data_title_caption(data, "Price".to_string(), caption)
    }

    pub fn external_price<'a, Message>(
        &self,
        data: ExcaliburText,
        block_number: Option<u64>,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let caption = format!("ETH/USD @ Block {}", block_number.unwrap_or(0));
        self.data_title_caption(data, "Price".to_string(), caption)
    }

    pub fn internal_portfolio_value<'a, Message>(&self, data: ExcaliburText) -> Element<'a, Message>
    where
        Message: 'a,
    {
        self.data_title_caption(data, "Internal AUM".to_string(), "USD".to_string())
    }

    pub fn portfolio_health<'a, Message>(&self, data: ExcaliburText) -> Element<'a, Message>
    where
        Message: 'a,
    {
        self.data_title_caption(
            data,
            "Portfolio Health".to_string(),
            "PFV / tPFV".to_string(),
        )
    }

    /// todo: this is a hack because right now all the reserves == all the
    /// deposits.
    pub fn tvl<'a, Message>(&self, data: ExcaliburText) -> Element<'a, Message>
    where
        Message: 'a,
    {
        self.data_title_caption(data, "External AUM".to_string(), "USD".to_string())
    }
    #[allow(dead_code)]
    pub fn get_positions_table<Message>(&self, positions: Positions) -> Element<'_, Message>
    where
        Message: 'static + Default + Clone,
    {
        let (table_builder, cell_data) = self.get_positions_table_builder(positions);
        table_builder.build_custom(cell_data).into()
    }

    /// Responsible for creating a table builder for the positions.
    /// It takes a `Positions` object as input and returns a tuple of
    /// `ExcaliburTable` and a 2D vector of `CellBuilder`.
    /// The `ExcaliburTable` is a custom table object that allows for the
    /// creation of a table with custom headers and cell data. The 2D vector
    /// of `CellBuilder` represents the cell data for each row in the table.
    ///
    /// The function iterates over each position in the `Positions` object and
    /// extracts the asset symbol, cost, balance, and weight. If all these
    /// values are present, they are formatted as strings and added to the cell
    /// data.
    ///
    /// If no positions are present, an empty cell with a "No data" label is
    /// added to the cell data. Finally, an `ExcaliburTable` is created with
    /// headers "Asset", "Price", "Balance", and "Weight", and the cell data is
    /// returned.
    pub fn get_positions_table_builder<Message>(
        &self,
        positions: Positions,
    ) -> (ExcaliburTable<Message>, Vec<Vec<CellBuilder<Message>>>)
    where
        Message: 'static + Default + Clone,
    {
        let mut cell_data: Vec<Vec<CellBuilder<Message>>> = vec![];

        for position in positions.0 {
            let asset = position.asset.symbol;
            let cost = position.cost;
            let balance = position.balance;
            let weight = position.weight;

            if let (Some(cost), Some(balance), Some(weight)) = (cost, balance, weight) {
                let cost = format!("{}", cost);
                let balance = format!("{}", balance);
                let weight = format!("{}", weight);

                cell_data.push(vec![
                    CellBuilder::new().child(label(&asset).body().build()),
                    CellBuilder::new().child(label(&cost).quantitative().build()),
                    CellBuilder::new().child(label(&balance).quantitative().build()),
                    CellBuilder::new().child(label(&weight).percentage().build()),
                ]);
            }
        }

        // If no positions add an empty cell with "no data" label.
        if cell_data.is_empty() {
            cell_data.push(vec![
                CellBuilder::new().child(label("No data").secondary().build())
            ]);
        }

        (
            ExcaliburTable::new()
                .header("Asset")
                .header("Price")
                .header("Balance")
                .header("Weight"),
            cell_data,
        )
    }
}
