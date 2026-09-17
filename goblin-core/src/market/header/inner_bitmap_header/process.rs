use crate::{
    Ctx,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedCodec},
    instructions::ix_make::ix_make,
    market::InnerBitmapHeader,
    quantities::{INNER_POS, POS_1, Pos0, Pos1},
    state::bitmap::{Bitmap, alias::OuterBitmap},
};

impl InnerBitmapHeader {
    pub fn process<MS: MarketSpec>(
        pos_0: Pos0,
        outer_bitmap_state: &mut OuterBitmap,
        reader: &ArgsReader,
        ctx: &mut Ctx<MS>,
    ) -> Result<(), GoblinError> {
        let Self {
            outer_pos,
            update_count,
        } = Self::try_fixed_decode(reader)?;

        let pos_1 = Pos1::new(pos_0, outer_pos);

        let (inner_bitmap_key, mut inner_bitmap_state) =
            Bitmap::<POS_1, INNER_POS>::conditional_read(pos_1, outer_pos, outer_bitmap_state, ctx);
        let inner_bitmap_clone = inner_bitmap_state;

        for _ in 0..update_count {
            ix_make(pos_1, reader, &mut inner_bitmap_state, ctx)?;
        }

        inner_bitmap_state.conditional_write(
            &inner_bitmap_clone,
            &inner_bitmap_key,
            outer_pos,
            outer_bitmap_state,
        );

        Ok(())
    }
}
