use crate::{
    axis::{market::market_marker::MarketMarker, token::token_reader::TokenReader},
    quantities::Position,
    state::{resting_order::RestingOrder, MarketPreimage, Preimage, SlotKey},
};

#[derive(Clone, Copy)]
pub struct RestingOrderPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub position: Position,
}

impl<M, B, Q> Preimage for RestingOrderPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    const SLOT_DISCRIMINATOR: u8 = 6;
    type SlotState = RestingOrder;
}
