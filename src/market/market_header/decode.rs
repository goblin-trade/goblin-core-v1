use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    market::MarketHeader,
    types::Tuple,
};

impl Decodable for MarketHeader {
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        let byte_0 = args.decode::<u8>(offset, len)?;

        let decode_deposit_amounts = (byte_0 & 0b0000_0001) != 0;

        let execute_takes = Tuple::new(
            // base
            (byte_0 & 0b0000_0010) != 0,
            // quote
            (byte_0 & 0b0000_0100) != 0,
        );

        // 2 bits- max value 3
        // Too less, decipher one more byte
        // This field is currently unused. Increase the amount if needed by reading a new byte.
        let outer_bitmap_indices = (byte_0 & 0b0001_1000) >> 3;

        Ok(Self {
            decode_deposit_amounts,
            execute_takes,
            outer_bitmap_indices,
        })
    }
}
