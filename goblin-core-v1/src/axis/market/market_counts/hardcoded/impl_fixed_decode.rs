use crate::{
    axis::market::market_counts::hardcoded::HardcodedCounts,
    input_processor::{ArgsReader, FixedDecode},
};

impl<'a> FixedDecode<'a> for HardcodedCounts {
    const ENCODED_SIZE: usize = 2;

    fn raw_fixed_decode(reader: &'a ArgsReader) -> Self {
        let byte_0 = u8::raw_fixed_decode(reader);
        let byte_1 = u8::raw_fixed_decode(reader);
        reader.advance_offset(Self::ENCODED_SIZE);

        Self::new([byte_0 & 0b0000_1111, byte_0 >> 4, byte_1 & 0b0000_1111])
    }
}
