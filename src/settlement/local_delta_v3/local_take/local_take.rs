use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    quantities::{
        BaseLotsPerBaseUnit, DeltaLots, QuoteLotsPerBaseUnitPerTick, Ticks, UnsideQuantity,
    },
    settlement::{
        local_delta_v3::{local_take::TakeCounterparties, DeltaLotsPair},
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

        // problem- direction
        // subtract taker_in and add taker_out
        Self::add_for_leg::<In>(&mut self.sender, take_in, base_lot_size, maker_delta_pair)?;

        let take_out = In::matching_lots_out(take_in, tick_size, price);
        Self::add_for_leg::<In::Opposite>(
            &mut self.sender,
            take_out,
            base_lot_size,
            maker_delta_pair,
        )?;

        Ok(())
    }

    fn add_for_leg<In: LegMatcher>(
        sender: &mut DeltaLotsPair,
        matching_lots: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        maker_delta_pair: &mut DeltaLotsPair,
    ) -> Result<(), GoblinError> {
        let unsided_lots = In::decode_matching_lots(matching_lots, base_lot_size).unsided();

        // increase and decrease follows Maker convention
        // increase = increase resting order, i.e. decrease from store

        let delta_lots =
            DeltaLots::try_from(In::decode_matching_lots(matching_lots, base_lot_size).unsided())?;

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
