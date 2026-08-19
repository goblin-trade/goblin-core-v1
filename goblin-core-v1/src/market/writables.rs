use crate::{
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::local_delta::{local_take::TakeCounterparties, LocalDelta, LocalDeposits},
    state::{MarketPreimage, MarketState, SlotKey},
};

pub struct Writables<'a> {
    pub local_delta: LocalDelta<'a>,
    pub market_state: MarketState,
}

impl<'a> Writables<'a> {
    pub fn try_new<MS: MarketSpec>(
        read_deposits: bool,
        ctx: &DecodeCtx,
        market_key: &SlotKey<MarketPreimage<MS>>,
        take_counterparties: &'a mut TakeCounterparties,
    ) -> Result<Self, GoblinError> {
        let deposits = LocalDeposits::try_new::<MS::Pair>(read_deposits, ctx)?;
        let local_delta = LocalDelta::new(deposits, take_counterparties);

        let market_state = market_key.load();

        Ok(Self {
            local_delta,
            market_state,
        })
    }
}
