use rug::ops::Pow;
use rug::Float;

use crate::TickSpacing;

use super::wrappers::{Price, Tick};

lazy_static::lazy_static! {
    pub static ref TICK_BASE: Float = Float::with_val(100, 1.0001);
}

/// the tick corresponding to this price
pub fn price_to_tick(price: Price) -> Tick {
    // saftey: Price is assumed to be in range to produce a valid tick
    unsafe {
        Tick::new_unchecked(
            // change of base log[1.0001](price)
            (price.into_inner().ln() / TICK_BASE.clone().ln())
                .to_i32_saturating_round(rug::float::Round::Down)
                .expect("Can create a valid i32 from a valid float"),
        )
    }
}

/// the *initializable lower* tick corresponding to this price
pub fn price_to_initializable_tick(price: Price, tick_spacing: TickSpacing) -> Tick {
    let spacing = tick_spacing as u8;

    // change of base log[1.0001](price)
    let tick = price.into_inner().ln() / TICK_BASE.clone().ln();
    // our tick must be of the form spacing * n where n is an integer
    let spacing_scalar = tick / spacing;

    // get the interger part of n and turn it back into the tick
    // round down cause the evm i guess?
    // saftey: price is assumed to be in range to produce a valid tick
    unsafe {
        Tick::new_unchecked(
            (spacing_scalar.floor() * spacing)
                .to_i32_saturating_round(rug::float::Round::Down)
                .expect("Can create a valid i32 from a valid float"),
        )
    }
}

/// 1.0001^tick
pub fn tick_to_price(tick: Tick) -> Price {
    TICK_BASE.clone().pow(*tick).into()
}
