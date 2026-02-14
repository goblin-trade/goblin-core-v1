use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::InnerBitmapIndex,
    quantities::Ticks,
    state::resting_order::RestingOrderPreimage,
};

pub struct RestingOrderPosition<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub inner_bitmap_index: InnerBitmapIndex,

    pub preimage: RestingOrderPreimage<M, B, Q>,
}

impl<M, B, Q> RestingOrderPosition<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn price(&self) -> Ticks {
        Ticks::from_inner_bitmap(self.inner_bitmap_index, self.preimage.inner_pos.row())
    }
}
