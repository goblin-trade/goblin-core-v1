use crate::input_processor::BitPack;

#[derive(Clone, Copy)]
pub struct TakeFlags {
    pub read_min_lots: bool,
    pub read_limit: bool,
}

/// Two flags packed into the low two bits of the take header, LSB first.
impl BitPack for TakeFlags {
    const CAPACITY: u8 = 2;

    #[inline]
    fn to_raw(self) -> u64 {
        (self.read_min_lots as u64) | ((self.read_limit as u64) << 1)
    }

    #[inline]
    fn from_raw(raw: u64) -> Self {
        Self {
            read_min_lots: raw & 1 != 0,
            read_limit: raw & 0b10 != 0,
        }
    }
}
