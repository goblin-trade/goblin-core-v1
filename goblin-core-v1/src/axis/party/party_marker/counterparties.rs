use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
        party::{Counterparties, PartyMarker},
        update::{Decrease, Increase},
    },
    goblin_error::GoblinError,
    settlement::local_delta::LocalDelta,
    types::{Address, StoreReader},
};

impl PartyMarker for Counterparties {
    fn add_local_delta<In: LegMatcher>(
        lots: In::Lots,
        lots_opposite: <In::Opposite as LegQuantities>::Lots,
        counterparty: &Address,
        local_delta: &mut LocalDelta,
    ) -> Result<(), GoblinError> {
        let counterparty_pair = Counterparties::get_leg_mut(local_delta)
            .get_or_insert_mut(*counterparty)
            .ok_or(GoblinError::LocalCounterpartyFull)?;

        counterparty_pair.add_leg::<Decrease, In>(lots)?;
        counterparty_pair.add_leg::<Increase, In::Opposite>(lots_opposite)?;

        Ok(())
    }
}
