use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::resting_order_iterator::{
        resting_order_position::RestingOrderPosition, RestingOrderIterator,
    },
};

impl<'a, M, B, Q, In> Iterator for RestingOrderIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    type Item = RestingOrderPosition<M, B, Q>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
