use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        update::{update_marker::UpdateMarker, Decrease, Increase},
    },
    goblin_error::GoblinError,
    quantities::{BaseLotsPerBaseUnit, QuantityToUnsidedDelta, QuoteLotsPerBaseUnitPerTick, Ticks
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
    /// Add matched lots to taker and maker deltas
    pub fn add_take<In: LegMatcher>(
        &mut self,
        maker: &Address,
        take_in: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError> {
        let maker_delta_pair = self
            .counterparties
            .get_or_insert_mut(*maker)
            .ok_or(GoblinError::LocalMakerListFull)?;

        Self::add_for_leg::<In, Decrease>(
            &mut self.sender,
            take_in,
            base_lot_size,
            maker_delta_pair,
        )?;

        let take_out = In::matching_lots_out(take_in, tick_size, price);
        Self::add_for_leg::<In::Opposite, Increase>(
            &mut self.sender,
            take_out,
            base_lot_size,
            maker_delta_pair,
        )?;

        Ok(())
    }

    fn add_for_leg<In: LegMatcher, U: UpdateMarker>(
        sender: &mut DeltaLotsPair,
        matching_lots: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        maker_delta_pair: &mut DeltaLotsPair,
    ) -> Result<(), GoblinError> {
        let delta_lots =
            In::decode_matching_lots(matching_lots, base_lot_size).try_into_unsided_delta::<U>()?;

        let sender_store = In::get_leg_mut(sender);
        *sender_store = sender_store
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        let counterparty_store = In::get_leg_mut(maker_delta_pair);
        *counterparty_store = counterparty_store
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
