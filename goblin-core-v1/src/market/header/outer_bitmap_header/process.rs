use crate::{
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    market::{
        header::{inner_bitmap_header::InnerBitmapHeader, outer_bitmap_header::OuterBitmapHeader},
        Readables, Writables,
    },
    quantities::{SafePosition, OUTER_POS, POS_0},
    state::bitmap::Bitmap,
};

impl OuterBitmapHeader {
    pub fn process<MS: MarketSpec>(
        ctx: &DecodeCtx,
        readables: &Readables<MS>,
        writables: &mut Writables,
    ) -> Result<(), GoblinError> {
        let Self {
            outer_bitmap_index,
            inner_bitmap_count,
        } = Self::try_fixed_decode(ctx)?;

        let pos_0 = SafePosition::<POS_0>::new(outer_bitmap_index);

        let (outer_bitmap_key, mut outer_bitmap_state) =
            Bitmap::<POS_0, OUTER_POS>::conditional_read(
                readables.market_readables.market_key,
                &writables.market_state.last_positions,
                pos_0,
            );
        let outer_bitmap_clone = outer_bitmap_state;

        for _ in 0..inner_bitmap_count {
            InnerBitmapHeader::process(ctx, readables, pos_0, writables, &mut outer_bitmap_state)?;
        }

        outer_bitmap_state.conditional_write(&outer_bitmap_clone, &outer_bitmap_key);

        Ok(())
    }
}
