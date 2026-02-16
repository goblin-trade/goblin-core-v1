use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::{price_coordinates::PriceCoordinates, OuterBitmapIndex, OuterPos},
    quantities::Ticks,
    state::resting_order::RestingOrderPreimage,
};

pub struct RestingOrderPosition<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub outer_bitmap_index: OuterBitmapIndex,
    pub outer_pos: OuterPos,

    pub preimage: RestingOrderPreimage<M, B, Q>,
}

impl<M, B, Q> RestingOrderPosition<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
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
