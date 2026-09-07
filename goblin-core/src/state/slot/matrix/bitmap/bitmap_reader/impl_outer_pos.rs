use crate::{
    axis::LegMatcher,
    axis_helpers::TokenPair,
    quantities::{OUTER_POS, POS_0, POS_1, Pos0, Pos1, Position},
    state::{
        MarketPreimage, Preimage, SlotKey,
        bitmap::{Bitmap, BitmapPreimage, BitmapReader},
    },
};
use core::range::RangeInclusive;

impl BitmapReader<POS_1> for Bitmap<POS_0, OUTER_POS> {
    fn active_iterator<TP: TokenPair, In: LegMatcher>(
        market_key: SlotKey<MarketPreimage<TP>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Pos1> {
        In::outer_bitmap_index_iter(range)
            .filter_map(move |outer_bitmap_index| {
                let pos_0 = Pos0::new(outer_bitmap_index);

                let preimage = BitmapPreimage::<TP, POS_0, OUTER_POS> {
                    market_key,
                    safe_position: pos_0,
                };
                let outer_bitmap = preimage.hash().load();

                outer_bitmap.is_active().then(|| {
                    In::outer_pos_iter(range, pos_0.into())
                        .filter(move |outer_pos| outer_bitmap.index_active(*outer_pos))
                        .map(move |outer_pos| Pos1::new(pos_0, outer_pos))
                })
            })
            .flatten()
    }
}
