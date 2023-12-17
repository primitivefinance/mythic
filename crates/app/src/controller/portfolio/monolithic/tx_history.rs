use super::*;
use crate::{
    components::{
        system::{ExcaliburContainer, ExcaliburTable},
        tables::{builder::TableBuilder, cells::CellBuilder, columns::ColumnBuilder},
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
                .push(table.into()),
        )
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
                CellBuilder::new().child(label("Success").build()),
                CellBuilder::new().child(label(&tx.tx_hash).secondary().caption().build()),
            ]);
        }

        ExcaliburTable::new()
            .headers(vec![
                "Position",
                "Action",
                "Market Value",
                "Result",
                "Tx hash",
            ])
            .build_custom(cells)
    }
}
