use crate::{quantities::Ticks, state::RestingOrder, types::Side};

pub struct QuoteIterator<'a> {
    pub side: Side,
    pub best_price: &'a mut Ticks,
}

pub struct Quote {
    pub price: Ticks,
    pub resting_order: RestingOrder,
}

impl<'a> Iterator for QuoteIterator<'a> {
    type Item = Quote;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
