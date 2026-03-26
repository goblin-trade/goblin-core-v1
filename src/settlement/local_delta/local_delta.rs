use crate::{
    axis::leg::{leg_matcher::LegMatcher, Pair},
    goblin_error::GoblinError,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, DeltaAtoms, QuantityOps, QuoteLots,
        QuoteLotsPerBaseUnitPerTick, Ticks,
    },
    require,
    settlement::{
        local_delta::{Deposits, LocalMakerDeltas, LocalSenderDelta},
        MatchedLots,
    },
    types::Address,
};

pub struct LocalDelta {
    /// Delta for msg.sender
    pub local_sender_delta: LocalSenderDelta,

    /// Deltas for makers of matched resting orders
    pub local_maker_deltas: LocalMakerDeltas,

    /// Deposits for opening and increasing resting orders
    pub resting_order_deposits: Pair<QuoteLots, BaseLots>,

    pub deposits: Deposits,
}

impl LocalDelta {
    pub const fn zero() -> Self {
        Self {
            local_sender_delta: LocalSenderDelta::zero(),
            local_maker_deltas: LocalMakerDeltas::zero(),
            resting_order_deposits: Pair::new(QuoteLots::ZERO, BaseLots::ZERO),
            deposits: Pair::new(DeltaAtoms::ZERO, DeltaAtoms::ZERO),
        }
    }

    pub fn add_resting_order_deposit<In>(
        &mut self,
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
    {
        let delta = In::maker_deposit(base_lots, base_lot_size, tick_size, price);

        // let store = In::get_leg_mut(&mut self.resting_order_deposits);
        Ok(())
    }

    /// Add matched lots to taker and maker deltas
    pub fn add_matched<In>(
        &mut self,
        maker: Address,
        taker_in: In::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
    {
        let taker_out = In::matching_lots_out(taker_in, tick_size, price);
        let matched_lots = MatchedLots {
            taker_in,
            taker_out,
        };

        let taker_delta = In::get_leg_mut(&mut self.local_sender_delta.taker_delta_pair);
        taker_delta
            .checked_add(matched_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        let maker_delta_pair = self
            .local_maker_deltas
            .get_or_insert_mut(maker)
            .ok_or(GoblinError::MakerListFull)?;

        let maker_delta = In::get_leg_mut(maker_delta_pair);
        maker_delta
            .checked_add(matched_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

    /// Validate whether minimum lots are matched
    pub fn verify_min_match<In>(
        &self,
        min_lots: In::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
    {
        let taker_delta = In::get_leg(&self.local_sender_delta.taker_delta_pair);
        let min_lots = In::matching_lots_in(min_lots, base_lot_size);
        require!(
            taker_delta.taker_in >= min_lots,
            GoblinError::InsufficientTakerFill
        );
        Ok(())
    }
}
