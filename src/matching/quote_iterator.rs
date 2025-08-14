use crate::{
    quantities::Ticks,
    state::RestingOrder,
    types::{Side, SideMarker},
};

pub struct QuoteIterator<'a, S: SideMarker> {
    pub best_price: &'a mut Ticks,
    _marker: core::marker::PhantomData<S>,
}

impl<'a, S: SideMarker> QuoteIterator<'a, S> {
    /// Create a new QuoteIterator starting at the given best price
    pub fn new(best_price: &'a mut Ticks) -> Self {
        Self {
            best_price,
            _marker: core::marker::PhantomData,
        }
    }
}

pub struct Quote {
    pub price: Ticks,
    pub resting_order: RestingOrder,
}

impl<'a, S: SideMarker> Iterator for QuoteIterator<'a, S> {
    type Item = Quote;

    fn next(&mut self) -> Option<Self::Item> {
        let index = S::INDEX;
        None
    }
}
