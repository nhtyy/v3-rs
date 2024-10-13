use std::future::Future;

use crate::error::V3PoolError;
use crate::math::Tick;
use crate::position::Balances;
use crate::traits::{Batch, IntoFloat};
// use crate::pool::{FeeTier, Tick, TickSpacing, V3Pool, V3PoolError};
use crate::{AlloyManager, PoolResult, V3Pool};
use alloy::contract::Error as ContractError;
use alloy::network::Network;
use alloy::primitives::Signed;
use alloy::primitives::{Address, Uint};
use alloy::providers::Provider;
use alloy::transports::{Transport, TransportError};

use rug::Float;
use V3PoolContract::V3PoolContractInstance;

pub type Pool<T, P, N> = AlloyPool<T, P, N, VanillaMarker>;

#[cfg(feature = "aerodrome")]
pub type AerodromePool<T, P, N> = AlloyPool<T, P, N, AerodromeMarker>;

alloy::sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    interface V3PoolContract {
        /// @notice The currently in range liquidity available to the pool
        /// @dev This value has no relationship to the total liquidity across all ticks
        function liquidity() external view returns (uint128);

        /// @notice The first of the two tokens of the pool, sorted by address
        /// @return The token contract address
        function token0() external view returns (address);

        /// @notice The second of the two tokens of the pool, sorted by address
        /// @return The token contract address
        function token1() external view returns (address);

        /// @notice The pool's fee in hundredths of a bip, i.e. 1e-6
        /// @return The fee
        function fee() external view returns (uint24);

        /// @notice The pool tick spacing
        /// @dev Ticks can only be used at multiples of this value, minimum of 1 and always positive
        /// e.g.: a tickSpacing of 3 means ticks can be initialized every 3rd tick, i.e., ..., -6, -3, 0, 3, 6, ...
        /// This value is an int24 to avoid casting even though it is always positive.
        /// @return The tick spacing
        function tickSpacing() external view returns (int24);
    }

    #[sol(rpc)]
    interface Vanilla {
        function slot0()
            external
            view
            returns (
                uint160 sqrtPriceX96,
                int24 tick,
                uint16 observationIndex,
                uint16 observationCardinality,
                uint16 observationCardinalityNext,
                uint8 feeProtocol,
                bool unlocked
            );

        function ticks(int24 tick)
            external
            view
            returns (
                uint128 liquidityGross,
                int128 liquidityNet,
                uint256 feeGrowthOutside0X128,
                uint256 feeGrowthOutside1X128,
                int56 tickCumulativeOutside,
                uint160 secondsPerLiquidityOutsideX128,
                uint32 secondsOutside,
                bool initialized
            );
    }

    #[sol(rpc)]
    #[cfg(feature = "aerodrome")]
    interface Aerodrome {
        function slot0()
            external
            view
            returns (
                uint160 sqrtPriceX96,
                int24 tick,
                uint16 observationIndex,
                uint16 observationCardinality,
                uint16 observationCardinalityNext,
                bool unlocked
            );

        function ticks(int24 tick)
            external
            view
            returns (
                uint128 liquidityGross,
                int128 liquidityNet,
                int128 stakedLiquidityNet,
                uint256 feeGrowthOutside0X128,
                uint256 feeGrowthOutside1X128,
                uint256 rewardGrowthOutsideX128,
                int56 tickCumulativeOutside,
                uint160 secondsPerLiquidityOutsideX128,
                uint32 secondsOutside,
                bool initialized
            );
    }
}

/// The alloy implementation of an on chain v3 pool.
///
/// See the [crate::V3Pool] trait for more information
pub struct AlloyPool<T, P, N, M> {
    pool: V3PoolContractInstance<T, P, N>,
    tick_spacing: Signed<24, 1>,
    token0: Address,
    token1: Address,
    token0_decimals: u8,
    token1_decimals: u8,
    fee: Uint<24, 1>,
    _marker: std::marker::PhantomData<M>,
}

impl<T, P, N, M> AlloyPool<T, P, N, M>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    pub async fn new(
        pool: V3PoolContractInstance<T, P, N>,
    ) -> Result<Self, alloy::contract::Error> {
        let token0 = pool.token0().call().await?._0;
        let token1 = pool.token1().call().await?._0;

        let token0_decimals = crate::utils::decimals(pool.provider(), token0).await?;
        let token1_decimals = crate::utils::decimals(pool.provider(), token1).await?;

        let tick_spacing = pool.tickSpacing().call().await?._0;
        let fee = pool.fee().call().await?._0;

        Ok(Self {
            pool,
            tick_spacing,
            token0,
            token1,
            token0_decimals,
            token1_decimals,
            fee,
            _marker: std::marker::PhantomData,
        })
    }
}

