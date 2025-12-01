use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    types::Pair,
};

pub struct MarketHeader {
    /// The type of market, encoded in 3 bits
    pub market_type_raw: u8,

    /// Whether to read decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute base-in and quote-in take orders
    pub execute_takes: Pair<bool, bool>,

    /// Number of outer bitmap indices
    pub outer_bitmap_indices: u8,
}

impl MarketHeader {
    pub fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        let byte_0 = args.decode::<u8>(offset, len)?;

        let market_type_raw = byte_0 & 0b0000_0111;
        let decode_deposit_amounts = (byte_0 & 0b0000_1000) != 0;

        let execute_takes = Pair {
            base: (byte_0 & 0b0001_0000) != 0,
            quote: (byte_0 & 0b0010_0000) != 0,
        };

        // 2 bits- max value 3
        // Too less, decipher one more byte
        // This field is currently unused. Increase the amount if needed by reading a new byte.
        let outer_bitmap_indices = (byte_0 & 0b1100_0000) >> 6;

        Ok(Self {
            market_type_raw,
            decode_deposit_amounts,
            execute_takes,
            outer_bitmap_indices,
        })
    }
}
