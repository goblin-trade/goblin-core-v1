use core::marker::PhantomData;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::outer_bitmap::outer_bitmap_item::OuterBitmapItem,
        bitmap::outer_bitmap_index::OuterBitmapIndex,
    },
    state::{
        outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
        MarketPreimage, Preimage, SlotKey,
    },
};

/// Proxy struct for market key with helper function to get OuterBitmapItem
pub struct MarketItem<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    /// Market key
    pub market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,

    _marker: PhantomData<In>,
}

impl<'a, M, B, Q, In> MarketItem<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn new(market_key: &'a SlotKey<MarketPreimage<M, B, Q>>) -> Self {
        Self {
            market_key,
            _marker: PhantomData,
        }
    }

    pub fn next_item(
        &self,
        outer_bitmap_index: OuterBitmapIndex<In>,
    ) -> Option<OuterBitmapItem<M, B, Q, In>> {
        let preimage = OuterBitmapPreimage {
            market_key: *self.market_key,
            outer_bitmap_index,
        };
        let outer_bitmap_key = preimage.hash();
        let outer_bitmap = outer_bitmap_key.load();
        let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

        if let OuterBitmapState::Active(active_outer_bitmap) = outer_bitmap_state {
            return Some(OuterBitmapItem {
                outer_bitmap_index,
                outer_bitmap_key,
                active_outer_bitmap,
            });
        }

        None
    }
}
