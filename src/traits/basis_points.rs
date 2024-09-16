pub trait ApplyBps {
    /// Scale value by 1 + bps / 10_000
    ///
    /// # Panics
    /// If bps > 10_000
    fn apply_bps_up(&self, bps: u16) -> Self;

    /// Scale value by 1 - bps / 10_000
    ///
    /// # Panics
    /// If bps > 10_000
    fn apply_bps_down(&self, bps: u16) -> Self;
}

impl<T> ApplyBps for T
where
    T: std::ops::Mul<u16, Output = T>
        + std::ops::Div<u16, Output = T>
        + std::ops::Mul<u16, Output = T>
        + Clone,
{
    fn apply_bps_down(&self, bps: u16) -> Self {
        assert!(bps <= 10_000, "bps must be <= 10_000");

        self.clone() * (10_000 - bps) / 10_000
    }

    fn apply_bps_up(&self, bps: u16) -> Self {
        assert!(bps <= 10_000, "bps must be <= 10_000");

        self.clone() * (10_000 + bps) / 10_000
    }
}

#[cfg(test)]
mod test {
    use super::ApplyBps;
    use rug::Float;

    #[test]
    fn test_apply_bps_up_float() {
        let value = Float::with_val(100, 100);
        // 100 bps = 1%
        let result = value.apply_bps_up(100);

        assert_eq!(result, Float::with_val(100, 101));
    }

    #[test]
    fn test_apply_bps_down_float() {
        let value = Float::with_val(100, 100);
        // 100 bps = 1%
        let result = value.apply_bps_down(100);

        assert_eq!(result, Float::with_val(100, 99));
    }
}
