use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::OUTER_POS,
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        SlotKey,
    },
};

impl Bitmap<OUTER_POS> {
    pub fn conditional_write<M, B, Q>(
        &mut self,
        clone: &Self,
        key: &SlotKey<BitmapPreimage<M, B, Q, OUTER_POS>>,
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
