use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::{OuterPosV2, INNER_POS_V2, OUTER_POS_V2},
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        SlotKey,
    },
};

impl BitmapV2<INNER_POS_V2> {
    pub fn conditional_write<M, B, Q>(
        &self,
        clone: &Self,
        key: &SlotKey<BitmapPreimageV2<M, B, Q, INNER_POS_V2>>,
        outer_bitmap_state: &mut BitmapV2<OUTER_POS_V2>,
        outer_pos: OuterPosV2,
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
