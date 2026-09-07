use crate::{
    axis_helpers::TokenPair,
    quantities::Position,
    state::{resting_order::RestingOrder, MarketPreimage, Preimage, SlotKey},
};

#[derive(Clone, Copy)]
pub struct RestingOrderPreimage<TP: TokenPair> {
    pub market_key: SlotKey<MarketPreimage<TP>>,
    pub position: Position,
}

impl<TP: TokenPair> Preimage for RestingOrderPreimage<TP> {
    const SLOT_DISCRIMINATOR: u8 = 6;
    type SlotState = RestingOrder;
}
