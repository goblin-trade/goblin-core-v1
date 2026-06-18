use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    settlement::{
        global_delta::{GlobalMakerDeltas, GlobalSenderDelta},
        ConstZero,
    },
};

/// The top level delta. Tracks pending token balance updates.
pub struct GlobalDelta {
    /// Delta for msg.sender
    pub global_sender_delta: GlobalSenderDelta,

    /// Deltas for makers of matched orders
    pub maker_deltas: GlobalMakerDeltas,
}

impl ConstZero for GlobalDelta {
    const ZEROED: Self = Self {
        global_sender_delta: GlobalSenderDelta::ZEROED,
        maker_deltas: GlobalMakerDeltas::ZEROED,
    };
}

impl GlobalDelta {
    pub fn settle(&self) -> Result<(), GoblinError> {
        Ok(())
    }
}
