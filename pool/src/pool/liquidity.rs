// use crate::V3Pool;
// use crate::position::PositionManager;
// use crate::position::Balances;

// use ethers::types::Address;
// use ethers::providers::Middleware;


// #[async_trait::async_trait]
// pub trait LiqudityExt: V3Pool {
//     async fn lp_balance<M: Middleware + 'static>(
//         &self,
//         manager: &PositionManager<M>,
//         who: Address,
//     ) -> anyhow::Result<Balances> {
//         manager.total_positions_balance(self, who).await
//     }
// }