use crate::{
    axis::leg::{leg_matcher::LegMatcher, Base, Leg, Pair, Quote},
    goblin_error::GoblinError,
    quantities::{BaseLotsPerBaseUnit, DeltaAtoms, QuantityOps},
    require,
    settlement::{
        local_delta::{Deposits, LocalMakerDeltas, LocalSenderDelta, MakerDelta, TakerDelta},
        MatchedLots,
    },
    types::{Address, StoreReader, Tuple},
};

pub struct LocalDelta {
    /// Delta for msg.sender
    pub local_sender_delta: LocalSenderDelta,

    /// Deltas for makers of matched resting orders
    pub local_maker_deltas: LocalMakerDeltas,

    pub deposits: Deposits,
}

impl LocalDelta {
    pub const fn zero() -> Self {
        Self {
            local_sender_delta: LocalSenderDelta::zero(),
            local_maker_deltas: LocalMakerDeltas::zero(),
            deposits: Pair::new(DeltaAtoms::ZERO, DeltaAtoms::ZERO),
        }
    }

    /// Add self trade amount to taker delta
    pub fn add_self_trade<In>(
        &mut self,
        taker_self_trade_unlocked: <In::Opposite as LegMatcher>::MatchingLots,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher
            + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>,
    {
        let taker_delta = In::get_leg_mut(&mut self.local_sender_delta.taker_delta_pair);
        taker_delta
            .taker_self_trade_unlocked
            .checked_add(taker_self_trade_unlocked)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

    /// Add matched lots to taker and maker deltas
    pub fn add_matched<In>(
        &mut self,
        maker: Address,
        matched_lots: MatchedLots<In>,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher
            + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>
            + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>,
    {
        let taker_delta = In::get_leg_mut(&mut self.local_sender_delta.taker_delta_pair);
        taker_delta
            .matched_lots
            .checked_add(matched_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        let maker_delta_pair = self
            .local_maker_deltas
            .get_or_insert_mut(maker)
            .ok_or(GoblinError::MakerListFull)?;

        let maker_delta = In::get_leg_mut(maker_delta_pair);
        maker_delta
            .matched_lots
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
        In: LegMatcher
            + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>,
    {
        let taker_delta = In::get_leg(&self.local_sender_delta.taker_delta_pair);
        let min_lots = In::matching_lots_taker(min_lots, base_lot_size);
        require!(
            taker_delta.matched_lots.taker_in >= min_lots,
            GoblinError::InsufficientTakerFill
        );
        Ok(())
    }
}
