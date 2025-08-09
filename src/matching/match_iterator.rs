use crate::{quantities::Ticks, state::RestingOrder, types::Side};

pub struct MatchIterator<'a> {
    pub side: Side,
    pub best_price: &'a mut Ticks,
}

impl<'a> Iterator for MatchIterator<'a> {
    type Item = RestingOrder;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
