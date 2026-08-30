use crate::input_processor::{
    global_args::global_header::MarketCountsV2, FixedDecode, VariableDecode,
};

impl<'a> VariableDecode<'a> for MarketCountsV2 {
    type Flags = bool;

    fn size(flags: &Self::Flags) -> usize {
        3 + 8 * usize::from(*flags)
    }

    fn raw_variable_decode(
        reader: &'a crate::input_processor::ArgsReader,
        flags: &Self::Flags,
    ) -> Self {
        let byte_0 = u8::raw_fixed_decode(reader);
        let byte_1 = u8::raw_fixed_decode(reader);

        // byte_1 has 4 free unused bits
        let hardcoded_count = [byte_0 & 0b0000_1111, byte_0 >> 4, byte_1 & 0b0000_1111];

        let custom_count = if *flags {
            let byte_2 = u8::raw_fixed_decode(reader);
            let byte_3 = u8::raw_fixed_decode(reader);
            let byte_4 = u8::raw_fixed_decode(reader);
            let byte_5 = u8::raw_fixed_decode(reader);

            [
                byte_2 & 0b0000_1111,
                byte_2 >> 4,
                byte_3 & 0b0000_1111,
                byte_3 >> 4,
                byte_4 & 0b0000_1111,
                byte_4 >> 4,
                byte_5 & 0b0000_1111,
                byte_5 >> 4,
            ]
        } else {
            [0u8; 8]
        };

        let mut counts = [0u8; 11];
        counts[..3].copy_from_slice(&hardcoded_count);
        counts[3..].copy_from_slice(&custom_count);

        Self { counts, index: 0 }
    }
}
