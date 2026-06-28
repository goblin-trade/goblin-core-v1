use crate::{
    axis::{leg::leg_matcher::LegMatcher, update::update_marker::UpdateMarker},
    goblin_error::GoblinError,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks, TryIntoUnsidedDelta,
    },
    settlement::{local_delta::DeltaLotsPair, CheckedOps},
    types::StoreReader,
};

pub struct LocalMake {
    pub inner: DeltaLotsPair,
}

impl LocalMake {
    /// Add make delta
    ///
    /// Delta is generated for the opposite side when making orders
    pub fn add_make<In, U>(
        &mut self,
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
        U: UpdateMarker,
    {
        let lots = In::opposite_lots_consumed_on_make(base_lots, base_lot_size, tick_size, price);
        let delta_lots = lots.try_into_unsided_delta::<U>()?;

        let store = In::Opposite::get_leg_mut(&mut self.inner);
        *store = store
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
