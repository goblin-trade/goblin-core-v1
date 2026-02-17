use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        price_coordinates::PriceCoordinates,
    },
    quantities::Ticks,
    state::resting_order::preimage::RestingOrderPreimage,
};

pub struct RestingOrderPosition<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_pos: OuterPos<In>,

    pub preimage: RestingOrderPreimage<M, B, Q, In>,
}

impl<M, B, Q, In> RestingOrderPosition<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn price(&self) -> Ticks {
        PriceCoordinates {
            outer_bitmap_index: self.outer_bitmap_index,
            outer_pos: self.outer_pos,
            row: self.preimage.inner_pos.into(),
        }
        .into()
    }
}