impl<T, P, N, M> AlloyPool<T, P, N, M>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
    Self: V3Pool<BackendError = alloy::contract::Error>,
{
    /// Get the (NFT) LP balance owned by the address
    ///
    /// # Errors
    /// - If the chain constants are not supported (see [crate::constants::NETWORKS] & [Self::lp_balance_with_manager])
    pub async fn lp_balance(
        &self,
        who: Address,
    ) -> Result<Balances<Self>, V3PoolError<alloy::contract::Error>> {
        let chain_id = self
            .pool
            .provider()
            .get_chain_id()
            .await
            .map_err(TransportError::from)
            .map_err(alloy::contract::Error::from)
            .map_err(V3PoolError::backend_error)?;

        let manager = AlloyManager::new(
            crate::constants::NETWORKS
                .get(&chain_id)
                .ok_or(V3PoolError::UnsupportedChain)?
                .manager,
            self.pool.provider(),
        );

        manager.total_positions_balance(self, who).await
    }

    /// Returns all the NFT liquidty positions for this manager
    ///
    /// Manager: The nft position manager contract to query
    pub async fn lp_balance_with_manager<P2>(
        &self,
        manager: &AlloyManager<T, P2, N>,
        who: Address,
    ) -> Result<Balances<Self>, V3PoolError<alloy::contract::Error>>
    where
        P2: Provider<T, N>,
    {
        manager.total_positions_balance(self, who).await
    }
}

#[async_trait::async_trait]
impl<T, P, N, M> V3Pool for AlloyPool<T, P, N, M>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
    M: ForkMarker<T, P, N>,
{
    type BackendError = alloy::contract::Error;

    /// will error if the ticks arent n * spacing apart
    async fn tick_range(
        &self,
        starting: Tick,
        ending: Tick,
    ) -> PoolResult<Vec<i128>, Self::BackendError> {
        let spacing: i32 = self.tick_spacing.unchecked_into();
        let mut down: bool = false;

        let differnce = starting - ending;
        let differnce = differnce.abs();
        let capactiy = differnce / spacing + 1;

        if capactiy > 500 {
            // todo should we just maxx out?
            return Err(V3PoolError::TooManyTicks);
        }

        tracing::trace!("getting tick range");
        tracing::trace!(
            "starting: {:?}, ending: {:?}, spacing: {:?}",
            starting,
            ending,
            spacing
        );
        tracing::trace!("difference: {}, capacity: {}", differnce, capactiy);

        if differnce % spacing != 0 {
            let tick_spacing = self.tick_spacing;

            return Err(V3PoolError::BadTickRange(starting, ending, tick_spacing));
        }

        if starting > ending {
            down = true;
        }

        if starting == ending {
            return Ok(vec![]);
        }

        Ok(M::liquidity_net(self, starting, ending, down)
            .await
            .map_err(V3PoolError::backend_error)?
            .into_iter()
            .map(|x| if down { -x } else { x })
            .collect::<Vec<_>>())
    }

    fn tick_spacing(&self) -> crate::I24 {
        self.tick_spacing
    }

    async fn current_liquidity(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        let liquidity = self
            .pool
            .liquidity()
            .call()
            .await
            .map_err(V3PoolError::backend_error)?;

        tracing::trace!("current liquidity: {}", liquidity._0);

        Ok(Float::with_val(100, liquidity._0))
    }

    async fn sqrt_price_x96(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        M::sqrt_price_x96(self)
            .await
            .map_err(V3PoolError::backend_error)
    }

    async fn tick(&self, tick: Tick) -> Result<Float, V3PoolError<Self::BackendError>> {
        M::tick(self, tick)
            .await
            .map_err(V3PoolError::backend_error)
    }

    fn token0(&self) -> &Address {
        &self.token0
    }

    fn token1(&self) -> &Address {
        &self.token1
    }

    fn token0_decimals(&self) -> &u8 {
        &self.token0_decimals
    }

    fn token1_decimals(&self) -> &u8 {
        &self.token1_decimals
    }

    fn fee(&self) -> alloy::primitives::Uint<24, 1> {
        self.fee
    }

    fn address(&self) -> Address {
        *self.pool.address()
    }
}

