use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        update::{Decrease, Increase, UpdateMarker},
    },
    goblin_error::GoblinError,
    quantities::{
        BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks, TryIntoUnsidedDelta,
        UnsideQuantity,
    },
    settlement::{
        local_delta::{
            local_take::{LocalCounterparty, TakeCounterparties},
            DeltaLotsPair,
        },
        CheckedOps, ConstZero,
    },
    types::Address,
};

pub struct LocalTake<'a> {
    pub sender: DeltaLotsPair,
    pub counterparties: &'a mut TakeCounterparties,
}

impl<'a> LocalTake<'a> {
    pub const fn new(counterparties: &'a mut TakeCounterparties) -> Self {
        Self {
            sender: DeltaLotsPair::ZEROED,
            counterparties,
        }
    }

    /// Add matched lots to sender and counterparty deltas
    pub fn add_take<In: LegMatcher>(
        &mut self,
        counterparty: &Address,
        matched: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError> {
        let counterparty_pair = self
            .counterparties
            .get_or_insert_mut(*counterparty)
            .ok_or(GoblinError::LocalCounterpartyFull)?;

        Self::add_for_leg::<In, Decrease>(
            &mut self.sender,
            matched,
            base_lot_size,
            counterparty_pair,
        )?;

        let take_out = In::matching_lots_out(matched, tick_size, price);
        Self::add_for_leg::<In::Opposite, Increase>(
            &mut self.sender,
            take_out,
            base_lot_size,
            counterparty_pair,
        )?;

        Ok(())
    }

    fn add_for_leg<In: LegMatcher, U: UpdateMarker>(
        sender: &mut DeltaLotsPair,
        matching_lots: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        counterparty_pair: &mut SamePair<LocalCounterparty>,
    ) -> Result<(), GoblinError> {
        let lots = In::decode_matching_lots(matching_lots, base_lot_size);

        let sender_store = In::get_leg_mut(sender);
        let delta_lots = lots.try_into_unsided_delta::<U>()?;
        *sender_store = sender_store
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        let counterparty_store = U::get_leg_mut(In::get_leg_mut(counterparty_pair));
        *counterparty_store = counterparty_store
            .checked_add(lots.unsided())
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
