use crate::markets::MarketIndex;

#[repr(C)]
pub struct MarketInstructions {
    pub market_index: MarketIndex,
    pub instructions_byte: u8,
}

impl MarketInstructions {
    pub fn take_bid(&self) -> bool {
        self.instructions_byte & 0b0000_0001 == 1
    }

    pub fn take_ask(&self) -> bool {
        self.instructions_byte & 0b0000_0010 == 1
    }

    pub fn outer_bitmap_indices(&self) -> u8 {
        self.instructions_byte >> 2
    }
}
