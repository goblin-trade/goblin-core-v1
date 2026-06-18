use crate::{
    axis::{market::market_marker::MarketMarker, token::token_reader::TokenReader},
    quantities::{OUTER_POS, POS_0},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        SlotKey,
    },
};

impl Bitmap<POS_0, OUTER_POS> {
    pub fn conditional_write<M, B, Q>(
        &mut self,
        clone: &Self,
        key: &SlotKey<BitmapPreimage<M, B, Q, POS_0, OUTER_POS>>,
    ) where
        M: MarketMarker,
        B: TokenReader,
        Q: TokenReader,
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
