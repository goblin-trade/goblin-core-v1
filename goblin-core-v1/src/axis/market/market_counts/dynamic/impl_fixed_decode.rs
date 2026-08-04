use crate::{
    axis::market::market_counts::dynamic::DynamicCounts,
    input_processor::{DecodeCtx, FixedDecode},
};

impl<'a> FixedDecode<'a> for DynamicCounts {
    const ENCODED_SIZE: usize = 4;

    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self {
        let byte_0 = u8::raw_fixed_decode(ctx);
        let byte_1 = u8::raw_fixed_decode(ctx);
        let byte_2 = u8::raw_fixed_decode(ctx);
        let byte_3 = u8::raw_fixed_decode(ctx);

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

        ctx.advance_offset(Self::ENCODED_SIZE);

        Self { market_counts }
    }
}
