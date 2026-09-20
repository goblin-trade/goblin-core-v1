#[derive(Clone, Copy)]
pub struct TakeFlags {
    pub read_min_lots: bool,
    pub read_limit: bool,
}

impl TakeFlags {
    /// Two flags packed into the low two bits of the take header, LSB first.
    #[inline]
    pub const fn to_raw(self) -> u64 {
        (self.read_min_lots as u64) | ((self.read_limit as u64) << 1)
    }

    #[inline]
    pub const fn from_raw(raw: u64) -> Self {
        Self {
            read_min_lots: raw & 1 != 0,
            read_limit: raw & 0b10 != 0,
        }
    }
}
