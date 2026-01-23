use crate::{
    market::PairShape,
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

    // /// Reset the local delta so it can be reused
    // pub fn reset<P>(&mut self)
    // where
    //     P: PairShape
    //         + TripleReader<
    //             DeltaAtoms,
    //             DeltaAtoms,
    //             Pair<DeltaAtoms, DeltaAtoms>,
    //             ((ETH, ERC20), (ERC20, ETH), (ERC20, ERC20)),
    //             Result = P::ResolvedPair<DeltaAtoms>,
    //         >,
    //     P::ResolvedPair<DeltaAtoms>: Default,
    //     // LocalDepositStore: LocalDeposits<P>,
    // {
    //     self.local_sender_delta = LocalSenderDelta::zero();
    //     self.local_maker_deltas.reset();

    //     self.deposits.reset::<P>();
    //     // self.deposits.reset();
    // }
}

// impl<P> LocalDelta<P>
// where
//     P: PairShape + Decodable<P::ResolvedPair<DeltaAtoms>>,
//     P::ResolvedPair<DeltaAtoms>: Default,
// {
//     // pub fn set_deposit_amounts(
//     //     &mut self,
//     //     decode_deposit_amounts: bool,
//     //     args: &ArgsBuffer,
//     //     offset: &mut usize,
//     //     len: usize,
//     // ) -> Result<(), GoblinError> {
//     //     if decode_deposit_amounts {
//     //         self.deposit_pair = P::decode(args, offset, len)?;
//     //     }

//     //     Ok(())
//     // }

//     // pub fn new(
//     //     decode_deposit_amounts: bool,
//     //     args: &ArgsBuffer,
//     //     offset: &mut usize,
//     //     len: usize,
//     // ) -> Result<Self, GoblinError> {
//     //     let deposit_pair = if decode_deposit_amounts {
//     //         P::decode(args, offset, len)?
//     //     } else {
//     //         P::ResolvedPair::<DeltaAtoms>::default()
//     //     };

//     //     Ok(LocalDelta {
//     //         local_sender_delta: LocalSenderDelta::new(),
//     //         local_maker_deltas: LocalMakerDeltas::new(),
//     //         deposit_pair,
//     //     })
//     // }
// }
