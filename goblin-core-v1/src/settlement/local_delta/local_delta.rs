use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath, Base, Pair, SamePair},
        party::{Counterparties, Party, PartyMarker, Sender},
        update::{Decrease, Increase, UpdateMarker},
    },
    goblin_error::GoblinError,
    quantities::{
        BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks, TryIntoUnsidedDelta,
        UnsideQuantity, UnsidedDeltaLots,
    },
    settlement::{
        local_delta::{LocalCounterparties, LocalCounterparty, LocalSender},
        CheckedOps, ConstDefault,
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
    /// Add matched lots to sender and counterparty deltas
    pub fn add_take<In: LegMatcher>(
        &mut self,
        counterparty: &Address,
        matched: In::MatchingLots,

        // TODO combine (B, T, P) into common struct
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError> {
        // New design
        // 1. Get lots for base and quote size, convert into Pair
        // 2. Determine UM for base. In = decrease, opposite = increase. We can have a trait for this.
        // 3. Convert lots to delta lots based on UM

        // TODO simplified function
        // let base_lots = In::base_lots_from_matching(matched, tick_size, price);
        // let quote_lots =
        //     Base::opposite_lots_consumed_on_make(base_lots, base_lot_size, tick_size, price);

        // let lot_pair = Pair::new(base_lots, quote_lots);

        // Sender::add_local_delta::<In>(lot_pair, counterparty, self)?;

        let lots = In::decode_matching_lots(matched, base_lot_size);
        let matched_opposite = In::matching_lots_out(matched, tick_size, price);
        let lots_opposite =
            <In::Opposite as LegMath>::decode_matching_lots(matched_opposite, base_lot_size);

        // problem- we wanted to have a single function
        // passing counterparty_pair externally is akward
        //
        // Unified function or macro
        // For a given In- get (Decrease, In) and (Increase, Opposite)
        Sender::add_local_delta::<Decrease, In>(lots, counterparty, self)?;
        Sender::add_local_delta::<Increase, In::Opposite>(lots_opposite, counterparty, self)?;

        let counterparty_pair = Counterparties::get_leg_mut(self)
            .get_or_insert_mut(*counterparty)
            .ok_or(GoblinError::LocalCounterpartyFull)?;

        // // let counterparty_pair = self
        // //     .1
        // //     .get_or_insert_mut(*counterparty)
        // //     .ok_or(GoblinError::LocalCounterpartyFull)?;

        // // let sender_take = &mut self.0.take;

        // // Why does using inner fields with 0 and 1 work but using trait create
        // // problem?
        // let counterparty_pair = Counterparties::get_leg_mut(self)
        //     .get_or_insert_mut(*counterparty)
        //     .ok_or(GoblinError::LocalCounterpartyFull)?;

        // let sender_take = &mut Sender::get_leg_mut(self).take;

        // Self::add_for_leg::<In, Decrease>(sender_take, matched, base_lot_size, counterparty_pair)?;

        // let take_out = In::matching_lots_out(matched, tick_size, price);
        // Self::add_for_leg::<In::Opposite, Increase>(
        //     &mut self.sender,
        //     take_out,
        //     base_lot_size,
        //     counterparty_pair,
        // )?;

        Ok(())
    }

    fn add_for_leg_v2<In: LegMatcher, U: UpdateMarker>(
        sender: &mut SamePair<UnsidedDeltaLots>,
        counterparty_pair: &mut SamePair<LocalCounterparty>,
        lots: In::Lots,
    ) -> Result<(), GoblinError> {
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

    fn add_for_leg<In: LegMatcher, U: UpdateMarker>(
        sender: &mut SamePair<UnsidedDeltaLots>,
        counterparty_pair: &mut SamePair<LocalCounterparty>,
        matching_lots: In::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
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
