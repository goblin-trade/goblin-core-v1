use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::MarketIndex,
    types::Pair,
};

pub struct MarketHeader {
    /// The type of market, encoded in 3 bits
    pub market_type_raw: u8,

    /// Whether to execute base-in and quote-in take orders
    pub execute_takes: Pair<bool, bool>,

    /// Number of outer bitmap indices
    pub outer_bitmap_indices: u8,
}

impl MarketHeader {
    pub fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self, GoblinError> {
        let byte_0 = payload.decode::<u8>(offset, len)?;

        let market_type_raw = byte_0 & 0b0000_0111;

        let execute_takes = Pair {
            base: (byte_0 & 0b0000_1000) != 0,
            quote: (byte_0 & 0b0001_0000) != 0,
        };

        // 3 bits- max value 7
        // This field is currently unused. Increase the amount if needed by reading a new byte.
        let outer_bitmap_indices = (byte_0 & 0b1110_0000) >> 5;

        Ok(Self {
            market_type_raw,
            execute_takes,
            outer_bitmap_indices,
        })
    }
}

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
