use crate::{
    axis_helpers::TokenPair,
    quantities::{INNER_POS, OuterPos, POS_1},
    state::{
        SlotKey,
        bitmap::{Bitmap, alias::OuterBitmap, bitmap_preimage::BitmapPreimage},
    },
};

impl Bitmap<POS_1, INNER_POS> {
    pub fn conditional_write<TP: TokenPair>(
        &self,
        clone: &Self,
        key: &SlotKey<BitmapPreimage<TP, POS_1, INNER_POS>>,
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
