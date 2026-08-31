use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath},
        party::{Counterparties, Party, Sender},
    },
    goblin_error::GoblinError,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks},
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
    /// Add matched lots to sender and counterparty deltas
    pub fn add_take<In: LegMatcher>(
        &mut self,
        counterparty: &Address,
        matching_lots: In::MatchingLots,

        // TODO combine (B, T, P) into common struct
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError> {
        let lots = In::lots_taker(matching_lots, base_lot_size);

        let base_lots = In::base_lots_maker(matching_lots, tick_size, price);
        let matching_lots_opposite = In::Opposite::matching_lots_maker(base_lots, tick_size, price);
        let lots_opposite = In::Opposite::lots_taker(matching_lots_opposite, base_lot_size);

        let sender = Sender::get_leg_mut(self);
        sender.take.add::<In>(lots, lots_opposite)?;

        let counterparty = Counterparties::get_leg_mut(self)
            .get_or_insert_mut(*counterparty)
            .ok_or(GoblinError::LocalCounterpartyFull)?;

        counterparty.add::<In>(lots, lots_opposite)?;

        Ok(())
    }
}
