use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        party::{Counterparties, Party, Sender},
    },
    goblin_error::GoblinError,
    matching::{FillOutcome, MatchDelta},
    quantities::BaseLotsPerBaseUnit,
    settlement::{
        local_delta::{LocalCounterparties, LocalDeltaStore, LocalSender},
        ConstDefault,
    },
    types::{Address, StoreReader, Tuple},
};

pub type LocalDelta<'a> = Tuple<LocalSender, &'a mut LocalCounterparties, Party>;

impl<'a> From<&'a mut LocalCounterparties> for LocalDelta<'a> {
    fn from(value: &'a mut LocalCounterparties) -> Self {
        Self::new(LocalSender::DEFAULT, value)
    }
}

impl<'a> LocalDelta<'a> {
    pub fn add_take_v2<In: LegMatcher>(
        &mut self,
        base_lot_size: BaseLotsPerBaseUnit,
        fill_outcome: &FillOutcome<In>,
    ) -> Result<(), GoblinError> {
        let match_delta = MatchDelta::<In> {
            matching_lots: fill_outcome.matched,
            base_lot_size,
            price_in_quote_lots: fill_outcome.price_in_quote_lots,
        };
        let lots = match_delta.lots();
        let lots_opposite = match_delta.lots_opposite()?;

        let sender = Sender::get_leg_mut(self);
        sender.take.add::<In>(lots, lots_opposite)?;

        let counterparty = Counterparties::get_leg_mut(self)
            .get_or_insert_mut(*fill_outcome.counterparty)
            .ok_or(GoblinError::LocalCounterpartyFull)?;

        counterparty.add::<In>(lots, lots_opposite)?;

        Ok(())
    }

    /// Add matched lots to sender and counterparty deltas
    pub fn add_take<In: LegMatcher>(
        &mut self,
        counterparty: &Address,
        match_delta: MatchDelta<In>,
    ) -> Result<(), GoblinError> {
        let lots = match_delta.lots();
        let lots_opposite = match_delta.lots_opposite()?;

        let sender = Sender::get_leg_mut(self);
        sender.take.add::<In>(lots, lots_opposite)?;

        let counterparty = Counterparties::get_leg_mut(self)
            .get_or_insert_mut(*counterparty)
            .ok_or(GoblinError::LocalCounterpartyFull)?;

        counterparty.add::<In>(lots, lots_opposite)?;

        Ok(())
    }
}
