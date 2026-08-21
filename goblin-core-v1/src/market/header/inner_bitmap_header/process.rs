use crate::{
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    instructions::ix_make::ix_make,
    market::InnerBitmapHeader,
    quantities::{SafePosition, INNER_POS, POS_0, POS_1},
    state::bitmap::{alias::OuterBitmap, Bitmap},
    Ctx,
};

impl InnerBitmapHeader {
    pub fn process<MS: MarketSpec>(
        pos_0: SafePosition<POS_0>,
        outer_bitmap_state: &mut OuterBitmap,
        reader: &ArgsReader,
        ctx: &mut Ctx<MS>,
    ) -> Result<(), GoblinError> {
        let Self {
            outer_pos,
            update_count,
        } = Self::try_fixed_decode(reader)?;

        let pos_1 = SafePosition::<POS_1>::new(pos_0, outer_pos);

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
