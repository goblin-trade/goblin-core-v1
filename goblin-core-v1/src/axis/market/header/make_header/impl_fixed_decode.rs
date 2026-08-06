use crate::{
    axis::market::header::make_header::MakeHeader,
    input_processor::{DecodeCtx, FixedDecode},
    instructions::make_variant::MakeVariant,
    quantities::{BaseLots, InnerPos},
};

impl<'a> FixedDecode<'a> for MakeHeader {
    const ENCODED_SIZE: usize = 1 + 8;

    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self {
        let inner_pos = InnerPos::new(u8::raw_fixed_decode(ctx));
        let bytes = u64::raw_fixed_decode(ctx);

        let make_variant = MakeVariant::from(bytes);
        let base_lots = BaseLots::new(bytes >> 2);

        Self {
            inner_pos,
            base_lots,
            make_variant,
        }
    }
}
