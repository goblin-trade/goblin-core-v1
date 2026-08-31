use goblin_macros::ConstDefault;

use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks, TryIntoUnsidedDelta,
        UnsidedDeltaLots,
    },
};

#[derive(ConstDefault, Clone, Copy)]
pub struct LocalMake {
    pub inner: SamePair<UnsidedDeltaLots>,
}

impl LocalMake {
    /// Add base lots to local make. Depending on `UM` lots are added or subtracted.
    ///
    /// Base lots are converted and stored lots for the respective side.
    ///
    /// # Convention
    ///
    /// In: LegMarker represents the direction from perspective of taker.
    /// Therefore when make<In = Base>(), we deposit `Quote`.
    ///
    pub fn add_make<UM, OP>(
        &mut self,
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError>
    where
        UM: UpdateMarker,
        OP: LegMatcher,
    {
        let matching_lots = OP::matching_lots_maker(base_lots, tick_size, price);
        let lots = OP::lots_taker(matching_lots, base_lot_size);

        let delta_lots = lots.try_into_unsided_delta()?;
        let store = OP::get_leg_mut(&mut self.inner);

        *store = UM::checked_update(*store, delta_lots).ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
