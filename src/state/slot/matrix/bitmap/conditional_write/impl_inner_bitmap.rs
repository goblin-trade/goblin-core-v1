use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::{OuterPos, INNER_POS, OUTER_POS, POS_0, POS_1},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        SlotKey,
    },
};

impl Bitmap<POS_1, INNER_POS> {
    pub fn conditional_write<M, B, Q>(
        &self,
        clone: &Self,
        key: &SlotKey<BitmapPreimage<M, B, Q, POS_1, INNER_POS>>,
        outer_bitmap_state: &mut Bitmap<POS_0, OUTER_POS>,
        outer_pos: OuterPos,
    ) where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
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
