use crate::{
    axis_helpers::TokenPair,
    quantities::{OUTER_POS, POS_0},
    state::{
        bitmap::{preimage::BitmapPreimage, Bitmap},
        SlotKey,
    },
};

impl Bitmap<POS_0, OUTER_POS> {
    pub fn conditional_write<TP: TokenPair>(
        &mut self,
        clone: &Self,
        key: &SlotKey<BitmapPreimage<TP, POS_0, OUTER_POS>>,
    ) {
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
