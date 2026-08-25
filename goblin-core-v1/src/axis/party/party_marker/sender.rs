use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
        party::{PartyMarker, Sender},
        update::{Decrease, Increase},
    },
    goblin_error::GoblinError,
    settlement::local_delta::LocalDelta,
    types::{Address, StoreReader},
};

impl PartyMarker for Sender {
    fn add_local_delta<In: LegMatcher>(
        lots: In::Lots,
        lots_opposite: <In::Opposite as LegQuantities>::Lots,
        _counterparty: &Address,
        local_delta: &mut LocalDelta,
    ) -> Result<(), GoblinError> {
        let local_take = &mut Sender::get_leg_mut(local_delta).take;

        local_take.add_leg::<Decrease, In>(lots)?;
        local_take.add_leg::<Increase, In::Opposite>(lots_opposite)?;

        Ok(())
    }
}
