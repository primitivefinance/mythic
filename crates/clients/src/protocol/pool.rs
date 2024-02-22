use super::*;

#[derive(Debug, Clone, Copy)]
pub enum PoolKind {
    GeometricMean,
    LogNormal,
}

#[derive(Debug, Clone, Copy)]
pub struct Pool {
    pub kind: PoolKind,
    pub token_x: Address,
    pub token_y: Address,
    pub pool_id: U256,
}

impl Pool {
    pub async fn new(
        kind: PoolKind,
        token_x: Address,
        token_y: Address,
        pool_id: U256,
    ) -> Result<Self> {
        Ok(Self {
            kind,
            token_x,
            token_y,
            pool_id,
        })
    }
}
