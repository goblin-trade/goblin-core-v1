use crate::{
    axis::market::market_spec::MarketSpec,
    quantities::{OuterPos, INNER_POS, POS_1},
    state::{
        bitmap::{alias::OuterBitmap, preimage::BitmapPreimage, Bitmap},
        SlotKey,
    },
};

impl Bitmap<POS_1, INNER_POS> {
    pub fn conditional_write<MS: MarketSpec>(
        &self,
        clone: &Self,
        key: &SlotKey<BitmapPreimage<MS, POS_1, INNER_POS>>,
        outer_pos: OuterPos,
        outer_bitmap_state: &mut OuterBitmap,
    ) {
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
