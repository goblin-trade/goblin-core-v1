use crate::{
    axis::{leg::leg_matcher::LegMatcher, market::market_spec::MarketSpec},
    quantities::{Pos2, Position, SafePosition, INNER_POS, OUTER_POS, POS_0, POS_1, POS_2},
    state::{
        bitmap::{bitmap_reader::BitmapReader, preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

impl BitmapReader<POS_2> for Bitmap<POS_1, INNER_POS> {
    /// Get an iterator of active positions
    ///
    /// # Range
    /// - start() should be the lower bound. I.e. last_price in In=Quote and limit_price in In=Base
    fn active_iterator<MS: MarketSpec, In: LegMatcher>(
        market_key: SlotKey<MarketPreimage<MS>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = SafePosition<POS_2>> {
        // outer iterator ignores inner bits in range endpoints — no complement needed
        Bitmap::<POS_0, OUTER_POS>::active_iterator::<MS, In>(market_key, range.clone()).flat_map(
            move |pos_1| {
                let preimage = BitmapPreimage::<MS, POS_1, INNER_POS> {
                    market_key,
                    safe_position: pos_1,
                };
                let inner_bitmap = preimage.hash().load();

                In::inner_pos_iter(range.clone(), pos_1.into())
                    .filter(move |inner_pos| inner_bitmap.index_active((*inner_pos).into()))
                    .map(move |inner_pos| Pos2::new(pos_1, inner_pos))
            },
        )
    }
}
