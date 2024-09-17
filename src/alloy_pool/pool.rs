use crate::error::V3PoolError;
use crate::math::Tick;
use crate::position::Balances;
use crate::traits::IntoFloat;
// use crate::pool::{FeeTier, Tick, TickSpacing, V3Pool, V3PoolError};
use crate::{FeeTier, Manager, PoolResult, TickSpacing, V3Pool};
use alloy::contract::MultiCall;
use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::{Transport, TransportError};

use rug::Float;
use V3PoolContract::V3PoolContractInstance;

alloy::sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    interface V3PoolContract {
        /// @notice The currently in range liquidity available to the pool
        /// @dev This value has no relationship to the total liquidity across all ticks
        function liquidity() external view returns (uint128);

        /// @notice The 0th storage slot in the pool stores many values, and is exposed as a single method to save gas
        /// when accessed externally.
        /// @return sqrtPriceX96 The current price of the pool as a sqrt(token1/token0) Q64.96 value
        /// tick The current tick of the pool, i.e. according to the last tick transition that was run.
        /// This value may not always be equal to SqrtTickMath.getTickAtSqrtRatio(sqrtPriceX96) if the price is on a tick
        /// boundary.
        /// observationIndex The index of the last oracle observation that was written,
        /// observationCardinality The current maximum number of observations stored in the pool,
        /// observationCardinalityNext The next maximum number of observations, to be updated when the observation.
        /// feeProtocol The protocol fee for both tokens of the pool.
        /// Encoded as two 4 bit values, where the protocol fee of token1 is shifted 4 bits and the protocol fee of token0
        /// is the lower 4 bits. Used as the denominator of a fraction of the swap fee, e.g. 4 means 1/4th of the swap fee.
        /// unlocked Whether the pool is currently locked to reentrancy
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

        /// @notice Look up information about a specific tick in the pool
        /// @param tick The tick to look up
        /// @return liquidityGross the total amount of position liquidity that uses the pool either as tick lower or
        /// tick upper,
        /// liquidityNet how much liquidity changes when the pool price crosses the tick,
        /// feeGrowthOutside0X128 the fee growth on the other side of the tick from the current tick in token0,
        /// feeGrowthOutside1X128 the fee growth on the other side of the tick from the current tick in token1,
        /// tickCumulativeOutside the cumulative tick value on the other side of the tick from the current tick
        /// secondsPerLiquidityOutsideX128 the seconds spent per liquidity on the other side of the tick from the current tick,
        /// secondsOutside the seconds spent on the other side of the tick from the current tick,
        /// initialized Set to true if the tick is initialized, i.e. liquidityGross is greater than 0, otherwise equal to false.
        /// Outside values can only be used if the tick is initialized, i.e. if liquidityGross is greater than 0.
        /// In addition, these values are only relative and must be used only in comparison to previous snapshots for
        /// a specific position.
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
}

pub struct Pool<T, P, N> {
    pool: V3PoolContractInstance<T, P, N>,
    tick_spacing: TickSpacing,
    token0: Address,
    token1: Address,
    token0_decimals: u8,
    token1_decimals: u8,
    fee: FeeTier,
}

impl<T, P, N> Pool<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    pub async fn new(
        pool: V3PoolContractInstance<T, P, N>,
        fee: FeeTier,
    ) -> Result<Self, alloy::contract::Error> {
        let token0 = pool.token0().call().await?._0;
        let token1 = pool.token1().call().await?._0;

        let token0_decimals = crate::utils::decimals(pool.provider(), token0).await?;
        let token1_decimals = crate::utils::decimals(pool.provider(), token1).await?;

        Ok(Self {
            pool,
            tick_spacing: fee.as_spacing(),
            token0,
            token1,
            token0_decimals,
            token1_decimals,
            fee,
        })
    }

    /// Get the (NFT) LP balance owned by the address
    /// 
    /// # Errors
    /// - If the chain constants are not supported (see [crate::constants::NETWORKS] & [Self::lp_balance_with_manager])
    pub async fn lp_balance<'a>(
        &'a self,
        who: Address,
    ) -> Result<Balances<'a, Self>, V3PoolError<alloy::contract::Error>> {
        let chain_id = self
            .pool
            .provider()
            .get_chain_id()
            .await
            .map_err(TransportError::from)
            .map_err(alloy::contract::Error::from)
            .map_err(V3PoolError::backend_error)?;

        let manager = Manager::new(
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
    pub async fn lp_balance_with_manager<'a, P2>(
        &'a self,
        manager: &Manager<T, P2, N>,
        who: Address,
    ) -> Result<Balances<'a, Self>, V3PoolError<alloy::contract::Error>>
    where
        P2: Provider<T, N>,
    {
        manager.total_positions_balance(self, who).await
    }
}

#[async_trait::async_trait]
impl<T, P, N> V3Pool for Pool<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    type BackendError = alloy::contract::Error;

    /// will error if the ticks arent n * spacing apart
    async fn tick_range(
        &self,
        starting: Tick,
        ending: Tick,
    ) -> PoolResult<Vec<i128>, Self::BackendError> {
        let spacing = self.tick_spacing as i32;
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
            let tick_spacing = self.tick_spacing.clone();

            return Err(V3PoolError::BadTickRange(starting, ending, tick_spacing));
        }

        if starting > ending {
            down = true;
        }

        if starting == ending {
            return Ok(vec![]);
        }

        let multicall = MultiCall::new_checked(self.pool.provider()).await?;
        let mut aggregate = multicall.aggregate();
        aggregate.reserve(capactiy as usize);

        let mut current = starting;
        // we know this should happen because we check that the diff is a multiple of the spacing
        while current != ending {
            aggregate.add_call(self.pool.ticks(current.into()));

            if down {
                current = current.down(self.tick_spacing);
            } else {
                current = current.up(self.tick_spacing);
            }
        }

        Ok(aggregate
            .call()
            .await?
            .into_iter()
            .map(|x| {
                if down {
                    -x.liquidityNet
                } else {
                    x.liquidityNet
                }
            })
            .collect::<Vec<_>>())
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
        let slot = self
            .pool
            .slot0()
            .call()
            .await
            .map_err(V3PoolError::backend_error)?;

        Ok(slot.sqrtPriceX96.into_float())
    }

    async fn tick(&self, tick: Tick) -> Result<Float, V3PoolError<Self::BackendError>> {
        let tick = self
            .pool
            .ticks(tick.into())
            .call()
            .await
            .map_err(V3PoolError::backend_error)?;

        Ok(Float::with_val(100, tick.liquidityNet))
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

    fn fee(&self) -> &FeeTier {
        &self.fee
    }

    fn address(&self) -> Address {
        *self.pool.address()
    }
}
