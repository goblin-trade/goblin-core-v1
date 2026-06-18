use crate::{
    axis::{market::market_marker::MarketMarker, token::token_reader::TokenReader},
    quantities::{OuterPos, INNER_POS, POS_1},
    state::{
        bitmap::{alias::OuterBitmap, preimage::BitmapPreimage, Bitmap},
        SlotKey,
    },
};

impl Bitmap<POS_1, INNER_POS> {
    pub fn conditional_write<M, B, Q>(
        &self,
        clone: &Self,
        key: &SlotKey<BitmapPreimage<M, B, Q, POS_1, INNER_POS>>,
        outer_pos: OuterPos,
        outer_bitmap_state: &mut OuterBitmap,
    ) where
        M: MarketMarker,
        B: TokenReader,
        Q: TokenReader,
    {
        if *clone == *self {
            return;
        }

        if self.is_empty() {
            // Inner bitmap deactivated
            outer_bitmap_state.deactivate(outer_pos);
        } else {
            if clone.is_empty() {
                outer_bitmap_state.activate(outer_pos);
            }

            // Store activation and update cases
            key.store(self);
        }
    }
}
