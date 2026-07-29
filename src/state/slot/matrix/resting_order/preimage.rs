use crate::{
    axis::market::market_spec::MarketSpec,
    quantities::Position,
    state::{resting_order::RestingOrder, MarketPreimage, Preimage, SlotKey},
};

#[derive(Clone, Copy)]
pub struct RestingOrderPreimage<MS: MarketSpec> {
    pub market_key: SlotKey<MarketPreimage<MS>>,
    pub position: Position,
}

impl<MS: MarketSpec> Preimage for RestingOrderPreimage<MS> {
    const SLOT_DISCRIMINATOR: u8 = 6;
    type SlotState = RestingOrder;
}
