use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    quantities::{QuoteLotsPerBaseUnitPerTick, Ticks},
    settlement::{
        local_delta::{Deposits, LocalMakerDeltas},
        sender_delta::{SidedSenderDeltaPairV2, SidedTakeDeltaV2},
        CheckedAdd, ConstZero,
    },
    types::Address,
};

pub struct LocalDelta {
    /// Deposits of the market's token pair
    pub deposits: Deposits,

    /// Delta for msg.sender
    pub local_sender_delta: SidedSenderDeltaPairV2,

    /// Deltas for makers of matched resting orders
    pub local_maker_deltas: LocalMakerDeltas,
}

impl ConstZero for LocalDelta {
    const ZEROED: Self = Self {
        deposits: Deposits::ZEROED,
        local_sender_delta: SidedSenderDeltaPairV2::ZEROED,
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
        let delta = In::get_leg_mut(&mut self.local_sender_delta);
        let take_out = In::matching_lots_out(take_in, tick_size, price);
        let matched = SidedTakeDeltaV2::<In> { take_in, take_out };

        delta.take = matched;

        let maker_delta_pair = self
            .local_maker_deltas
            .get_or_insert_mut(maker)
            .ok_or(GoblinError::MakerListFull)?;

        let maker_delta = In::get_leg_mut(maker_delta_pair);
        // maker_delta
        //     .checked_add(matched)
        //     .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
