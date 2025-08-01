use crate::hostio::hostio_helpers;

#[repr(C, packed)]
pub struct OrderExpiry(u64);

impl OrderExpiry {
    pub fn valid(&self) -> bool {
        !self.used()
    }

    // Bit 0: Whether expiry is used
    fn used(&self) -> bool {
        self.0 & 0b1 != 0
    }

    // Bit 1: Whether expiry is based on block number of block time
    fn is_block_number(&self) -> bool {
        self.0 & 0b10 != 0
    }

    /// Bits 2..63: Expiry value
    fn value(&self) -> u64 {
        self.0 >> 2
    }
}
