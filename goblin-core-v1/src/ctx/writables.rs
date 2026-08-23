use crate::{
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    settlement::local_delta::{LocalCounterparties, LocalDelta},
    state::{MarketPreimage, MarketState, SlotKey},
};

pub struct Writables<'a> {
    pub local_delta: LocalDelta<'a>,
    pub market_state: MarketState,
}

impl<'a> Writables<'a> {
    pub fn try_new<MS: MarketSpec>(
        market_key: &SlotKey<MarketPreimage<MS>>,
        local_counterparties: &'a mut LocalCounterparties,
    ) -> Result<Self, GoblinError> {
        let local_delta = LocalDelta::from(local_counterparties);

        let market_state = market_key.load();

        Ok(Self {
            local_delta,
            market_state,
        })
    }
}
