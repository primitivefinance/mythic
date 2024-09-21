use chrono::Utc;
use iced::widget::{svg, Space};
use iced_aw::{graphics::icons::icon_to_char, Icon::Info};

use self::logos::lp_logo;
use super::*;
use crate::{
    components::{
        logos::{ether_logo, usdc_logo},
        system::{ExcaliburContainer, ExcaliburHistogram, ExcaliburText},
    },
    model::portfolio::HistoricalTx,
    view::portfolio_view::ValueToLabel,
};

#[derive(Debug, Clone, Default)]
pub struct MonolithicPresenter {
    model: Model,
    pub historical_txs: Vec<HistoricalTx>,
    pub cached_strategy_preview: ExcaliburChart,
    pub cached_strategy_histogram: ExcaliburHistogram,
}

impl MonolithicPresenter {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            cached_strategy_preview: ExcaliburChart::new(),
            cached_strategy_histogram: ExcaliburHistogram::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, model: Model) {
        self.model = model;
    }

    pub fn cache_historical_txs(&mut self, txs: Vec<HistoricalTx>) {
        self.historical_txs = txs;
    }

    pub fn get_last_sync_timestamp(&self) -> ExcaliburText {
        if self.model.get_current().is_none() {
            return label("Timestamp: N/A").caption().tertiary();
        }

        let data = self.model.get_current().unwrap().last_sync;
        match data {
            Some(data) => label(format!("Timestamp: {:}", data)).caption().tertiary(),
            None => label("Timestamp: N/A").caption().tertiary(),
        }
    }

    pub fn get_historical_txs(&self) -> Vec<HistoricalTx> {
        if self.model.get_current().is_none() {
            return vec![];
        }

        self.model
            .get_current()
            .unwrap()
            .user_history
            .clone()
            .unwrap_or_default()
    }
}

#[allow(dead_code)]
pub const M_WIDTH: f32 = 920.0;

pub struct MonolithicView;

impl MonolithicView {
    pub fn layout<'a, Message>() -> Container<'a, Message>
    where
        Message: 'static + Default + Clone,
    {
        ExcaliburContainer::default()
            .build(
                Column::new().spacing(Sizes::Lg).push(
                    label(format!(
                        "Timestamp: {}",
                        Utc::now().format("%Y-%m-%d %H:%M:%S")
                    ))
                    .caption()
                    .tertiary()
                    .build(),
                ),
            )
            .center_x()
            .width(Length::Fill)
    }

    #[allow(dead_code)]
    pub fn item_layout<'a, Message>(
        title: impl ToString,
        element: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a,
    {
        ExcaliburContainer::default()
            .transparent()
            .light_border()
            .build(Self::stacked_containers(
                ExcaliburContainer::default().transparent().light_border(),
                ExcaliburContainer::default().transparent(),
                ExcaliburContainer::default().transparent(),
                Row::new()
                    .spacing(Sizes::Sm)
                    .width(Length::Fill)
                    .push(
                        Column::new()
                            .width(Length::FillPortion(4))
                            .push(label(title).secondary().build()),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .push(label(icon_to_char(Info)).icon().secondary().build())
                            .align_items(alignment::Alignment::End),
                    ),
                element,
                Space::new(Length::Fill, 0.0),
                9.0,
                800.0,
            ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stacked_containers<'a, Message>(
        header: ExcaliburContainer,
        body: ExcaliburContainer,
        footer: ExcaliburContainer,
        header_content: impl Into<Element<'a, Message>>,
        body_content: impl Into<Element<'a, Message>>,
        footer_content: impl Into<Element<'a, Message>>,
        border_radius: f32,
        max_height: f32,
    ) -> Column<'a, Message>
    where
        Message: 'a,
    {
        Column::with_children(vec![
            header
                .border_radius([border_radius, border_radius, 0.0, 0.0].into())
                .build(header_content)
                .center_x()
                .padding(Sizes::Sm)
                .width(Length::Fill)
                .into(),
            body.build(body_content)
                .width(Length::Fill)
                .padding(Sizes::Lg)
                .center_x()
                .center_y()
                .max_height(max_height)
                .into(),
            footer
                .border_radius([0.0, 0.0, border_radius, border_radius].into())
                .build(footer_content)
                .center_x()
                .padding(Sizes::Md)
                .width(Length::Fill)
                .into(),
        ])
    }
}
