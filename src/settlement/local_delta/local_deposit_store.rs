use crate::{
    markets::PairShape,
    quantities::DeltaAtoms,
    token::{TokenMarker, ERC20, ETH},
    types::{DepositPair, Pair, PairShapeTriple, Triple, TripleReader, Tuple},
};

pub type LocalDepositStore = Pair<DepositPair, DepositPair>;

impl LocalDepositStore {
    pub const fn zero() -> Self {
        Self::new(DepositPair::zero(), DepositPair::zero())
    }
}

// Can I get rid of triple and use a nested tuple instead?
// There are 4 total combinations.
// pub type LocalDepositStore =
//     PairShapeTriple<DepositPair<ETH, ERC20>, DepositPair<ERC20, ETH>, DepositPair<ERC20, ERC20>>;

// /// Deposit amounts for a market. The values are 0 by default, and filled
// /// according to the 3 versions of P: PairShape
// impl LocalDepositStore {
//     pub const fn zero() -> Self {
//         Triple::new(
//             DepositPair::new((), DeltaAtoms::ZERO),
//             DepositPair::new(DeltaAtoms::ZERO, ()),
//             DepositPair::new(DeltaAtoms::ZERO, DeltaAtoms::ZERO),
//         )
//         // Triple::new(
//         //     DeltaAtoms::ZERO,
//         //     DeltaAtoms::ZERO,
//         //     Tuple::new(DeltaAtoms::ZERO, DeltaAtoms::ZERO),
//         // )
//     }

//     // /// Reset deposit amounts for P
//     // ///
//     // /// # Optimization
//     // ///
//     // /// No need to zero fill the entire struct. Only reset what was used.
//     // pub fn reset<P>(&mut self)
//     // where
//     //     P: PairShape
//     //         + TripleReader<
//     //             DeltaAtoms,
//     //             DeltaAtoms,
//     //             Pair<DeltaAtoms, DeltaAtoms>,
//     //             ((ETH, ERC20), (ERC20, ETH), (ERC20, ERC20)),
//     //             Result = P::ResolvedPair<DeltaAtoms>,
//     //         >,
//     //     P::ResolvedPair<DeltaAtoms>: Default,
//     // {
//     //     *P::get_leg_mut(self) = P::ResolvedPair::<DeltaAtoms>::default();
//     // }
// }
