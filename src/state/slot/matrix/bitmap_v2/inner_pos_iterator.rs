use core::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{InnerPosV2, OuterPosV2, INNER_POS_V2},
    state::{
        bitmap_v2::{
            ordered_index::OrderedIndex, outer_index::OuterIndex, preimage::BitmapPreimageV2,
        },
        MarketPreimage, Preimage, SlotKey,
    },
};

impl InnerPosV2 {
    pub fn get_iter<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(OuterIndex<Self>, Self)>,
    ) -> impl Iterator<Item = (OuterIndex<Self>, impl Iterator<Item = Self>)>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
        Self: Clone + Copy,
    {
        let mapped_start = (((), range.start().0 .0), range.start().0 .1);
        let mapped_end = (((), range.end().0 .0), range.end().0 .1);

        OuterPosV2::get_iter::<M, B, Q, In>(market_key, mapped_start..=mapped_end).flat_map(
            move |((_, outer_bitmap_index), outer_pos_iter)| {
                let range = range.clone();
                outer_pos_iter.flat_map(move |outer_pos| {
                    let outer_index = (outer_bitmap_index, outer_pos);

                    let preimage = BitmapPreimageV2::<M, B, Q, INNER_POS_V2> {
                        market_key,
                        outer_index,
                    };

                    let hash = preimage.hash();

                    let inner_bitmap = hash.load();

                    let inner_pos_range = Self::clamped_range::<In>(&range, outer_index);

                    let inner_pos_iterator =
                        InnerPosV2::linear_iterator::<In>(inner_pos_range.clone())
                            .filter(move |inner_pos| inner_bitmap.index_active(*inner_pos));

                    Some((outer_index, inner_pos_iterator))
                })
            },
        )
    }
}
