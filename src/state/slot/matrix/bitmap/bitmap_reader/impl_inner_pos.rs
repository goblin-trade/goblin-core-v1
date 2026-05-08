use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Position, INNER_POS, OUTER_POS},
    state::{
        bitmap::{bitmap_reader::BitmapReader, preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

impl BitmapReader for Bitmap<INNER_POS> {
    /// Get an iterator of active positions
    ///
    /// # Range
    /// - start() should be the lower bound. I.e. last_price in In=Quote and limit_price in In=Base
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        // outer iterator ignores inner bits in range endpoints — no complement needed
        Bitmap::<OUTER_POS>::active_iterator::<M, B, Q, In>(market_key, range.clone()).flat_map(
            move |position| {
                let preimage = BitmapPreimage::<M, B, Q, INNER_POS> {
                    market_key,
                    position,
                };
                let inner_bitmap = preimage.hash().load();

                In::inner_pos_iter(range.clone(), position)
                    .filter(move |inner_pos| inner_bitmap.index_active((*inner_pos).into()))
                    .map(move |inner_pos| position + inner_pos)
            },
        )
    }
}
