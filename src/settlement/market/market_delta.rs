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
    pub sender_delta: SenderDelta,
    pub maker_deltas: MarketMakerDeltas,
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
        // let gg = if decode_deposit_amounts {
        //     P::decode(payload, offset, len)?
        // } else {
        //     P::ResolvedPair::<DeltaAtoms>::default()
        // };

        let deposit_pair: P::ResolvedPair<DeltaAtoms> = P::decode(payload, offset, len)?;

        Ok(MarketDelta {
            sender_delta: SenderDelta::default(),
            maker_deltas: MarketMakerDeltas::default(),
            deposit_pair,
        })
    }
}
