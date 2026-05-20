use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Position, SafePosition, OUTER_POS, POS_0, POS_1},
    state::{
        bitmap::{bitmap_reader::BitmapReader, preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

impl BitmapReader<POS_1> for Bitmap<POS_0, OUTER_POS> {
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = SafePosition<POS_1>>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        In::outer_bitmap_index_iter(range.clone())
            .filter_map(move |outer_bitmap_index| {
                let pos_0 = SafePosition::<POS_0>::new(outer_bitmap_index);

                let preimage = BitmapPreimage::<M, B, Q, POS_0, OUTER_POS> {
                    market_key,
                    safe_position: pos_0,
                };
                let outer_bitmap = preimage.hash().load();

                outer_bitmap.is_active().then(|| {
                    In::outer_pos_iter(range.clone(), pos_0.into())
                        .filter(move |outer_pos| outer_bitmap.index_active(*outer_pos))
                        .map(move |outer_pos| SafePosition::<POS_1>::new(pos_0, outer_pos))
                })
            })
            .flatten()
    }
}
