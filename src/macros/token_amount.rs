macro_rules! impl_token_amount_cmp_eq_inner {
    ($token_amount:ident, $($native:ty),* $(,)?) => {
        $(
            impl<'a, P: V3Pool> PartialEq<$native> for $token_amount<'a, P> {
                fn eq(&self, other: &$native) -> bool {
                    let scaled = ::rug::Float::with_val(100, *other) * ::rug::Float::with_val(100, 10).pow(*self.decimals());

                    self.amount.eq(&scaled.floor())
                }
            }

            impl<'a, P: V3Pool> PartialOrd<$native> for $token_amount<'a, P> {
                fn partial_cmp(&self, other: &$native) -> Option<std::cmp::Ordering> {
                    let scaled = ::rug::Float::with_val(100, *other) * ::rug::Float::with_val(100, 10).pow(*self.decimals());

                    self.amount.partial_cmp(&scaled.floor())
                }
            }

            impl<'a, P: V3Pool> PartialEq<$token_amount<'a, P>> for $native {
                fn eq(&self, other: &$token_amount<'a, P>) -> bool {
                    let scaled = ::rug::Float::with_val(100, *self) * ::rug::Float::with_val(100, 10).pow(*other.decimals());

                    scaled.floor().eq(&other.amount)
                }
            }

            impl<'a, P: V3Pool> PartialOrd<$token_amount<'a, P>> for $native {
                fn partial_cmp(&self, other: &$token_amount<'a, P>) -> Option<std::cmp::Ordering> {
                    let scaled = ::rug::Float::with_val(100, *self) * ::rug::Float::with_val(100, 10).pow(*other.decimals());

                    scaled.floor().partial_cmp(&other.amount)
                }
            }

            impl<'a, P: V3Pool> IntoTokenAmount<'a, P> for $native {
                fn into_token_amount(self, pool: &'a P, token: TokenIdx) -> $token_amount<'a, P> {
                    let decimals = match token {
                        TokenIdx::Zero => pool.token0_decimals(),
                        TokenIdx::One => pool.token1_decimals(),
                    };

                    let scaled = rug::Float::with_val(100, self) * rug::Float::with_val(100, 10).pow(decimals);

                    $token_amount::from_scaled(pool, token, scaled.floor())
                }
            }
        )*
    }
}

/// Implement token amount comparison and equality for native types
///
/// Native types will be scaled to match the token amount's decimals
macro_rules! impl_token_amount_cmp_eq_native {
    ($name:ident) => {
        impl_token_amount_cmp_eq_inner! {
            $name,
            usize,
            u8,
            u16,
            u32,
            u64,
            f32,
            f64,
            i8,
            i16,
            i32,
            i64,
        }
    };
}
