use crate::{
    axis::market::market_counts::dynamic::DynamicCounts,
    input_processor::{ArgsReader, FixedDecode},
};

impl<'a> FixedDecode<'a> for DynamicCounts {
    const ENCODED_SIZE: usize = 4;

    fn raw_fixed_decode(reader: &'a ArgsReader) -> Self {
        let byte_0 = u8::raw_fixed_decode(reader);
        let byte_1 = u8::raw_fixed_decode(reader);
        let byte_2 = u8::raw_fixed_decode(reader);
        let byte_3 = u8::raw_fixed_decode(reader);

        let market_counts = [
            byte_0 & 0b0000_1111,
            byte_0 >> 4,
            byte_1 & 0b0000_1111,
            byte_1 >> 4,
            byte_2 & 0b0000_1111,
            byte_2 >> 4,
            byte_3 & 0b0000_1111,
            byte_3 >> 4,
        ];

        reader.advance_offset(Self::ENCODED_SIZE);

        Self {
            inner: market_counts,
        }
    }
}
