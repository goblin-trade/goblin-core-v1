use crate::{
    axis::leg::LegMatcher,
    axis_helpers::TokenPair,
    quantities::{INNER_POS, OUTER_POS, POS_0, POS_1, POS_2, Pos2, Position},
    state::{
        MarketPreimage, Preimage, SlotKey,
        bitmap::{Bitmap, BitmapPreimage, BitmapReader},
    },
};
use core::range::RangeInclusive;

impl BitmapReader<POS_2> for Bitmap<POS_1, INNER_POS> {
    /// Get an iterator of active positions
    ///
    /// # Range
    /// - start should be the lower bound. I.e. last_price in In=Quote and limit_price in In=Base
    fn active_iterator<TP: TokenPair, In: LegMatcher>(
        market_key: SlotKey<MarketPreimage<TP>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Pos2> {
        // outer iterator ignores inner bits in range endpoints — no complement needed
        Bitmap::<POS_0, OUTER_POS>::active_iterator::<TP, In>(market_key, range).flat_map(
            move |pos_1| {
                let preimage = BitmapPreimage::<TP, POS_1, INNER_POS> {
                    market_key,
                    safe_position: pos_1,
                };
                let inner_bitmap = preimage.hash().load();

                In::inner_pos_iter(range, pos_1.into())
                    .filter(move |inner_pos| inner_bitmap.index_active(*inner_pos))
                    .map(move |inner_pos| Pos2::new(pos_1, inner_pos))
            },
        )
    }
}
