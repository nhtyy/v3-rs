use rug::ops::Pow;
use rug::Float;

use crate::TickSpacing;

use super::wrappers::{Price, Tick};

lazy_static::lazy_static! {
    pub static ref TICK_BASE: Float = Float::with_val(100, 1.0001);

    pub static ref LN_TICK_BASE: Float = TICK_BASE.clone().ln();
}

/// the tick corresponding to this price
///
/// # Caution
/// this tick is not guaranteed to be initializable
/// use [price_to_initializable_tick] if you need an initializable tick
pub fn price_to_tick(price: Price) -> Tick {
    // saftey: Price is assumed to be in range to produce a valid tick
    unsafe {
        Tick::new_unchecked(
            // change of base log[1.0001](price)
            (price.into_inner().ln() / &*LN_TICK_BASE)
                .to_i32_saturating_round(rug::float::Round::Down)
                .expect("Can create a valid i32 from a valid price"),
        )
    }
}

/// the *initializable lower* tick corresponding to this price
pub fn price_to_initializable_tick(price: Price, tick_spacing: crate::I24) -> Tick {
    let spacing: i32 = tick_spacing.unchecked_into();

    // change of base log[1.0001](price)
    let tick = price.into_inner().ln() / &*LN_TICK_BASE;
    // our tick must be of the form spacing * n where n is an integer
    let spacing_scalar = tick / spacing;

    // get the interger part of n and turn it back into the tick
    // saftey: price is assumed to be in range to produce a valid tick
    unsafe {
        Tick::new_unchecked(
            (spacing_scalar.floor() * spacing)
                .to_i32_saturating_round(rug::float::Round::Down)
                .expect("Can create a valid i32 from a valid price"),
        )
    }
}

/// 1.0001^tick
pub fn tick_to_price(tick: Tick) -> Price {
    // saftey: tick is assumed to be in range to produce a valid price
    unsafe { Price::new_unchecked(TICK_BASE.clone().pow(*tick)) }
}
