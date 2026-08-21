use crate::{
    axis_helpers::MarketSpec,
    matching::region::make_region::MakeRegion,
    quantities::{SafePosition, OUTER_POS, POS_0},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        Preimage, SlotKey,
    },
    Ctx,
};

impl Bitmap<POS_0, OUTER_POS> {
    pub fn conditional_read<MS: MarketSpec>(
        safe_position: SafePosition<POS_0>,
        ctx: &Ctx<MS>,
    ) -> (SlotKey<BitmapPreimage<MS, POS_0, OUTER_POS>>, Self) {
        let key = BitmapPreimage::<MS, POS_0, OUTER_POS> {
            market_key: ctx.readables.market_readables().market_key,
            safe_position,
        }
        .hash();

        let region = MakeRegion::new(
            &ctx.writables.market_state.last_positions,
            safe_position.into(),
        );

        let bitmap = if region == MakeRegion::Spread {
            Bitmap::<POS_0, OUTER_POS>::default()
        } else {
            let outer_bitmap = key.load();
            if outer_bitmap.is_closed() {
                Bitmap::<POS_0, OUTER_POS>::default()
            } else {
                outer_bitmap
            }
        };

        (key, bitmap)
    }
}
