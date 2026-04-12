use core::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{OuterBitmapIndexV2, OuterPosV2, OUTER_POS_V2},
    state::{
        bitmap_v2::{ordered_index::OrderedIndex, preimage::BitmapPreimageV2},
        MarketPreimage, Preimage, SlotKey,
    },
};

impl OuterPosV2 {
    pub fn get_iter<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(<Self as OrderedIndex>::Prev, Self)>,
    ) where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let (start, end) = range.into_inner();
        let outer_range = start.0..=end.0;

        OuterBitmapIndexV2::linear_iterator::<In>(outer_range).filter_map(
            move |outer_bitmap_index| {
                let preimage = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
                    market_key,
                    outer_index: ((), outer_bitmap_index),
                };
                let hash = preimage.hash();

                let outer_bitmap = hash.load();

                if !outer_bitmap.is_active() {
                    return None;
                }

                Some(())
            },
        );
    }
}
