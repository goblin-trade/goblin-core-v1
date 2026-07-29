use crate::{
    axis::{leg::SamePair, market::market_spec::MarketSpec},
    matching::region::make_region::MakeRegion,
    quantities::{Position, SafePosition, OUTER_POS, POS_0},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl Bitmap<POS_0, OUTER_POS> {
    pub fn conditional_read<MS: MarketSpec>(
        market_key: SlotKey<MarketPreimage<MS>>,
        last_positions: &SamePair<Position>,
        safe_position: SafePosition<POS_0>,
    ) -> (SlotKey<BitmapPreimage<MS, POS_0, OUTER_POS>>, Self) {
        let key = BitmapPreimage::<MS, POS_0, OUTER_POS> {
            market_key,
            safe_position,
        }
        .hash();

        let region = MakeRegion::new(last_positions, safe_position.into());

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
