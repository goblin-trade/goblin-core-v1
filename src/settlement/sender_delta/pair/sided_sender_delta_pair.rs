use crate::{
    axis::leg::{leg_matcher::LegMatcher, Base, Pair, Quote},
    goblin_error::GoblinError,
    quantities::BaseLotsPerBaseUnit,
    require,
    settlement::{sender_delta::SidedSenderDeltaV2, ConstZero},
};

pub type SidedSenderDeltaPairV2 = Pair<SidedSenderDeltaV2<Base>, SidedSenderDeltaV2<Quote>>;

impl ConstZero for SidedSenderDeltaPairV2 {
    const ZEROED: Self = Pair::new(
        SidedSenderDeltaV2::<Base>::ZEROED,
        SidedSenderDeltaV2::<Quote>::ZEROED,
    );
}

impl SidedSenderDeltaPairV2 {
    pub fn verify_min_match<In>(
        &self,
        min_lots: In::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
    {
        let delta = In::get_leg(self);
        let min_lots = In::matching_lots_in(min_lots, base_lot_size);
        require!(
            delta.take.take_in >= min_lots,
            GoblinError::InsufficientTakerFill
        );
        Ok(())
    }
}
