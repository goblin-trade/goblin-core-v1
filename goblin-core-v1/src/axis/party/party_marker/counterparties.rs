use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
        party::{Counterparties, PartyMarker},
        update::{Decrease, Increase},
    },
    goblin_error::GoblinError,
    quantities::UnsideQuantity,
    settlement::{local_delta::LocalDelta, CheckedOps},
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

        let counterparty_in = Decrease::get_leg_mut(In::get_leg_mut(counterparty_pair));
        *counterparty_in = counterparty_in
            .checked_add(lots.unsided())
            .ok_or(GoblinError::DeltaOverflow)?;

        let counterparty_out = Increase::get_leg_mut(In::Opposite::get_leg_mut(counterparty_pair));
        *counterparty_out = counterparty_out
            .checked_add(lots_opposite.unsided())
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
