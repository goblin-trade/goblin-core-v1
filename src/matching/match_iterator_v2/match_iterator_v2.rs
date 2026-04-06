use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::Position,
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        MarketPreimage, SlotKey,
    },
};

pub struct RestingOrderEntryV2<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub position: Position,
    pub resting_order_key: SlotKey<RestingOrderPreimage<M, B, Q>>,
    pub resting_order: RestingOrder,
}

pub fn match_iterator_v2<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    start: Position,
    end: Position,
) -> impl Iterator<Item = RestingOrderEntryV2<M, B, Q>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
}
