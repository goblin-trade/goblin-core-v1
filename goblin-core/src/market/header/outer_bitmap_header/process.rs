use crate::{
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    market::header::{
        inner_bitmap_header::InnerBitmapHeader, outer_bitmap_header::OuterBitmapHeader,
    },
    quantities::{Pos0, OUTER_POS, POS_0},
    state::bitmap::Bitmap,
    Ctx,
};

impl OuterBitmapHeader {
    pub fn process<MS: MarketSpec>(
        reader: &ArgsReader,
        ctx: &mut Ctx<MS>,
    ) -> Result<(), GoblinError> {
        let Self {
            outer_bitmap_index,
            inner_bitmap_count,
        } = Self::try_fixed_decode(reader)?;

        let pos_0 = Pos0::new(outer_bitmap_index);

        let (outer_bitmap_key, mut outer_bitmap_state) =
            Bitmap::<POS_0, OUTER_POS>::conditional_read(pos_0, ctx);
        let outer_bitmap_clone = outer_bitmap_state;

        for _ in 0..inner_bitmap_count {
            InnerBitmapHeader::process(pos_0, &mut outer_bitmap_state, reader, ctx)?;
        }

        outer_bitmap_state.conditional_write(&outer_bitmap_clone, &outer_bitmap_key);

        Ok(())
    }
}
