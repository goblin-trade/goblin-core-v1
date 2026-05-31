use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    goblin_error::GoblinError,
    quantities::{BaseLots, InnerPos},
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, SlotKey},
    types::Address,
};

pub trait OccupancyMarker {
    fn increase<M, B, Q>(
        msg_sender: &Address,
        base_lots: BaseLots,
        inner_pos: InnerPos,
        inner_bitmap_state: &mut InnerBitmap,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker;
}
