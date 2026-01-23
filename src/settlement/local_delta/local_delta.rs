use crate::{
    quantities::DeltaAtoms,
    settlement::local_delta::{Deposits, LocalMakerDeltas, LocalSenderDelta},
    types::Pair,
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
}
