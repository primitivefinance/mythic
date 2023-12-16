use std::{env, path::PathBuf};

use chrono::Utc;
use datatypes::portfolio::position::{Position, Positions};
use iced::{
    widget::{image, Image, Space},
    window::Icon,
};
use iced_aw::{graphics::icons::icon_to_char, Icon::Info, ICON_FONT};

use super::{inventory::Inventory, *};
use crate::{
    components::{
        logos::{ether_logo, usdc_logo},
        system::{ExcaliburButton, ExcaliburColor, ExcaliburContainer, ExcaliburText},
    },
    view::portfolio_view::ValueToLabel,
};

#[derive(Debug, Clone, Default)]
pub struct MonolithicPresenter {
    model: Model,
}

impl MonolithicPresenter {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    pub fn get_positions(&self) -> (Positions, Vec<String>) {
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

        let current_dir = env::current_dir().unwrap();
        let ether_logo_path =
            PathBuf::from(current_dir.clone()).join("assets/logos/ether_logo.png");
        let usdc_logo_path = PathBuf::from(current_dir.clone()).join("assets/logos/usdc_logo.png");

        let logos = vec![
            ether_logo_path.to_str().unwrap().to_string(),
            usdc_logo_path.to_str().unwrap().to_string(),
        ];

        (vec![position_x, position_y].into(), logos)
    }

    pub fn get_metrics(
        &self,
        address: AlloyAddress,
    ) -> (ExcaliburText, ExcaliburText, ExcaliburText, ExcaliburText) {
        let position = self
            .model
            .user
            .portfolio
            .positions
            .0
            .iter()
            .find(|x| x.asset.address == address)
            .cloned();

        if let Some(position) = position {
            let external_price = match position.asset.symbol.as_str() {
                "X" => self.model.portfolio.raw_external_spot_price.to_label(),
                "Y" => self.model.portfolio.raw_external_quote_price.to_label(),
                _ => label("n/a").title3().quantitative(),
            };

            let aum = self
                .model
                .portfolio
                .derive_internal_portfolio_value()
                .to_label();

            let health = self.model.portfolio.derive_portfolio_health().to_label();

            (
                label(position.asset.name).title1(),
                external_price,
                aum,
                health,
            )
        } else {
            (
                label("n/a").title3().secondary(),
                label("n/a").title3().quantitative(),
                label("N/a").title3().quantitative(),
                label("n/a").title3().quantitative(),
            )
        }
    }
}

pub struct MonolithicView;

impl MonolithicView {
    pub fn new() -> Self {
        Self
    }

    pub fn layout<'a, Message>(
        &self,
        positions: Positions,
        logos: Vec<String>,
        on_allocate: Option<Message>,
        on_select_position: impl Fn(AlloyAddress) -> Message + 'static,
        on_input: impl Fn(Option<String>) -> Message + 'static,
    ) -> Container<'a, Message>
    where
        Message: 'static + Default + Clone,
    {
        ExcaliburContainer::default()
            .build(
                Column::new()
                    .spacing(Sizes::Lg)
                    .push(Inventory::layout(
                        positions,
                        logos,
                        on_allocate,
                        on_select_position,
                    ))
                    .push(
                        label(&format!(
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

    /// Single container with a header, tooltip icon, and body content.
    /// Body content fills the remaining space.
    /// Light gray border surrounds the container, with no background.
    pub fn item_layout<'a, Message>(
        &self,
        title: impl ToString,
        element: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a,
    {
        ExcaliburContainer::default()
            .transparent()
            .light_border()
            .build(
                self.stacked_containers(
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
                ),
            )
    }

    /// Stacks three containers into a compact card with a max_height.
    /// Expects containers to be edited already except for border radius.
    pub fn stacked_containers<'a, Message>(
        &self,
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
