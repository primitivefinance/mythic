use iced::widget::Space;

use super::*;
use crate::{
    components::{
        system::{ExcaliburColor, ExcaliburContainer, ExcaliburTable},
        tables::{builder::TableBuilder, cells::CellBuilder},
    },
    model::portfolio::HistoricalTx,
};

#[derive(Debug, Clone, Default)]
pub struct TxHistory;

impl TxHistory {
    pub fn layout<'a, Message>(
        title: impl ToString,
        subtitle: impl ToString,
        table: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message>
    where
        Message: 'a + Clone,
    {
        ExcaliburContainer::default().build(
            Column::new()
                .spacing(Sizes::Md)
                .push(
                    Column::new()
                        .spacing(Sizes::Sm)
                        .push(label(subtitle).secondary().build())
                        .push(label(title).title1().build()),
                )
                .push(
                    ExcaliburContainer::default()
                        .light_border()
                        .build(table.into()),
                ),
        )
    }

    pub fn separator<'a, Message>() -> Container<'a, Message>
    where
        Message: 'a,
    {
        ExcaliburContainer::default()
            .background(ExcaliburColor::Custom(GRAY_600))
            .build(Space::new(Length::Fill, 1.0))
            .width(Length::Fill)
    }

    pub fn table<'a, Message>(txs: &Vec<HistoricalTx>) -> TableBuilder<Message>
    where
        Message: 'a + Clone + Default,
    {
        let mut cells: Vec<Vec<CellBuilder<Message>>> = Vec::new();

        for tx in txs {
            cells.push(vec![
                CellBuilder::new().child(label(&tx.position_name).build()),
                CellBuilder::new().child(label(&tx.action).build()),
                CellBuilder::new().child(label(&tx.market_value).quantitative().build()),
                CellBuilder::new().child(
                    label(&tx.timestamp.format("%Y-%m-%d %H:%M:%S %Z"))
                        .secondary()
                        .caption()
                        .build(),
                ),
                CellBuilder::new().child(label(&tx.tx_hash).secondary().caption().build()),
            ]);
        }

        // If the table is empty, add a placeholder row.
        if cells.is_empty() {
            cells.push(vec![CellBuilder::new().child(
                label("No transactions to show")
                    .secondary()
                    .caption()
                    .build(),
            )]);
        }

        ExcaliburTable::new()
            .headers(vec![
                "Position",
                "Action",
                "Value ($)",
                "Timestamp",
                "Tx hash",
            ])
            .build_custom(cells)
    }
}
