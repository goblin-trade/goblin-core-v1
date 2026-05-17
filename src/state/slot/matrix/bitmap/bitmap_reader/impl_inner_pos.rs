use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Pos2, Position, INNER_POS, OUTER_POS, POS_0, POS_1},
    state::{
        bitmap::{bitmap_reader::BitmapReader, preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

impl BitmapReader for Bitmap<POS_1, INNER_POS> {
    /// Get an iterator of active positions
    ///
    /// # Range
    /// - start() should be the lower bound. I.e. last_price in In=Quote and limit_price in In=Base
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Pos2>,
    ) -> impl Iterator<Item = Pos2>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        // outer iterator ignores inner bits in range endpoints — no complement needed
        Bitmap::<POS_0, OUTER_POS>::active_iterator::<M, B, Q, In>(market_key, range.clone())
            .flat_map(move |pos_1| {
                let preimage = BitmapPreimage::<M, B, Q, POS_1, INNER_POS> {
                    market_key,
                    safe_position: pos_1,
                };
                let inner_bitmap = preimage.hash().load();

                In::inner_pos_iter(range.clone(), pos_1)
                    .filter(move |inner_pos| inner_bitmap.index_active((*inner_pos).into()))
                    .map(move |inner_pos| pos_1 + inner_pos)
            })
    }
}
