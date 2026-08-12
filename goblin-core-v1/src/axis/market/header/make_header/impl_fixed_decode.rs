use crate::{
    axis::{market::header::make_header::MakeHeader, occupancy::OccupancyEnum},
    input_processor::{DecodeCtx, FixedDecode},
    quantities::{BaseLots, InnerPos},
};

impl<'a> FixedDecode<'a> for MakeHeader {
    const ENCODED_SIZE: usize = 1 + 8;

    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self {
        let inner_pos = InnerPos::new(u8::raw_fixed_decode(ctx));
        let bytes = u64::raw_fixed_decode(ctx);

        let occupancy_enum = OccupancyEnum::from((bytes & 0b01) == 1);
        let inner_enum_raw = (bytes & 0b10) == 1;

        let base_lots = BaseLots::new(bytes >> 2);

        Self {
            inner_pos,
            occupancy_enum,
            inner_enum_raw,
            base_lots,
        }
    }
}
