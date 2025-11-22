use core::default;

use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::{MarketVariant, PairShape},
    quantities::DeltaAtoms,
    settlement::market::{MarketMakerDeltas, SenderDelta},
};

// #[derive(Default)]
pub struct MarketDelta<P: PairShape> {
    /// Delta for msg.sender
    pub sender_delta: SenderDelta,

    /// Deltas for makers of matched resting orders
    pub maker_deltas: MarketMakerDeltas,

    /// Atoms to deposit for the market's token pair
    pub deposit_pair: P::ResolvedPair<DeltaAtoms>,
}

impl<P> MarketDelta<P>
where
    P: PairShape + Decodable<P::ResolvedPair<DeltaAtoms>>,
{
    pub fn new(
        decode_deposit_amounts: bool,
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self, GoblinError> {
        let gg = if decode_deposit_amounts {
            P::decode(payload, offset, len)?
        } else {
            // Need to implement default()
            //
            // Should we apply it at top level of trait, or only on ResolvedPair<DeltaAtoms>?
            // If we apply on top level, we need to apply it for TokenIndex and DynamicIndex
            P::ResolvedPair::<DeltaAtoms>::default()
        };

        let deposit_pair: P::ResolvedPair<DeltaAtoms> = P::decode(payload, offset, len)?;

        Ok(MarketDelta {
            sender_delta: SenderDelta::default(),
            maker_deltas: MarketMakerDeltas::default(),
            deposit_pair,
        })
    }
}
