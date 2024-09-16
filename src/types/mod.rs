use alloy::primitives::{uint, Uint};

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
    /// Added after by governance = 1 bps
    Lowest,
    Min,
    Mid,
    Max,
}

impl FeeTier {
    /// Big endian representation of the fee tier
    /// used for encoding paths
    pub const fn as_u24_bytes(&self) -> [u8; 3] {
        self.as_scaled_bp().to_be_bytes()
    }

    pub const fn as_spacing(&self) -> TickSpacing {
        match self {
            FeeTier::Lowest => TickSpacing::Lowest,
            FeeTier::Min => TickSpacing::Min,
            FeeTier::Mid => TickSpacing::Mid,
            FeeTier::Max => TickSpacing::Max,
        }
    }

    pub const fn as_bp(&self) -> u16 {
        match self {
            FeeTier::Lowest => 1,
            FeeTier::Min => 5,
            FeeTier::Mid => 30,
            FeeTier::Max => 100,
        }
    }

    pub const fn as_scaled_bp(&self) -> Uint<24, 1> {
        match self {
            FeeTier::Lowest => uint!(100_U24),
            FeeTier::Min => uint!(500_U24),
            FeeTier::Mid => uint!(3000_U24),
            FeeTier::Max => uint!(10000_U24),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TickSpacing {
    /// Added after by governance = 1 bps
    Lowest = 1,
    Min = 10,
    Mid = 60,
    Max = 200,
}

impl TickSpacing {
    pub const fn as_fee(tick_spacing: TickSpacing) -> FeeTier {
        match tick_spacing {
            TickSpacing::Lowest => FeeTier::Lowest,
            TickSpacing::Min => FeeTier::Min,
            TickSpacing::Mid => FeeTier::Mid,
            TickSpacing::Max => FeeTier::Max,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::Tick, FeeTier, PoolResult};
    use alloy::primitives::Address;
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
            Address::ZERO
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

        async fn tick_range(
            &self,
            _starting: Tick,
            _ending: Tick,
        ) -> PoolResult<Vec<i128>, Self::BackendError> {
            Ok(vec![])
        }
    }
}
