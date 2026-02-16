use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        bitmap::OuterBitmapIndex,
        resting_order_iterator::{
            resting_order_position::RestingOrderPosition, RestingOrderIterator,
        },
    },
    state::{
        outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
        Preimage,
    },
};

impl<'a, M, B, Q, In> Iterator for RestingOrderIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    type Item = RestingOrderPosition<M, B, Q>;

    fn next(&mut self) -> Option<Self::Item> {
        // Rudimentary implementation
        // Loop through outer bitmaps
        //
        // Steps
        // - Exit if market_state.outer_index_count is 0
        // - Derive outer_bitmap_index from best price (coordinates() function)
        // - Read slot
        //
        // Question
        // - Do we need to store outer index count?

        let last_outer_bitmap_index = OuterBitmapIndex::from(*self.last_opposite_price);
        let outer_bitmap_preimage = OuterBitmapPreimage {
            market_key: *self.market_key,
            outer_bitmap_index: last_outer_bitmap_index,
        };
        let outer_bitmap_key = outer_bitmap_preimage.hash();
        let outer_bitmap = outer_bitmap_key.load();

        match OuterBitmapState::from(outer_bitmap) {
            OuterBitmapState::Closed => {
                // TODO read next
            }
            OuterBitmapState::Active(_) => todo!(),
        };

        None
    }
}
