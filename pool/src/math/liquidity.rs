use rug::Float;

use super::wrappers::SqrtPrice;

/// How much token0 is in the pool reserves given a price range and liquidity
pub fn real_token0_from_l(sqrt_current: SqrtPrice, sqrt_upper: SqrtPrice, l: Float) -> Float {
    let inverse_current = sqrt_current.into_inner().recip();
    let inverse_upper = sqrt_upper.into_inner().recip();

    let real: Float = l * (inverse_current - inverse_upper);

    if real.is_sign_negative() {
        // return 0
        Float::with_val(64, 0)
    } else {
        real.floor()
    }
}

/// How much token1 is in the pool reserves given a price range and liquidity
pub fn real_token1_from_l(sqrt_currnet: SqrtPrice, sqrt_lower: SqrtPrice, l: Float) -> Float {
    let real = l * (sqrt_currnet - sqrt_lower);

    if real.is_sign_negative() {
        // return 0
        Float::with_val(64, 0)
    } else {
        real.floor()
    }
}

/// the `L` value of the pool given the amount of token0
pub fn liqudity_from_real_token1(
    sqrt_current: SqrtPrice,
    sqrt_lower: SqrtPrice,
    token1: Float,
) -> Float {
    let l = token1 / (sqrt_current - sqrt_lower);

    l.floor()
}

/// the `L` value of the pool given the amount of token1
pub fn liqudity_from_real_token0(
    sqrt_current: SqrtPrice,
    sqrt_upper: SqrtPrice,
    token0: Float,
) -> Float {
    let inverse_current = sqrt_current.into_inner().recip();
    let inverse_upper = sqrt_upper.into_inner().recip();

    let l = token0 / (inverse_current - inverse_upper);

    l.floor()
}
