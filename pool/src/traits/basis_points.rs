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

impl<T> ApplyBps for T where
    T: std::ops::Mul<u16, Output = T>
        + std::ops::Div<u16, Output = T>
        + std::ops::Mul<u16, Output = T>
        + Clone
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