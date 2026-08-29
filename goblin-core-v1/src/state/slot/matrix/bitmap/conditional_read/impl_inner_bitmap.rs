use crate::{
    axis_helpers::MarketSpec,
    matching::region::make_region::MakeRegion,
    quantities::{OuterPos, Pos1, INNER_POS, OUTER_POS, POS_0, POS_1},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        Preimage, SlotKey,
    },
    Ctx,
};

impl Bitmap<POS_1, INNER_POS> {
    pub fn conditional_read<MS: MarketSpec>(
        safe_position: Pos1,
        outer_pos: OuterPos,
        outer_bitmap_state: &Bitmap<POS_0, OUTER_POS>,
        ctx: &Ctx<MS>,
    ) -> (SlotKey<BitmapPreimage<MS::Pair, POS_1, INNER_POS>>, Self) {
        let key = BitmapPreimage {
            market_key: ctx.readables.market_readables().market_key,
            safe_position,
        }
        .hash();

        let region = MakeRegion::new(
            &ctx.writables.market_state.last_positions,
            safe_position.into(),
        );

        let bitmap = if region == MakeRegion::Spread || !outer_bitmap_state.index_active(outer_pos)
        {
            Bitmap::<POS_1, INNER_POS>::default()
        } else {
            key.load()
        };

        (key, bitmap)
    }
}
