use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::OUTER_POS_V2,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        SlotKey,
    },
};

impl BitmapV2<OUTER_POS_V2> {
    pub fn conditional_write<M, B, Q>(
        &mut self,
        clone: &Self,
        key: &SlotKey<BitmapPreimageV2<M, B, Q, OUTER_POS_V2>>,
    ) where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        if *clone == *self {
            return;
        }

        if self.is_empty() {
            // Outer bitmap deactivated
            self.close_with_sentinel();
        }

        // Store sentinel and update cases
        key.store(self);
    }
}
