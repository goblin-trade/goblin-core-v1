use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        update::{Decrease, Increase, UpdateMarker},
    },
    goblin_error::GoblinError,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks, TryIntoUnsidedDelta},
    settlement::{
        local_delta::{local_take::TakeCounterparties, DeltaLotsPair},
        CheckedOps,
    },
    types::Address,
};

pub struct LocalTake {
    pub sender: DeltaLotsPair,
    pub counterparties: TakeCounterparties,
}

impl LocalTake {
    /// Add matched lots to sender and counterparty deltas
    pub fn add_take<In: LegMatcher>(
        &mut self,
        counterparty: &Address,
        take_in: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError> {
        let counterparty_delta_pair = self
            .counterparties
            .get_or_insert_mut(*counterparty)
            .ok_or(GoblinError::LocalMakerListFull)?;

        Self::add_for_leg::<In, Decrease>(
            &mut self.sender,
            take_in,
            base_lot_size,
            counterparty_delta_pair,
        )?;

        let take_out = In::matching_lots_out(take_in, tick_size, price);
        Self::add_for_leg::<In::Opposite, Increase>(
            &mut self.sender,
            take_out,
            base_lot_size,
            counterparty_delta_pair,
        )?;

        Ok(())
    }

    fn add_for_leg<In: LegMatcher, U: UpdateMarker>(
        sender: &mut DeltaLotsPair,
        matching_lots: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        counterparty_delta_pair: &mut DeltaLotsPair,
    ) -> Result<(), GoblinError> {
        // U: UpdateMarker will add positive or negative sign
        //
        // For In + decrease: negative
        // For Opposite + increase: positive
        //
        // It makes sense to net values for sender, since we deal with take.
        //
        // However counterparties have make values updated.
        //
        // 1. In + decrease: counterparty gains this token (add to free)
        // 2. Opposite + decrease: counterparty loses token (subtract from locked)
        //
        // Suppose counterparty placed orders on both sides
        // 1. In: Base. Add to base free, subtract from quote locked.
        // 2. In: Quote. Add to quote free, subtract from base locked.
        //
        // Therefore the two can't be netted. Use unsided units for counterparty.
        let delta_lots =
            In::decode_matching_lots(matching_lots, base_lot_size).try_into_unsided_delta::<U>()?;

        let sender_store = In::get_leg_mut(sender);
        *sender_store = sender_store
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        let counterparty_store = In::get_leg_mut(counterparty_delta_pair);
        *counterparty_store = counterparty_store
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
