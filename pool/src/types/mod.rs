pub mod deltas;
pub mod price;
pub mod amount;

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