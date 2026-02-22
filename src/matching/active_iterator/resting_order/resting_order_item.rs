use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{
        inner_coordinates::InnerCoordinates, outer_bitmap_index::OuterBitmapIndex,
        outer_pos::OuterPos,
    },
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
};

pub struct RestingOrderItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_pos: OuterPos<In>,
    pub inner_coordinates: InnerCoordinates<In>,
    pub resting_order_key: SlotKey<RestingOrderPreimage<M, B, Q, In>>,
    pub resting_order: RestingOrder<M, B, Q>,
    pub limit_reached: bool,
}
