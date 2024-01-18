use super::*;

#[derive(Debug)]
pub struct Tokens<C> {
    pub client: Arc<C>,
    pub protocol: DFMM<C>,
    pub ln_strategy: LogNormal<C>,
    pub ln_solver: LogNormalSolver<C>,
    pub g_strategy: G3M<C>,
    pub g_solver: G3MSolver<C>,
    pub token_x: Address,
    pub token_y: Address,
}
