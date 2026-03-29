use crate::{
    axis::leg::{leg_matcher::LegMatcher, Base, Pair, Quote},
    goblin_error::GoblinError,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, QuantityOps, QuoteLots, QuoteLotsPerBaseUnitPerTick, Ticks,
    },
    settlement::{MatchedLots, MatchedLotsPair},
};

/// The sender delta of local namespace
///
/// Values are denominated in MatchingLots. Convert it to TakerTokenUpdate
/// so it can be added to the global delta
pub struct LocalSenderDelta {
    /// The results of matching take orders
    pub taker_delta_pair: MatchedLotsPair,

    /// Deposits for opening and increasing resting orders
    pub resting_order_deposits: Pair<QuoteLots, BaseLots>,
}

impl LocalSenderDelta {
    pub const fn zero() -> Self {
        Self {
            taker_delta_pair: Pair::new(MatchedLots::<Base>::zero(), MatchedLots::<Quote>::zero()),
            resting_order_deposits: Pair::new(QuoteLots::ZERO, BaseLots::ZERO),
        }
    }

    pub fn add_resting_order_deposit<In: LegMatcher>(
        &mut self,
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError> {
        let delta = In::maker_deposit(base_lots, base_lot_size, tick_size, price);
        let deposit = In::get_leg_mut(&mut self.resting_order_deposits);
        *deposit = deposit.checked_add(delta).ok_or(GoblinError::Overflow)?;
        Ok(())
    }

    pub fn subtract_resting_order_deposit<In: LegMatcher>(
        &mut self,
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError> {
        let delta = In::maker_deposit(base_lots, base_lot_size, tick_size, price);
        let deposit = In::get_leg_mut(&mut self.resting_order_deposits);
        *deposit = deposit.checked_sub(delta).ok_or(GoblinError::Overflow)?;
        Ok(())
    }
}
