use crate::{
    quantities::{InnerBitmapIndex, InnerIndex, Ticks},
    state::InnerBitmapKey,
    types::LegMarker,
};

pub struct RestingOrderPositionIterator<'a, In: LegMarker> {
    pub best_price: &'a mut Ticks,
    _marker: core::marker::PhantomData<In>,
}

impl<'a, In: LegMarker> RestingOrderPositionIterator<'a, In> {
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
    pub inner_bitmap_key: InnerBitmapKey,
}

impl RestingOrderPosition {
    pub fn price(&self) -> Ticks {
        Ticks::from_inner_bitmap_index_row(self.inner_bitmap_index, self.inner_index.row())
    }
}

impl<'a, In: LegMarker> Iterator for RestingOrderPositionIterator<'a, In> {
    type Item = RestingOrderPosition;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
