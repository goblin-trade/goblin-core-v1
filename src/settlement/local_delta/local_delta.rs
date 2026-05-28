use crate::{
    axis::leg::{leg_matcher::LegMatcher, Base, Leg, Pair, Quote},
    goblin_error::GoblinError,
    quantities::{
        BaseLotsPerBaseUnit, DeltaAtoms, QuantityOps, QuoteLotsPerBaseUnitPerTick, Ticks,
    },
    require,
    settlement::{
        local_delta::{Deposits, LocalMakerDeltas, LocalSenderDelta},
        sender_delta::alias::{SidedSenderDeltaV2, SidedTakeDeltaV2},
        ConstZero, MatchedLots,
    },
    types::{Address, StoreReader, Tuple},
};

pub struct LocalDelta {
    /// Deposits of the market's token pair
    pub deposits: Deposits,

    /// Delta for msg.sender
    pub local_sender_delta: LocalSenderDelta,

    /// Deltas for makers of matched resting orders
    pub local_maker_deltas: LocalMakerDeltas,
}

impl ConstZero for LocalDelta {
    const ZEROED: Self = Self {
        deposits: Deposits::ZEROED,
        local_sender_delta: LocalSenderDelta::ZEROED,
        local_maker_deltas: LocalMakerDeltas::ZEROED,
    };
}

impl LocalDelta {
    /// Add matched lots to taker and maker deltas
    pub fn add_matched<In>(
        &mut self,
        maker: Address,
        take_in: In::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
    {
        let take_out = In::matching_lots_out(take_in, tick_size, price);

        let leg_delta = In::get_leg_mut(&mut self.local_sender_delta);
        leg_delta.take = SidedTakeDeltaV2::<In> { take_in, take_out };

        // let taker_delta = In::get_leg_mut(&mut self.local_sender_delta.taker_delta_pair);
        // taker_delta
        //     .checked_add(matched_lots)
        //     .ok_or(GoblinError::DeltaOverflow)?;

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
