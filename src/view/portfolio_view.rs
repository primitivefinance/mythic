use chrono::{DateTime, Utc};

use super::*;
use crate::components::{
    double_labeled_data,
    system::{label, ExcaliburChart, ExcaliburTable, ExcaliburText},
    tables::cells::CellBuilder,
};

pub trait ValueToLabel<V> {
    fn to_label(&self) -> ExcaliburText;
}

impl ValueToLabel<Option<U256>> for Option<U256> {
    fn to_label(&self) -> ExcaliburText {
        if let Some(value) = self {
            let value = ethers::utils::format_ether(*value);
            match value.parse::<f64>() {
                Ok(_) => label(&value).quantitative().title1(),
                Err(_) => label("Failed to parse U256 as float.").caption().tertiary(),
            }
        } else {
            label("N/A").title1().secondary()
        }
    }
}

impl ValueToLabel<Result<U256, anyhow::Error>> for Result<U256, anyhow::Error> {
    fn to_label(&self) -> ExcaliburText {
        if let Ok(value) = self {
            let value = ethers::utils::format_ether(*value);
            match value.parse::<f64>() {
                Ok(_) => label(&value).quantitative().title1(),
                Err(_) => label("Failed to parse U256 as float.").caption().tertiary(),
            }
        } else {
            label("N/A").title1().secondary()
        }
    }
}

#[derive(Debug, Clone)]
pub struct PortfolioPresenter {
    model: Model,
    pub portfolio_value_series: ExcaliburChart,
    pub portfolio_strategy_plot: ExcaliburChart,
}

impl Default for PortfolioPresenter {
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

    pub fn update(&mut self, model: Model) {
        self.model = model;
    }

    pub fn static_needs_update(&self) -> bool {
        self.portfolio_strategy_plot.chart.series.is_empty()
    }

    pub fn get_block_number(&self) -> Option<u64> {
        self.model.get_current().and_then(|x| x.last_sync_block)
    }
    #[allow(dead_code)]
    pub fn get_block_timestamp(&self) -> Option<DateTime<Utc>> {
        self.model.get_current().and_then(|x| x.last_sync)
    }

    pub fn get_last_sync_timestamp(&self) -> ExcaliburText {
        if let Some(connected_model) = self.model.get_current() {
            let data = connected_model.last_sync;
            match data {
                Some(data) => label(format!("Timestamp: {:}", data)).caption().tertiary(),
                None => label("Timestamp: N/A").caption().tertiary(),
            }
        } else {
            label("Timestamp: N/A").caption().tertiary()
        }
    }

    pub fn get_last_sync_block(&self) -> ExcaliburText {
        if let Some(connected_model) = self.model.get_current() {
            let data = connected_model.last_sync_block;
            match data {
                Some(data) => label(format!("Block: {:}", data)).caption().tertiary(),
                None => label("Block: N/A").caption().tertiary(),
            }
        } else {
            label("Block: N/A").caption().tertiary()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DataView;

#[allow(dead_code)]
impl DataView {
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

    pub fn tvl<'a, Message>(&self, data: ExcaliburText) -> Element<'a, Message>
    where
        Message: 'a,
    {
        self.data_title_caption(data, "External AUM".to_string(), "USD".to_string())
    }
}
