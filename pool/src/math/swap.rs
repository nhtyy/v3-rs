use rug::Float;

use super::wrappers::SqrtPrice;

/// How much token0 reserves will change given a change in price and a fixed amount of liquidity
pub fn token0_delta(sqrt_price: SqrtPrice, target_price: SqrtPrice, liquidity: Float) -> Float {
    let invert_target = target_price.into_inner().recip();
    let invert_price = sqrt_price.into_inner().recip();

    liquidity * (invert_target - invert_price)
}

/// How much token1 reserves will change given a change in price and a fixed amount of liquidity
pub fn token1_delta(sqrt_price: SqrtPrice, target_price: SqrtPrice , liquidity: Float) -> Float {
    liquidity * (target_price - sqrt_price)
}