/// The vanilla implementation of an on chain v3 pool.
pub struct VanillaMarker;

/// The aerodrome implementation of an on chain v3 pool. 
#[cfg(feature = "aerodrome")]
pub struct AerodromeMarker;

trait ForkMarker<T, P, N>: Sized + Send + Sync + 'static {
    fn liquidity_net(
        instance: &AlloyPool<T, P, N, Self>,
        start: Tick,
        end: Tick,
        down: bool,
    ) -> impl Future<Output = Result<Vec<i128>, ContractError>> + Send;

    fn tick(
        instance: &AlloyPool<T, P, N, Self>,
        tick: Tick,
    ) -> impl Future<Output = Result<Float, ContractError>> + Send;

    fn sqrt_price_x96(
        instance: &AlloyPool<T, P, N, Self>,
    ) -> impl Future<Output = Result<Float, ContractError>> + Send;
}

impl<T, P, N> ForkMarker<T, P, N> for VanillaMarker
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    async fn liquidity_net(
        instance: &AlloyPool<T, P, N, Self>,
        start: Tick,
        end: Tick,
        down: bool,
    ) -> Result<Vec<i128>, ContractError> {
        let pool =
            Vanilla::VanillaInstance::new(*instance.pool.address(), instance.pool.provider());

        let mut calls = Vec::new();
        let mut current = start;
        // we know this should happen because we check that the diff is a multiple of the spacing
        while current != end {
            calls.push(pool.ticks(current.into()));

            if down {
                current = current.down(instance.tick_spacing);
            } else {
                current = current.up(instance.tick_spacing);
            }
        }

        Ok(calls
            .batch()
            .call()
            .await?
            .into_iter()
            .map(|x| x.liquidityNet)
            .collect())
    }

    async fn tick(instance: &AlloyPool<T, P, N, Self>, tick: Tick) -> Result<Float, ContractError> {
        let pool =
            Vanilla::VanillaInstance::new(*instance.pool.address(), instance.pool.provider());

        pool.ticks(tick.into())
            .call()
            .await
            .map(|x| Float::with_val(100, x.liquidityNet))
    }

    async fn sqrt_price_x96(instance: &AlloyPool<T, P, N, Self>) -> Result<Float, ContractError> {
        let pool =
            Vanilla::VanillaInstance::new(*instance.pool.address(), instance.pool.provider());

        pool.slot0()
            .call()
            .await
            .map(|x| x.sqrtPriceX96.into_float())
    }
}

#[cfg(feature = "aerodrome")]
impl<T, P, N> ForkMarker<T, P, N> for AerodromeMarker
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    async fn liquidity_net(
        instance: &AlloyPool<T, P, N, Self>,
        start: Tick,
        end: Tick,
        down: bool,
    ) -> Result<Vec<i128>, ContractError> {
        let pool =
            Aerodrome::AerodromeInstance::new(*instance.pool.address(), instance.pool.provider());

        let mut calls = Vec::new();
        let mut current = start;
        // we know this should happen because we check that the diff is a multiple of the spacing
        while current != end {
            calls.push(pool.ticks(current.into()));

            if down {
                current = current.down(instance.tick_spacing);
            } else {
                current = current.up(instance.tick_spacing);
            }
        }

        Ok(calls
            .batch()
            .call()
            .await?
            .into_iter()
            .map(|x| x.liquidityNet)
            .collect())
    }

    async fn tick(instance: &AlloyPool<T, P, N, Self>, tick: Tick) -> Result<Float, ContractError> {
        let pool =
            Aerodrome::AerodromeInstance::new(*instance.pool.address(), instance.pool.provider());

        pool.ticks(tick.into())
            .call()
            .await
            .map(|x| Float::with_val(100, x.liquidityNet))
    }

    async fn sqrt_price_x96(instance: &AlloyPool<T, P, N, Self>) -> Result<Float, ContractError> {
        let pool =
            Aerodrome::AerodromeInstance::new(*instance.pool.address(), instance.pool.provider());

        pool.slot0()
            .call()
            .await
            .map(|x| x.sqrtPriceX96.into_float())
    }
}
