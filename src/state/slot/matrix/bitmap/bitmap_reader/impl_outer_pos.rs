use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Position, OUTER_POS, POS_0},
    state::{
        bitmap::{bitmap_reader::BitmapReader, preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

impl BitmapReader for Bitmap<POS_0, OUTER_POS> {
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
        In::outer_bitmap_index_iter(range.clone())
            .filter_map(move |position| {
                let preimage = BitmapPreimage::<M, B, Q, POS_0, OUTER_POS> {
                    market_key,
                    safe_position: position,
                };
                let outer_bitmap = preimage.hash().load();

                outer_bitmap.is_active().then(|| {
                    In::outer_pos_iter(range.clone(), position)
                        .filter(move |outer_pos| outer_bitmap.index_active((*outer_pos).into()))
                        .map(move |outer_pos| position + outer_pos)
                })
            })
            .flatten()
    }
}
