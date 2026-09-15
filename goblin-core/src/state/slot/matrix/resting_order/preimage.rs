use crate::{
    axis_helpers::TokenPair,
    quantities::PositionV2,
    state::{MarketPreimage, Preimage, SlotKey, resting_order::RestingOrder},
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RestingOrderPreimage<TP: TokenPair> {
    pub market_key: SlotKey<MarketPreimage<TP>>,
    pub position: PositionV2,
}

impl<TP: TokenPair> Preimage for RestingOrderPreimage<TP> {
    const SLOT_DISCRIMINATOR: u8 = 6;
    type SlotState = RestingOrder;
}
