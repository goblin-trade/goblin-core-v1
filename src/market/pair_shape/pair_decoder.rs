/// Decode token indices and deposit amounts for various pair shapes
use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    market::PairShape,
    quantities::DeltaAtoms,
    require,
    token::{DynamicIndex, ERC20, ETH},
    types::Tuple,
};

// impl Decodable<<(ETH, ERC20) as PairShape>::ResolvedPair<DynamicIndex>> for (ETH, ERC20) {
//     fn decode(
//         args: &ArgsBuffer,
//         offset: &mut usize,
//         len: usize,
//     ) -> Result<<(ETH, ERC20) as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
//         let byte_quote = args.decode::<u8>(offset, len)?;
//         DynamicIndex::new(byte_quote)
//     }
// }

// impl Decodable<<(ERC20, ETH) as PairShape>::ResolvedPair<DynamicIndex>> for (ERC20, ETH) {
//     fn decode(
//         args: &ArgsBuffer,
//         offset: &mut usize,
//         len: usize,
//     ) -> Result<<(ERC20, ETH) as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
//         let byte_quote = args.decode::<u8>(offset, len)?;
//         DynamicIndex::new(byte_quote)
//     }
// }

// impl Decodable<<(ERC20, ERC20) as PairShape>::ResolvedPair<DynamicIndex>> for (ERC20, ERC20) {
//     fn decode(
//         args: &ArgsBuffer,
//         offset: &mut usize,
//         len: usize,
//     ) -> Result<<(ERC20, ERC20) as PairShape>::ResolvedPair<DynamicIndex>, GoblinError> {
//         let byte_base = args.decode::<u8>(offset, len)?;
//         let byte_quote = args.decode::<u8>(offset, len)?;

//         require!(byte_base != byte_quote, GoblinError::InvalidTokenPair);

//         let base_index = DynamicIndex::new(byte_base)?;
//         let quote_index = DynamicIndex::new(byte_quote)?;

//         Ok(Tuple::new(base_index, quote_index))
//     }
// }

// // Decode deposit amounts

// // Reuse the generic and trait to decode deposit and withdraw amounts too
// impl Decodable<<(ETH, ERC20) as PairShape>::ResolvedPair<DeltaAtoms>> for (ETH, ERC20) {
//     fn decode(
//         args: &ArgsBuffer,
//         offset: &mut usize,
//         len: usize,
//     ) -> Result<<(ETH, ERC20) as PairShape>::ResolvedPair<DeltaAtoms>, GoblinError> {
//         args.decode::<i64>(offset, len).map(DeltaAtoms::new)
//     }
// }

// impl Decodable<<(ERC20, ETH) as PairShape>::ResolvedPair<DeltaAtoms>> for (ERC20, ETH) {
//     fn decode(
//         args: &ArgsBuffer,
//         offset: &mut usize,
//         len: usize,
//     ) -> Result<<(ERC20, ETH) as PairShape>::ResolvedPair<DeltaAtoms>, GoblinError> {
//         args.decode::<i64>(offset, len).map(DeltaAtoms::new)
//     }
// }

// impl Decodable<<(ERC20, ERC20) as PairShape>::ResolvedPair<DeltaAtoms>> for (ERC20, ERC20) {
//     fn decode(
//         args: &ArgsBuffer,
//         offset: &mut usize,
//         len: usize,
//     ) -> Result<<(ERC20, ERC20) as PairShape>::ResolvedPair<DeltaAtoms>, GoblinError> {
//         Ok(Tuple::new(
//             args.decode::<i64>(offset, len).map(DeltaAtoms::new)?,
//             args.decode::<i64>(offset, len).map(DeltaAtoms::new)?,
//         ))
//     }
// }
