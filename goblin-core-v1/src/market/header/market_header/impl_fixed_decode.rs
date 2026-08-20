use super::MarketHeader;
use crate::{
    axis_helpers::MarketSpec,
    input_processor::{ArgsReader, FixedDecode},
    types::Tuple,
};

impl<'a, MS: MarketSpec> FixedDecode<'a> for MarketHeader<MS> {
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let byte_0 = u8::raw_fixed_decode(reader);

        let decode_deposit_amounts = (byte_0 & 0b0000_0001) != 0;

        let execute_takes = Tuple::new(
            // base
            (byte_0 & 0b0000_0010) != 0,
            // quote
            (byte_0 & 0b0000_0100) != 0,
        );

        // 2 bits- max value 3
        // This field is currently unused. Increase the amount if needed by reading a new byte.
        let outer_bitmap_indices = (byte_0 & 0b0001_1000) >> 3;

        Self::new(decode_deposit_amounts, execute_takes, outer_bitmap_indices)
    }
}
