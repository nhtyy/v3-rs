pub mod amount;
pub mod deltas;
pub mod price;

/// The index of the token in the pool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Zero,
    One,
}

// todo specailed fee tier?
#[derive(Debug, Clone, Copy)]
pub enum FeeTier {
    Min,
    Mid,
    Max,
}

impl FeeTier {
    /// Big endian representation of the fee tier
    /// used for encoding paths
    pub const fn as_u24_bytes(&self) -> [u8; 3] {
        match self {
            FeeTier::Min => [0, 1, 244],
            FeeTier::Mid => [0, 11, 184],
            FeeTier::Max => [0, 39, 16],
        }
    }

    pub const fn as_spacing(&self) -> TickSpacing {
        match self {
            FeeTier::Min => TickSpacing::Min,
            FeeTier::Mid => TickSpacing::Mid,
            FeeTier::Max => TickSpacing::Max,
        }
    }

    pub const fn as_bp(&self) -> u32 {
        match self {
            FeeTier::Min => 5,
            FeeTier::Mid => 30,
            FeeTier::Max => 100,
        }
    }

    pub const fn as_scaled_bp(&self) -> u32 {
        match self {
            FeeTier::Min => 500,
            FeeTier::Mid => 3000,
            FeeTier::Max => 10000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TickSpacing {
    Min = 10,
    Mid = 60,
    Max = 200,
}

impl TickSpacing {
    pub const fn as_fee(tick_spacing: TickSpacing) -> FeeTier {
        match tick_spacing {
            TickSpacing::Min => FeeTier::Min,
            TickSpacing::Mid => FeeTier::Mid,
            TickSpacing::Max => FeeTier::Max,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::V3PoolError, math::Tick, FeeTier, PoolResult};
    use ethers::types::Address;
    use rug::Float;
    use std::convert::Infallible;

    pub(crate) struct MockPool {
        pub token0: Address,
        pub token1: Address,
        pub token0_decimals: u8,
        pub token1_decimals: u8,
        pub fee: FeeTier,
    }

    #[async_trait::async_trait]
    impl crate::V3Pool for MockPool {
        type BackendError = Infallible;
        type Ticks<'a> = std::future::Ready<PoolResult<Vec<i128>, Self::BackendError>>;

        fn token0(&self) -> &Address {
            &self.token0
        }

        fn token0_decimals(&self) -> &u8 {
            &self.token0_decimals
        }

        fn token1(&self) -> &Address {
            &self.token1
        }

        fn token1_decimals(&self) -> &u8 {
            &self.token1_decimals
        }

        fn fee(&self) -> &FeeTier {
            &self.fee
        }

        fn address(&self) -> Address {
            Address::zero()
        }

        async fn current_liquidity(&self) -> PoolResult<Float, Self::BackendError> {
            Ok(Float::with_val(100, 100))
        }

        async fn sqrt_price_x96(&self) -> PoolResult<Float, Self::BackendError> {
            Ok(Float::with_val(100, 100))
        }

        async fn tick(&self, _tick: Tick) -> PoolResult<Float, Self::BackendError> {
            Ok(Float::with_val(100, 100))
        }

        fn tick_range(
            &self,
            _starting: Tick,
            _ending: Tick,
        ) -> Self::Ticks<'_> {
            std::future::ready(Ok(vec![]))
        }
    }
}
