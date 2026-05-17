use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{
        OuterBitmapIndex, Pos2, Position, PositionRange, SafePosition, OUTER_BITMAP_INDEX,
        OUTER_POS, POS_0, POS_1,
    },
    state::{
        bitmap::{bitmap_reader::BitmapReader, preimage::BitmapPreimage, Bitmap},
        MarketPreimage, Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

// TODO this should return Pos_1
impl BitmapReader<POS_1> for Bitmap<POS_0, OUTER_POS> {
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Pos2>,
    ) -> impl Iterator<Item = SafePosition<POS_1>>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let casted_range = range.cast_range::<u64, OUTER_BITMAP_INDEX>();

        In::outer_bitmap_index_iter(casted_range.clone())
            .filter_map(move |outer_bitmap_index| {
                let safe_position = SafePosition::<POS_0>::new(outer_bitmap_index);

                let preimage = BitmapPreimage::<M, B, Q, POS_0, OUTER_POS> {
                    market_key,
                    safe_position,
                };
                let outer_bitmap = preimage.hash().load();

                outer_bitmap.is_active().then(|| {
                    // TODO clamp range
                    let clamped_range = range
                        .clamp_range(outer_bitmap_index.into())
                        .cast_range::<u8, OUTER_POS>();

                    // In::outer_pos_iter(range.clone(), outer_bitmap_index)
                    In::outer_pos_iter(clamped_range)
                        .filter(move |outer_pos| outer_bitmap.index_active(*outer_pos))
                        .map(move |outer_pos| SafePosition::<POS_1>::new(safe_position, outer_pos))
                })
            })
            .flatten()
    }
}
