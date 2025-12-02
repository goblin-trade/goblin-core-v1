use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::PairShape,
    quantities::DeltaAtoms,
    settlement::local_delta::{LocalDeposits, LocalMakerDeltas, LocalSenderDelta},
};

// #[derive(Default)]
pub struct LocalDelta {
    /// Delta for msg.sender
    pub local_sender_delta: LocalSenderDelta,

    /// Deltas for makers of matched resting orders
    pub local_maker_deltas: LocalMakerDeltas,

    pub deposits: LocalDeposits, // /// Atoms to deposit for the market's token pair
                                 // pub deposit_pair: P::ResolvedPair<DeltaAtoms>,
}

impl LocalDelta {
    pub const fn new() -> Self {
        Self {
            local_sender_delta: LocalSenderDelta::new(),
            local_maker_deltas: LocalMakerDeltas::new(),
            deposits: LocalDeposits::new(),
        }
    }
}

// impl<P> LocalDelta<P>
// where
//     P: PairShape + Decodable<P::ResolvedPair<DeltaAtoms>>,
//     P::ResolvedPair<DeltaAtoms>: Default,
// {
//     pub const fn new() -> Self {
//         Self {
//             local_sender_delta: LocalSenderDelta::new(),
//             local_maker_deltas: LocalMakerDeltas::new(),
//             // deposit_pair: (),
//         }
//     }

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
