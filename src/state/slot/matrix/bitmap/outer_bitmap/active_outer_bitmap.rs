use crate::{
    axis::{
        leg::{leg_iterator::LegIterator, leg_matcher::LegMatcher, Base, Quote, SamePair},
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, Coordinate},
    state::{
        bitmap::{
            outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
            Bitmap,
        },
        SlotKey,
    },
    types::StoreReader,
};

#[repr(C)]
#[derive(PartialEq, Default)]
pub struct ActiveOuterBitmap {
    pub inner: [u8; 32],
}

impl ActiveOuterBitmap {
    pub const fn new(inner: [u8; 32]) -> Self {
        Self { inner }
    }

    fn clean_in_range(
        &mut self,
        outer_bitmap_index: OuterBitmapIndex,
        outer_bitmap_index_pair: &SamePair<OuterBitmapIndex>,
        outer_pos_pair: &SamePair<OuterPos>,
    ) {
        let on_last_base = outer_bitmap_index == Base::get(outer_bitmap_index_pair);
        let on_last_quote = outer_bitmap_index == Quote::get(outer_bitmap_index_pair);

        let base_next = Base::get(outer_pos_pair).next();
        let quote_prev = Quote::get(outer_pos_pair).prev();

        let range_option = match (on_last_base, on_last_quote) {
            (true, true) => match (base_next, quote_prev) {
                (Some(start), Some(end)) => Some(start..=end),
                _ => None, // unreachable branch. If both values are guaranteed to be present.
            },
            (true, false) => base_next.map(|start| start..=OuterPos::MAX),
            (false, true) => quote_prev.map(|end| OuterPos::MIN..=end),
            (false, false) => None,
        };

        if let Some(range_inclusive) = range_option {
            let iter = Quote::outer_pos_iter(range_inclusive);
            for pos in iter {
                self.deactivate(pos);
            }
        }
    }

    pub fn new_cleaned<M, B, Q>(
        key: &SlotKey<OuterBitmapPreimage<M, B, Q>>,
        outer_bitmap_index: OuterBitmapIndex,
        outer_bitmap_index_pair: &SamePair<OuterBitmapIndex>,
        outer_pos_pair: &SamePair<OuterPos>,
    ) -> Self
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        if outer_bitmap_index.holds_garbage(outer_bitmap_index_pair) {
            Self::default()
        } else {
            if let OuterBitmapState::Active(mut active_outer_bitmap) =
                OuterBitmapState::from(key.load())
            {
                active_outer_bitmap.clean_in_range(
                    outer_bitmap_index,
                    outer_bitmap_index_pair,
                    outer_pos_pair,
                );

                active_outer_bitmap
            } else {
                ActiveOuterBitmap::default()
            }
        }
    }
}
