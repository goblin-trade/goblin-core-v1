use crate::{
    matching::bitmap::{InnerBitmapIndex, InnerPos},
    quantities::Ticks,
    state::{Preimage, RestingOrderPreimage, SlotKey},
};

pub struct RestingOrderPosition {
    pub inner_bitmap_index: InnerBitmapIndex,
    pub inner_pos: InnerPos,
    pub inner_bitmap_key: [u8; 32], // pub inner_bitmap_key: SlotKey<InnerBitmap>,
}

impl RestingOrderPosition {
    pub fn price(&self) -> Ticks {
        Ticks::from_inner_bitmap(self.inner_bitmap_index, self.inner_pos.row())
    }

    pub fn hash(&self) -> SlotKey<RestingOrderPreimage> {
        let preimage = RestingOrderPreimage {
            inner_bitmap_key: self.inner_bitmap_key,
            inner_pos: self.inner_pos,
        };

        preimage.hash()
    }
}
