use crate::state::Side;

use super::{BitmapRemover, IndexListRemover};

/// Removes resting orders from slot. The resting order itself is not written, instead
/// we update the bitmaps and index list to mark the order as cleared.
///
/// This involves 3 updates
///
/// 1. Bitmap group- Clear the corresponding bit
/// 2. Index list- Remove outer index if the corresponding bitmap group is cleared
/// 3. Market state- Update the outer index count and best price
///
pub struct RestingOrderRemover {
    /// Index list inserter- to insert outer indices in index lists and for writing them to slot
    pub index_list_remover: IndexListRemover,

    /// Bitmap inserter- to activate bits in bitmap groups and writing them to slot
    pub bitmap_remover: BitmapRemover,
}

impl RestingOrderRemover {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        RestingOrderRemover {
            index_list_remover: IndexListRemover::new(side, outer_index_count),
            bitmap_remover: BitmapRemover::new(),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_remover.side()
    }
}
