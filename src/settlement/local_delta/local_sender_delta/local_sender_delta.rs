use crate::{
    axis::leg::{leg_matcher::LegMatcher, Base, Pair, Quote},
    goblin_error::GoblinError,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, QuantityOps, QuoteLots, QuoteLotsPerBaseUnitPerTick, Ticks,
    },
    settlement::{sender_delta::alias::SidedSenderDeltaV2, MatchedLots, MatchedLotsPair},
};

pub type LocalSenderDelta = Pair<SidedSenderDeltaV2<Base>, SidedSenderDeltaV2<Quote>>;

// /// The sender delta of local namespace
// ///
// /// Values are denominated in MatchingLots. Convert it to TakerTokenUpdate
// /// so it can be added to the global delta
// ///
// /// TODO update. This has 3 pair types inside. Replace with a common struct and a single Pair
// pub struct LocalSenderDelta {
//     /// The results of matching take orders
//     pub taker_delta_pair: MatchedLotsPair,

//     /// Deposits for opening and increasing resting orders
//     pub make_locked: Pair<QuoteLots, BaseLots>,

//     /// Amount unlocked when resting order is reduced
//     pub reduce_unlocked: Pair<QuoteLots, BaseLots>,
// }

// impl LocalSenderDelta {
//     pub const fn zero() -> Self {
//         Self {
//             taker_delta_pair: Pair::new(MatchedLots::<Base>::zero(), MatchedLots::<Quote>::zero()),
//             make_locked: Pair::new(QuoteLots::ZERO, BaseLots::ZERO),
//             reduce_unlocked: Pair::new(QuoteLots::ZERO, BaseLots::ZERO),
//         }
//     }

//     pub fn add_resting_order_deposit<In: LegMatcher>(
//         &mut self,
//         base_lots: BaseLots,
//         base_lot_size: BaseLotsPerBaseUnit,
//         tick_size: QuoteLotsPerBaseUnitPerTick,
//         price: Ticks,
//     ) -> Result<(), GoblinError> {
//         let delta = In::maker_deposit(base_lots, base_lot_size, tick_size, price);
//         let deposit = In::get_leg_mut(&mut self.make_locked);
//         *deposit = deposit.checked_add(delta).ok_or(GoblinError::Overflow)?;
//         Ok(())
//     }

//     pub fn subtract_resting_order_deposit<In: LegMatcher>(
//         &mut self,
//         base_lots: BaseLots,
//         base_lot_size: BaseLotsPerBaseUnit,
//         tick_size: QuoteLotsPerBaseUnitPerTick,
//         price: Ticks,
//     ) -> Result<(), GoblinError> {
//         let delta = In::maker_deposit(base_lots, base_lot_size, tick_size, price);
//         let reduction = In::get_leg_mut(&mut self.reduce_unlocked);
//         *reduction = reduction.checked_add(delta).ok_or(GoblinError::Overflow)?;
//         Ok(())
//     }
// }
