use crate::{
    quantities::{InnerBitmapIndex, InnerIndex, Ticks},
    state::{Preimage, RestingOrderPreimage, SlotKey},
};

pub struct RestingOrderPosition {
    pub inner_bitmap_index: InnerBitmapIndex,
    pub inner_index: InnerIndex,
    pub inner_bitmap_key: [u8; 32], // pub inner_bitmap_key: SlotKey<InnerBitmap>,
}

impl RestingOrderPosition {
    pub fn price(&self) -> Ticks {
        Ticks::from_inner_bitmap_index_row(self.inner_bitmap_index, self.inner_index.row())
    }

    pub fn hash(&self) -> SlotKey<RestingOrderPreimage> {
        let preimage = RestingOrderPreimage {
            inner_bitmap_key: self.inner_bitmap_key,
            inner_index: self.inner_index,
        };

        preimage.hash()
    }
}
