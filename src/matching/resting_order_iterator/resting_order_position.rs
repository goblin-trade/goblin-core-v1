use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::{OuterBitmapIndex, OuterPos},
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
        Ticks::from_matrix(
            self.outer_bitmap_index,
            self.outer_pos,
            self.preimage.inner_pos,
        )
        // Ticks::from_inner_bitmap(self.inner_bitmap_index, self.preimage.inner_pos.row())
    }
}
