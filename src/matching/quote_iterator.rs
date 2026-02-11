use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{InnerBitmapIndex, InnerIndex, Ticks},
};

pub struct RestingOrderPositionIterator<'a, In: LegMatcher> {
    pub best_price: &'a mut Ticks,
    _marker: core::marker::PhantomData<In>,
}

impl<'a, In: LegMatcher> RestingOrderPositionIterator<'a, In> {
    /// Create a new QuoteIterator starting at the given best price
    pub fn new(best_price: &'a mut Ticks) -> Self {
        Self {
            best_price,
            _marker: core::marker::PhantomData,
        }
    }
}

pub struct RestingOrderPosition {
    pub inner_bitmap_index: InnerBitmapIndex,
    pub inner_index: InnerIndex,
    pub inner_bitmap_key: [u8; 32], // pub inner_bitmap_key: SlotKey<InnerBitmap>,
}

impl RestingOrderPosition {
    pub fn price(&self) -> Ticks {
        Ticks::from_inner_bitmap_index_row(self.inner_bitmap_index, self.inner_index.row())
    }
}

impl<'a, In: LegMatcher> Iterator for RestingOrderPositionIterator<'a, In> {
    type Item = RestingOrderPosition;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
