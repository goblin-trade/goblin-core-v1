use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::inner_pos::InnerPos,
    state::{
        bitmap::inner_bitmap::preimage::InnerBitmapPreimage, resting_order::RestingOrder, Preimage,
        SlotKey,
    },
};

#[derive(Clone, Copy)]
pub struct RestingOrderPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub inner_bitmap_key: SlotKey<InnerBitmapPreimage<M, B, Q>>,
    pub inner_pos: InnerPos,
}

impl<M, B, Q> Preimage for RestingOrderPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    const SLOT_DISCRIMINATOR: u8 = 7;
    type SlotState = RestingOrder;
}
