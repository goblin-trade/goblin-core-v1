use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::PairShape,
    quantities::DeltaAtoms,
    settlement::local_delta::{LocalMakerDeltas, LocalSenderDelta},
};

// #[derive(Default)]
pub struct LocalDelta<P: PairShape> {
    /// Delta for msg.sender
    pub local_sender_delta: LocalSenderDelta,

    /// Deltas for makers of matched resting orders
    pub local_maker_deltas: LocalMakerDeltas,

    /// Atoms to deposit for the market's token pair
    pub deposit_pair: P::ResolvedPair<DeltaAtoms>,
}

impl<P> LocalDelta<P>
where
    P: PairShape + Decodable<P::ResolvedPair<DeltaAtoms>>,
    P::ResolvedPair<DeltaAtoms>: Default,
{
    pub fn new(
        decode_deposit_amounts: bool,
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self, GoblinError> {
        let deposit_pair = if decode_deposit_amounts {
            P::decode(payload, offset, len)?
        } else {
            P::ResolvedPair::<DeltaAtoms>::default()
        };

        Ok(LocalDelta {
            local_sender_delta: LocalSenderDelta::default(),
            local_maker_deltas: LocalMakerDeltas::default(),
            deposit_pair,
        })
    }
}
