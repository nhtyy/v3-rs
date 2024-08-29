use crate::V3Pool;
use crate::position::PositionManager;
use crate::position::Balances;

use ethers::types::Address;
use ethers::providers::Middleware;


#[async_trait::async_trait]
pub trait LiquidityExt: V3Pool {
    async fn lp_balance<'a, M: Middleware + 'static>(
        &'a self,
        manager: &'a PositionManager<M>,
        who: Address,
    ) -> anyhow::Result<Balances<'a, Self>> {
        manager.total_positions_balance(self, who).await
    }
}

impl<P: V3Pool> LiquidityExt for P {}